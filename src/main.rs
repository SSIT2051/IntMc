#[cfg(not(target_os = "android"))]
use log::info;

#[cfg(target_os = "android")]
use android_activity::AndroidApp;
#[cfg(target_os = "android")]
use android_logger::Config;
#[cfg(target_os = "android")]
use log::{info, LevelFilter};

mod app;
mod server;
mod playit;
mod lan;
mod plugins;
mod storage;
mod ui;

#[cfg(not(target_os = "android"))]
fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    info!("intmc starting...");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "intmc",
        options,
        Box::new(|cc| Ok(Box::new(app::IntmcApp::new(cc)))),
    )
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Info));
    info!("intmc starting on Android...");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "intmc",
        options,
        Box::new(|cc| Ok(Box::new(app::IntmcApp::new(cc)))),
    ).expect("Failed to run eframe on Android");
}