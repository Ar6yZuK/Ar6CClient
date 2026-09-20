use eframe::NativeOptions;

mod app;
pub mod client;

#[tokio::main]
async fn main() -> eframe::Result {
    let native_options = NativeOptions::default();

    eframe::run_native(
        "Ar6CClient",
        native_options,
        Box::new(|_ctx| Ok(Box::new(app::MyApp::default()))),
    )
}
