#[cfg(target_os = "android")]
use jni::{JavaVM, objects::JObject};

pub mod App;
mod app;
mod client;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) {
    let mut native_options = eframe::NativeOptions::default();
    hide_navigation_bar(&app);
	native_options.persistence_path = Some("/sdcard/".into());
	native_options.android_app = Some(app);

    let run_result = eframe::run_native(
        "Ar6CClient",
        native_options,
        Box::new(|_ctx| Ok(Box::new(app::MyApp::default()))),
    );
    println!("run_native exited: {:?}", run_result);
}

// #[unsafe(no_mangle)]
// fn android_on_create(state: &android_activity::OnCreateState) {
// 	state.;
//     // Initialization code here
// }

#[cfg(target_os = "android")]
pub fn hide_navigation_bar(app: &android_activity::AndroidApp) {
    let vm_ptr = app.vm_as_ptr();
    let activity_ptr = app.activity_as_ptr();

    let vm = unsafe { JavaVM::from_raw(vm_ptr.cast()) }.expect("Failed to get JavaVM");
    let mut env = vm.attach_current_thread().expect("Failed to attach thread");

    // 1. Получаем текущее окно (Window) из Activity
    let activity = unsafe { JObject::from_raw(activity_ptr.cast()) };
	
    // 2. Получаем Window из Activity: activity.getWindow()
    let window = env
        .call_method(&activity, "getWindow", "()Landroid/view/Window;", &[])
        .unwrap()
        .l()
        .unwrap();

    // 3. Отключаем автоматическую подгонку контента под системные бары (Аналог WindowCompat.setDecorFitsSystemWindows(window, false))
    env.call_method(&window, "setDecorFitsSystemWindows", "(Z)V", &[false.into()])
        .ok(); // Метод доступен с API 30, на старых устройствах пропустит

    // 4. Получаем WindowInsetsController: window.getInsetsController()
    if let Ok(controller_val) = env.call_method(&window, "getInsetsController", "()Landroid/view/WindowInsetsController;", &[]) {
        let controller = controller_val.l().unwrap();

        if !controller.is_null() {
            // Константы из Android API
            // WindowInsets.Type.systemBars() = 7 (скрывает и статус-бар, и навигацию)
            // WindowInsets.Type.navigationBars() = 2 (только панель навигации)
            let type_system_bars: i32 = 7; 
            
            // WindowInsetsController.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE = 2
            // Позволяет временно показать панели, если пользователь проведет пальцем от края экрана
            let behavior_transient_swipe: i32 = 2;

            // Задаем поведение при свайпах
            env.call_method(&controller, "setSystemBarsBehavior", "(I)V", &[behavior_transient_swipe.into()])
                .expect("Failed to set system bars behavior");

            // Скрываем панели
            env.call_method(&controller, "hide", "(I)V", &[type_system_bars.into()])
                .expect("Failed to hide system bars");
            
            return;
        }
    }

    // --- НАЗАД В ПРОШЛОЕ (Фоллбэк для устройств с Android 10 и старше) ---
    // Если getInsetsController вернул null или устройство старое, используем классический setSystemUiVisibility
    if let Ok(decor_view_val) = env.call_method(&window, "getDecorView", "()Landroid/view/View;", &[]) {
        let decor_view = decor_view_val.l().unwrap();
        
        // Флаги: SYSTEM_UI_FLAG_HIDE_NAVIGATION (2) | SYSTEM_UI_FLAG_FULLSCREEN (4) | SYSTEM_UI_FLAG_IMMERSIVE_STICKY (4096)
        let old_flags: i32 = 2 | 4096 | 4;
        
        env.call_method(&decor_view, "setSystemUiVisibility", "(I)V", &[old_flags.into()])
            .expect("Failed to set old system UI visibility");
    }
}
