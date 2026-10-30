use eframe::egui::{Color32, Ui, Vec2, Align2, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextEdit};
use crate::storage::{PluginInfo, PluginFormat, ServerMetadata};
use crate::app::AppMessage;
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};

pub fn render_resources_tab(ui: &mut Ui, _servers: &[ServerMetadata], _messages: &mut Vec<AppMessage>) {
    ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        ui.strong(RichText::new("⚙").size(24.0));
        ui.add_space(16.0);
        
        ui.horizontal(|ui| {
            ui.label(RichText::new("Memory Usage").size(16.0).color(ON_SURFACE));
            ui.add_space(12.0);
            ui.label(RichText::new("1.2 GB / 2 GB").size(16.0).color(PRIMARY));
        });
        ui.add_space(16.0);
        
        ui.horizontal(|ui| {
            ui.label(RichText::new("CPU Usage").size(16.0).color(ON_SURFACE));
            ui.add_space(12.0);
            ui.label(RichText::new("42%").size(16.0).color(PRIMARY));
        });
        ui.add_space(16.0);
        
        ui.separator();
        ui.add_space(16.0);
        
        ui.horizontal(|ui| {
            ui.label(RichText::new("Disk Usage").size(16.0).color(ON_SURFACE));
            ui.add_space(12.0);
            ui.label(RichText::new("3.4 GB / 50 GB").size(16.0).color(PRIMARY));
        });
        ui.add_space(16.0);
        
        ui.horizontal(|ui| {
            ui.label(RichText::new("Running Servers").size(16.0).color(ON_SURFACE));
            ui.add_space(12.0);
            ui.label(RichText::new("0 / 4").size(16.0).color(ON_SURFACE_VARIANT));
        });
    });
}