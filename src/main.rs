use eframe::egui;
use log::info;

mod app;
mod server;
mod playit;
mod lan;
mod plugins;
mod storage;
mod ui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    info!("intmc starting...");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(400.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "intmc",
        options,
        Box::new(|cc| Box::new(app::IntmcApp::new(cc))),
    )
}