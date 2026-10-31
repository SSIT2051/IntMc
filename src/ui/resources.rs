use eframe::egui::{Color32, Ui, Vec2, Align2, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextEdit};
use crate::storage::{PluginInfo, PluginFormat, ServerMetadata};
use crate::app::AppMessage;
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Clone, Default)]
pub struct ResourceMonitor {
    pub last_update: Option<Instant>,
    pub update_interval: std::time::Duration,
}

impl ResourceMonitor {
    pub fn should_update(&self) -> bool {
        self.last_update.map_or(true, |last| last.elapsed() >= self.update_interval)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ResourceMetrics {
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub cpu_percent: f32,
    pub disk_used_gb: f64,
    pub disk_total_gb: f64,
    pub running_servers: usize,
    pub max_servers: usize,
}

pub fn collect_system_metrics(_server_path: &PathBuf) -> ResourceMetrics {
    // Placeholder implementation - would use sysinfo crate in real app
    ResourceMetrics {
        memory_used_mb: 1200,
        memory_total_mb: 2048,
        cpu_percent: 42.0,
        disk_used_gb: 3.4,
        disk_total_gb: 50.0,
        running_servers: 0,
        max_servers: 4,
    }
}

pub fn render_resources_tab(ui: &mut Ui, _monitor: &ResourceMonitor, server_id: &str) {
    let metrics = collect_system_metrics(&PathBuf::from("."));
    
    ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.strong(RichText::new("⚙").size(24.0));
            ui.add_space(8.0);
            ui.label(RichText::new(format!("Resources: {}", server_id)).size(18.0).color(ON_SURFACE));
        });
        ui.add_space(16.0);
        
        ui.horizontal(|ui| {
            ui.label(RichText::new("Memory Usage").size(16.0).color(ON_SURFACE));
            ui.add_space(12.0);
            ui.label(RichText::new(format!("{} MB / {} MB", metrics.memory_used_mb, metrics.memory_total_mb)).size(16.0).color(PRIMARY));
        });
        ui.add_space(16.0);
        
        ui.horizontal(|ui| {
            ui.label(RichText::new("CPU Usage").size(16.0).color(ON_SURFACE));
            ui.add_space(12.0);
            ui.label(RichText::new(format!("{:.1}%", metrics.cpu_percent)).size(16.0).color(PRIMARY));
        });
        ui.add_space(16.0);
        
        ui.separator();
        ui.add_space(16.0);
        
        ui.horizontal(|ui| {
            ui.label(RichText::new("Disk Usage").size(16.0).color(ON_SURFACE));
            ui.add_space(12.0);
            ui.label(RichText::new(format!("{:.1} GB / {:.1} GB", metrics.disk_used_gb, metrics.disk_total_gb)).size(16.0).color(PRIMARY));
        });
        ui.add_space(16.0);
        
        ui.horizontal(|ui| {
            ui.label(RichText::new("Running Servers").size(16.0).color(ON_SURFACE));
            ui.add_space(12.0);
            ui.label(RichText::new(format!("{} / {}", metrics.running_servers, metrics.max_servers)).size(16.0).color(ON_SURFACE_VARIANT));
        });
    });
}