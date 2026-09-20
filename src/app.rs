use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use eframe::egui::CentralPanel;
use egui_file_dialog::FileDialog;
use reqwest::header::CONTENT_DISPOSITION;
use uuid::Uuid;

use super::client::{Client, types::OperationType};

pub struct MyApp {
    client: Arc<Client>,
    file_dialog: egui_file_dialog::FileDialog,
    rt: tokio::runtime::Runtime,
    file_name_to_save: Arc<Mutex<Option<String>>>,
    picking_file: Arc<Mutex<bool>>,
    path_to_saved_file: Arc<Mutex<Option<std::path::PathBuf>>>,
    file_data: Arc<Mutex<Option<Bytes>>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            client: Arc::new(Client::new("http://192.168.1.13:5000")),
            file_dialog: egui_file_dialog::FileDialog::new()
				.on_android(|x| x.initial_directory("/sdcard/Download".into())),
            rt: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            file_name_to_save: Arc::new(Mutex::new(None)),
            picking_file: Arc::new(Mutex::new(false)),
            path_to_saved_file: Arc::new(None.into()),
            file_data: Arc::new(None.into()),
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
		let frame = eframe::egui::Frame::default()
			.inner_margin(eframe::egui::Margin { left: 10, right: 10, top: 25, bottom: 25 }); // margin for better working on android

		CentralPanel::default()
			.frame(frame)
			.show(ui, |ui| self.receive_file_from_client_button(ui));
    }
}

impl MyApp {
    fn update_file_dialog(
        file_dialog: &mut FileDialog,
        path_to_saved_file: &Arc<Mutex<Option<PathBuf>>>,
        ui: &mut eframe::egui::Ui,
    ) -> Option<PathBuf> {
        file_dialog.update(ui.ctx());

        if let Some(path_to_saved_file_from_arc) = file_dialog.take_picked() {
            println!("Take picked: {:?}", path_to_saved_file_from_arc);

            if let Ok(mut path_to_saved_file_mutex) = path_to_saved_file.lock() {
                let to_return = path_to_saved_file_from_arc.clone();
                *path_to_saved_file_mutex = Some(path_to_saved_file_from_arc);
                return Some(to_return);
            };
        }

        return None;
    }

    fn receive_file_from_client_button(&mut self, ui: &mut eframe::egui::Ui) {
        if let Ok(mut file_name_to_save_mutex) = self.file_name_to_save.lock()
            && let Some(file_name_to_save) = &*file_name_to_save_mutex
        {
            self.file_dialog.config_mut().default_file_name = file_name_to_save.clone();
            *file_name_to_save_mutex = None; // clear file_name_to_save. Its already set to file_dialog above
        }

        if ui.button("Receive file from server").clicked() {
            let client = Arc::clone(&self.client);
            let file_name_to_save = Arc::clone(&self.file_name_to_save);
            let picking_file = Arc::clone(&self.picking_file);
            let file_data = Arc::clone(&self.file_data);

            let f = async move || -> anyhow::Result<()> {
                const RECEIVE_FILE_TO_CLIENT_OPERATION: &OperationType = &OperationType(6); // See Ar6C\Ar6CLibrary\V1\OperationType.cs enum
                let request_file_response = client
                    .invoke_operation(RECEIVE_FILE_TO_CLIENT_OPERATION, None)
                    .await?;

                let response_text = request_file_response.text().await?;
                println!("{}", response_text);
                let text = response_text
                    .trim_start_matches("data: \"")
                    .trim_end()
                    .trim_end_matches("\""); // trim `data: \"\"\n`. To get guid
                println!("trimmed: {}", text);
                let uuid = Uuid::parse_str(text)?;

                let get_file_response = client.get_server_file(&uuid).await?;

                let file_description = get_file_response.headers().get(CONTENT_DISPOSITION);
                if let Some(file_description) = file_description
                    && let Ok(file_description) = file_description.to_str()
                {
                    let file_description =
                        content_disposition::parse_content_disposition(file_description);
                    if let Some(filename) = file_description.filename_full() {
                        if let Ok(mut file_name_to_save) = file_name_to_save.lock() {
                            println!("File name from headers: {}", filename);
                            *file_name_to_save = Some(filename);
                        }
                    };
                }

                let response_file_data = get_file_response.bytes().await;
                if let Ok(response_file_data) = response_file_data {
                    if let Ok(mut file_data) = file_data.lock() {
                        *file_data = Some(response_file_data);
                    }
                }

                if let Ok(mut picking_file) = picking_file.lock() {
                    *picking_file = true;
					// ui.request_repaint(); // TODO: repaint when picked file. for update file_dialog
                }

                Ok(())
            };

            self.rt.spawn(async move {
                let r = f().await;
                println!(
                    "{:?}",
                    match r {
                        Ok(ok) => {
                            format!("Ok {:?}", ok)
                        }
                        Err(err) => {
                            err.downcast_ref::<String>()
                                .cloned()
                                .or(Some(String::from("Cant downcast error")))
                                .unwrap()
                        }
                    }
                )
            });
        };

        // TODO: catch lock error
        if let Ok(mut picking_file) = self.picking_file.lock()
            && *picking_file
        {
            println!("Picking file");
            *picking_file = false;
            self.file_dialog.save_file();
        }

        if let Some(_path_to_saving_file) = MyApp::update_file_dialog(&mut self.file_dialog, &self.path_to_saved_file, ui)
        {
            let path_to_saved_file = Arc::clone(&self.path_to_saved_file);
            let file_data = Arc::clone(&self.file_data);
            self.rt.spawn(async move {
                if let Ok(path_to_saved_file) = path_to_saved_file.lock()
                    && let Some(path_to_saved_file) = &*path_to_saved_file
                {
                    match File::create_new(path_to_saved_file) {
                        Ok(mut file) => {
                            if let Ok(mut file_data_mutex) = file_data.lock()
                                && let Some(file_data) = &*file_data_mutex
                            {
                                match file.write(file_data) {
                                    Ok(_written) => println!("File written"),
                                    Err(err) => println!("Error when write to file: {}", err),
                                };
                                *file_data_mutex = None;
                            } else {
                                println!("Have no filedata to write or lock error");
                            }
                        }
                        Err(err) => println!("Error with create file: {}", err),
                    }
                }
            });
        }
    }
}

trait FileDialogMyExt {
	fn on_android<T>(self, execute : T)  -> Self where T: Fn(FileDialog) -> FileDialog;
}
impl FileDialogMyExt for FileDialog {
	#[inline]
	fn on_android<T>(self, #[allow(unused_variables)] execute : T) -> Self where T: Fn(FileDialog) -> FileDialog {
		#[cfg(target_os = "android")]
		return execute(self);
		#[cfg(not(target_os = "android"))]
		return self;
	}
}