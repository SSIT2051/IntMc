use eframe::egui::{Color32, Ui, Vec2, Align2, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextStyle, TextEdit, ComboBox};
use crate::app::AppMessage;
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING, render_section_header};

pub fn render_settings_tab(ui: &mut Ui, _storage: &std::sync::Arc<tokio::sync::Mutex<crate::storage::Storage>>, _playit: &crate::playit::PlayitManager, _messages: &mut Vec<AppMessage>) {
    ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        render_section_header(ui, "🌐", "Playit.gg Tunnel");
        card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Playit.gg Account").size(16.0).color(ON_SURFACE).strong());
                    ui.add_space(4.0);
                    ui.label(RichText::new("Create tunnels to access your server from anywhere").size(13.0).color(ON_SURFACE_VARIANT));
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    if outlined_button(ui, "Sign In").clicked() {
                    }
                });
            });
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);
            
            ui.horizontal(|ui| {
                ui.label(RichText::new("Email").size(14.0).color(ON_SURFACE));
                ui.add_space(12.0);
                ui.add(TextEdit::singleline(&mut String::new()).hint_text("email@example.com").desired_width(250.0));
            });
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Auth Token").size(14.0).color(ON_SURFACE));
                ui.add_space(12.0);
                ui.add(TextEdit::singleline(&mut String::new()).hint_text("Your Playit.gg auth token").password(true).desired_width(250.0));
            });
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    if primary_button(ui, "Save & Connect").clicked() {
                    }
                });
            });
        });
        ui.add_space(16.0);

        render_section_header(ui, "🎨", "Appearance");
        card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Dark Theme").size(16.0).color(ON_SURFACE).strong());
                    ui.add_space(4.0);
                    ui.label(RichText::new("Use dark mode (requires restart)").size(13.0).color(ON_SURFACE_VARIANT));
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    let mut dark = true;
                    ui.add(egui::widgets::Switch::new(&mut dark));
                });
            });
        });
        ui.add_space(16.0);

        render_section_header(ui, "🔔", "Notifications");
        card_frame().show(ui, |ui| {
            let mut notifications = true;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Push Notifications").size(16.0).color(ON_SURFACE).strong());
                    ui.add_space(4.0);
                    ui.label(RichText::new("Receive alerts for server events").size(13.0).color(ON_SURFACE_VARIANT));
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::widgets::Switch::new(&mut notifications));
                });
            });
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);
            
            let mut auto_refresh = true;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Auto Refresh").size(16.0).color(ON_SURFACE).strong());
                    ui.add_space(4.0);
                    ui.label(RichText::new("Automatically refresh server status").size(13.0).color(ON_SURFACE_VARIANT));
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::widgets::Switch::new(&mut auto_refresh));
                });
            });
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new("Refresh Interval").size(14.0).color(ON_SURFACE));
                ui.add_space(12.0);
                ui.add(egui::DragValue::new(&mut 30u64).speed(1.0).suffix(" sec"));
            });
        });
        ui.add_space(16.0);

        render_section_header(ui, "💾", "Storage");
        card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Database Location").size(16.0).color(ON_SURFACE).strong());
                    ui.add_space(4.0);
                    ui.label(RichText::new("/data/data/rust.intmc/intmc.db").size(13.0).color(ON_SURFACE_VARIANT).monospace());
                });
            });
            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);
            
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Server Files").size(16.0).color(ON_SURFACE).strong());
                    ui.add_space(4.0);
                    ui.label(RichText::new("/data/data/rust.intmc/servers/").size(13.0).color(ON_SURFACE_VARIANT).monospace());
                });
            });
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    if outlined_button(ui, "Open Folder").clicked() {
                    }
                    ui.add_space(8.0);
                    if primary_button(ui, "Backup Data").clicked() {
                    }
                });
            });
        });
        ui.add_space(16.0);

        render_section_header(ui, "⚙", "Advanced");
        card_frame().show(ui, |ui| {
            let mut advanced = false;
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Show Advanced Settings").size(16.0).color(ON_SURFACE).strong());
                    ui.add_space(4.0);
                    ui.label(RichText::new("Enable developer options and debug features").size(13.0).color(ON_SURFACE_VARIANT));
                });
                ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(egui::widgets::Switch::new(&mut advanced));
                });
            });
            
            if advanced {
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(12.0);
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Max Log Lines").size(14.0).color(ON_SURFACE));
                    ui.add_space(12.0);
                    ui.add(egui::DragValue::new(&mut 1000usize).speed(10.0).suffix(" lines"));
                });
                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Log Level").size(14.0).color(ON_SURFACE));
                    ui.add_space(12.0);
                    ComboBox::from_label("")
                        .selected_text("Info")
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut "info", "info", "Debug");
                            ui.selectable_value(&mut "info", "info", "Info");
                            ui.selectable_value(&mut "warn", "warn", "Warning");
                            ui.selectable_value(&mut "error", "error", "Error");
                        });
                });
            }
        });
        ui.add_space(16.0);

        render_section_header(ui, "ℹ", "About");
        card_frame().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("intmc").size(24.0).color(PRIMARY).strong());
                ui.add_space(4.0);
                ui.label(RichText::new("Minecraft Server Hosting for Mobile").size(14.0).color(ON_SURFACE_VARIANT));
                ui.add_space(8.0);
                ui.label(RichText::new("Version 0.1.0").size(13.0).color(ON_SURFACE_VARIANT));
                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if text_button(ui, "GitHub").clicked() {
                    }
                    ui.add_space(12.0);
                    if text_button(ui, "Report Issue").clicked() {
                    }
                    ui.add_space(12.0);
                    if text_button(ui, "License").clicked() {
                    }
                });
            });
        });
        ui.add_space(32.0);
    });
}

fn render_section_header(ui: &mut Ui, icon: &str, title: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(icon).size(20.0));
        ui.label(RichText::new(title).size(18.0).strong().color(ON_SURFACE));
    });
    ui.add_space(8.0);
}