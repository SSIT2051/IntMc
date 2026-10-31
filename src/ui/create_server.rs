use eframe::egui::{Color32, Ui, Vec2, Align2, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextEdit, ComboBox};
use crate::app::AppMessage;
use crate::server::{ServerBackendType};
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING, render_section_header};

#[derive(Debug, Clone)]
pub struct CreateServerData {
    pub step: CreateServerStep,
    pub name: String,
    pub backend: ServerBackendType,
    pub version: String,
    pub port: u16,
    pub memory_mb: usize,
    pub auto_tunnel: bool,
    pub selected_version_index: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateServerStep {
    Basic,
    Version,
    Advanced,
    Confirm,
}

pub fn render_create_server_dialog(ctx: &egui::Context, data: &mut CreateServerData, should_close: &mut bool, messages: &mut Vec<AppMessage>) {
    egui::Window::new("Create Server")
        .collapsible(false)
        .resizable(false)
        .default_size([450.0, 500.0])
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                render_section_header(ui, "🎮", Some("New Server"), None::<(&str, fn())>);

                card_frame().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Server Name").size(16.0).color(ON_SURFACE));
                        ui.add_space(16.0);
                        ui.add(
                            egui::TextEdit::singleline(&mut data.name)
                                .hint_text("my-server")
                                .desired_width(300.0)
                        );
                    });
                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Version").size(16.0).color(ON_SURFACE));
                        ui.add_space(16.0);
                        ui.add(
                            egui::TextEdit::singleline(&mut data.version)
                                .hint_text("1.21.1")
                                .desired_width(150.0)
                        );
                    });
                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Backend").size(16.0).color(ON_SURFACE));
                        ui.add_space(16.0);
                        ComboBox::from_label(format!("{:?}", data.backend))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut data.backend, ServerBackendType::Pumpkin, "Pumpkin");
                                ui.selectable_value(&mut data.backend, ServerBackendType::Steel, "Steel");
                                ui.selectable_value(&mut data.backend, ServerBackendType::Rosewater, "Rosewater");
                                ui.selectable_value(&mut data.backend, ServerBackendType::Glowstone, "Glowstone");
                            });
                    });
                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Port").size(16.0).color(ON_SURFACE));
                        ui.add_space(16.0);
                        ui.add(egui::DragValue::new(&mut data.port).speed(1.0));
                    });
                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Max Players").size(16.0).color(ON_SURFACE));
                        ui.add_space(16.0);
                        ui.add(egui::DragValue::new(&mut 20u32).speed(1.0));
                    });
                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Create Playit.gg Tunnel").size(16.0).color(ON_SURFACE));
                        ui.add_space(16.0);
                        ui.checkbox(&mut data.auto_tunnel, "");
                    });
                    ui.add_space(24.0);

                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if outlined_button(ui, "Cancel").clicked() {
                            *should_close = true;
                        }
                        ui.add_space(8.0);
                        if primary_button(ui, "Create Server").clicked() {
                            messages.push(AppMessage::CreateServer {
                                name: data.name.clone(),
                                backend: data.backend,
                                port: data.port,
                            });
                            *should_close = true;
                        }
                    });
                });
            });
            ui.add_space(32.0);
        });
}