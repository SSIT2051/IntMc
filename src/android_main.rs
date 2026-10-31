use android_activity::AndroidApp;
use android_logger::Config;
use log::{info, error, LevelFilter};
use slint::ComponentHandle;
use std::panic;
use std::sync::Arc;

mod app;
mod server;
mod playit;
mod lan;
mod plugins;
mod storage;
mod ui;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Debug));
    info!("intmc starting on Android...");

    #[cfg(target_os = "android")]
    slint::android::init(app).expect("Failed to initialize Slint Android platform");

    panic::set_hook(Box::new(|panic_info| {
        error!("PANIC: {}", panic_info);
        if let Some(location) = panic_info.location() {
            error!("Panic at {}:{}", location.file(), location.line());
        }
        if let Some(payload) = panic_info.payload().downcast_ref::<&str>() {
            error!("Panic payload: {}", payload);
        } else if let Some(payload) = panic_info.payload().downcast_ref::<String>() {
            error!("Panic payload: {}", payload);
        }
    }));

    // Open storage
    let storage = crate::storage::Storage::open(&crate::app::get_db_path())
        .expect("Failed to open storage");

    // Create app state
    let app_state = Arc::new(app::AppState::new(storage));

    // Create Slint main window
    let main_window = ui::create_main_window().expect("Failed to create main window");

    // Connect Slint UI callbacks to AppState logic
    ui::connect_messages(&main_window, app_state.clone());

    // Show the window
    main_window.show().unwrap();

    info!("intmc UI initialized and showing");

    // Run the Slint event loop
    main_window.run();
}
