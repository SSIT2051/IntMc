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
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "intmc",
        options,
        Box::new(|cc| Ok(Box::new(app::IntmcApp::new(cc)))),
    )
}