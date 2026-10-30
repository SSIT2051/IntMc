use eframe::egui::{Color32, Ui, Vec2, Align2, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextEdit, TextStyle};
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};

#[derive(Debug, Clone, Default)]
pub struct LogViewer {
    pub logs: Vec<String>,
    pub max_logs: usize,
    pub auto_scroll: bool,
}

impl LogViewer {
    pub fn add_log(&mut self, log: String) {
        self.logs.push(log);
        if self.logs.len() > self.max_logs {
            self.logs.remove(0);
        }
    }
}

pub fn render_log_viewer(ui: &mut Ui, viewer: &mut LogViewer) {
    card_frame().show(ui, |ui| {
        ScrollArea::vertical().auto_shrink([false, false]).max_height(400.0).show(ui, |ui| {
            for line in &viewer.logs {
                let color = if line.contains("[ERROR]") || line.contains("[FATAL]") {
                    ERROR
                } else if line.contains("[WARN]") || line.contains("[WARNING]") {
                    WARNING
                } else if line.contains("[INFO]") {
                    Color32::from_gray(200)
                } else {
                    ON_SURFACE_VARIANT
                };
                ui.colored_label(color, line);
            }
        });
    });
}

pub fn render_logs_tab(ui: &mut Ui, _logs: &[String], _selected_server: Option<&str>, _messages: &mut Vec<crate::app::AppMessage>) {
    ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.strong(RichText::new("📋").size(20.0));
            ui.label(RichText::new("Server Console").size(18.0).strong().color(ON_SURFACE));
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if outlined_button(ui, "Clear").clicked() {
                }
                ui.add_space(8.0);
                let mut auto_scroll = true;
                ui.checkbox(&mut auto_scroll, "Auto-scroll");
            });
        });
        ui.add_space(8.0);
        
        card_frame().show(ui, |ui| {
            ScrollArea::vertical().auto_shrink([false, false]).max_height(400.0).show(ui, |ui| {
                let lines = vec![
                    "[INFO] Server started on port 25565",
                    "[INFO] World 'world' loaded successfully",
                    "[INFO] Player 'Steve' joined the game",
                    "[INFO] Player 'Steve' left the game",
                    "[WARNING] Entity count high: 150 entities",
                ];
                for line in lines {
                    ui.colored_label(Color32::from_gray(200), line);
                }
            });
        });
        ui.add_space(16.0);
        
        card_frame().show(ui, |ui| {
            ui.label(RichText::new("Send Command").size(16.0).color(ON_SURFACE).strong());
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                let mut command = String::new();
                ui.add(
                    egui::TextEdit::singleline(&mut command)
                        .hint_text("Enter command...")
                        .desired_width(f32::INFINITY)
                );
                ui.add_space(8.0);
                if primary_button(ui, "Send").clicked() {
                }
            });
        });
        ui.add_space(32.0);
    });
}