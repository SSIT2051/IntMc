use android_activity::AndroidApp;
use android_logger::Config;
use log::{info, error, LevelFilter};
use egui_wgpu::{WgpuSetup, WgpuSetupCreateNew, wgpu};
use std::sync::Arc;
use std::panic;

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

    let result = run_eframe(app);
    if let Err(e) = result {
        error!("Failed to run eframe: {}", e);
    }
}

fn run_eframe(app: AndroidApp) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up wgpu with GLES backend preference...");
    
    let wgpu_setup = WgpuSetup::CreateNew(WgpuSetupCreateNew {
        device_descriptor: Arc::new(|adapter| {
            info!("Adapter: {:?}, Backend: {:?}", adapter.get_info().name, adapter.get_info().backend);
            let base_limits = if adapter.get_info().backend == wgpu::Backend::Gl {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            };

            wgpu::DeviceDescriptor {
                label: Some("egui wgpu device"),
                required_limits: wgpu::Limits {
                    max_color_attachments: 4,
                    max_texture_dimension_2d: 8192,
                    ..base_limits
                },
                ..Default::default()
            }
        }),
        ..WgpuSetupCreateNew::without_display_handle()
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 600.0]),
        android_app: Some(app),
        wgpu_options: eframe::WgpuConfiguration {
            wgpu_setup,
            ..Default::default()
        },
        ..Default::default()
    };

    info!("Starting eframe run_native...");
    eframe::run_native(
        "intmc",
        options,
        Box::new(|cc| {
            info!("eframe CreationContext received, egui context available");
            Ok(Box::new(app::IntmcApp::new(cc)))
        }),
    ).map_err(|e| format!("eframe error: {}", e).into())
}