use android_activity::AndroidApp;
use android_logger::Config;
use eframe::egui;
use log::{info, LevelFilter};

mod app;
mod server;
mod playit;
mod lan;
mod plugins;
mod storage;
mod ui;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Info));
    info!("intmc starting on Android...");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::Vec2::new(400.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "intmc",
        options,
        Box::new(|cc| Box::new(app::IntmcApp::new(cc))),
    ).expect("Failed to run eframe on Android");
}