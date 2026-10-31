use log::info;
use slint::ComponentHandle;
use std::sync::Arc;

mod app;
mod server;
mod playit;
mod lan;
mod plugins;
mod storage;
mod ui;

fn main() -> Result<(), slint::PlatformError> {
    env_logger::init();
    info!("intmc starting...");

    // Open storage
    let storage = crate::storage::Storage::open(&crate::app::get_db_path())
        .map_err(|e| slint::PlatformError::Other(format!("storage open failed: {}", e)))?;

    // Create app state
    let app_state = Arc::new(app::AppState::new(storage));

    // Create Slint main window
    let main_window = ui::create_main_window()?;

    // Connect Slint UI callbacks to AppState logic
    ui::connect_messages(&main_window, app_state.clone());

    // Show the window
    main_window.show()?;

    info!("intmc UI initialized and running");

    // Run the Slint event loop
    main_window.run();

    Ok(())
}
