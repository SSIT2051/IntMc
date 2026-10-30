use eframe::egui::{Color32, Ui, Vec2, Align2, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextEdit, ComboBox, DragValue};
use crate::storage::{PluginInfo, PluginFormat};
use crate::app::AppMessage;
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};

pub fn render_file_browser_tab(ui: &mut Ui, _storage: &std::sync::Arc<tokio::sync::Mutex<crate::storage::Storage>>, _messages: &mut Vec<AppMessage>) {
    ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        ui.strong(RichText::new("📁").size(24.0));
        ui.add_space(8.0);
        
        ui.horizontal(|ui| {
            ui.label(RichText::new("Directory").size(16.0).color(ON_SURFACE));
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if outlined_button(ui, "Browse...").clicked() {
                }
            });
        });
        ui.add_space(16.0);
        
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            ui.label(RichText::new("No files found").size(16.0).color(ON_SURFACE_VARIANT));
            ui.add_space(16.0);
        });
        
        ui.add_space(16.0);
        
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            ui.label(RichText::new("Server Files").size(16.0).color(ON_SURFACE));
            ui.add_space(16.0);
        });
        
        ui.add_space(24.0);
        
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            let mut new_server_name = String::new();
            ui.text_edit_singleline(&mut new_server_name).hint_text("New server name...");
            ui.add_space(8.0);
            if primary_button(ui, "Create Server").clicked() {
            }
            ui.add_space(16.0);
        });
        
        ui.add_space(32.0);
        
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Server directory: /data/data/rust.intmc/servers/").size(14.0).color(ON_SURFACE_VARIANT).monospace());
                ui.add_space(8.0);
                ui.label(RichText::new("No servers configured yet. Create a new server to get started.").size(14.0).color(ON_SURFACE_VARIANT));
            });
        });
    });
}