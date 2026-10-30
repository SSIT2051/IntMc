use eframe::egui::*;
use crate::storage::ServerMetadata;

pub struct IntmcUI {
    servers: Vec<ServerMetadata>,
    selected_server: Option<String>,
    show_create_dialog: bool,
    new_server_name: String,
    new_server_backend: usize,
    new_server_port: String,
    error_message: Option<String>,
    status_message: Option<String>,
}

impl IntmcUI {
    pub fn new() -> Self {
        Self {
            servers: Vec::new(),
            selected_server: None,
            show_create_dialog: false,
            new_server_name: String::new(),
            new_server_backend: 0,
            new_server_port: "25565".to_string(),
            error_message: None,
            status_message: None,
        }
    }

    pub fn set_servers(&mut self, servers: Vec<ServerMetadata>) {
        self.servers = servers;
    }

    pub fn set_error(&mut self, msg: Option<String>) {
        self.error_message = msg;
    }

    pub fn set_status(&mut self, msg: Option<String>) {
        self.status_message = msg;
    }

    pub fn render(&mut self, ctx: &Context, ui: &mut Ui) {
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = Vec2::splat(8.0);
        ctx.set_style(style);
        
        // Header
        ui.heading("intmc");
        ui.label("Minecraft Server Hosting for Mobile");
        ui.add_space(8.0);

        // Error/Status messages
        if let Some(err) = &self.error_message {
            ui.colored_label(Color32::RED, err);
            ui.add_space(4.0);
        }
        if let Some(status) = &self.status_message {
            ui.colored_label(Color32::GREEN, status);
            ui.add_space(4.0);
        }

        // Action buttons
        ui.horizontal(|ui| {
            if ui.button("Create Server").clicked() {
                self.show_create_dialog = true;
            }
        });
        ui.add_space(8.0);

        // Server list
        if self.servers.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("No servers yet. Create one to get started!");
            });
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for server in &self.servers {
                    self.render_server_card(ui, server);
                }
            });
        }

        // Create dialog
        if self.show_create_dialog {
            let mut show = self.show_create_dialog;
            egui::Window::new("Create New Server")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .open(&mut show)
                .show(ctx, |ui| {
                    ui.label("Server Name");
                    ui.text_edit_singleline(&mut self.new_server_name);
                    ui.add_space(4.0);

                    ui.label("Backend");
                    egui::ComboBox::from_label("").selected_text(match self.new_server_backend {
                        0 => "PumpkinMC",
                        1 => "SteelMC",
                        2 => "Rosewater",
                        3 => "Glowstone-rs",
                        _ => "Unknown",
                    }).show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.new_server_backend, 0, "PumpkinMC");
                        ui.selectable_value(&mut self.new_server_backend, 1, "SteelMC");
                        ui.selectable_value(&mut self.new_server_backend, 2, "Rosewater");
                        ui.selectable_value(&mut self.new_server_backend, 3, "Glowstone-rs");
                    });
                    ui.add_space(4.0);

                    ui.label("Port");
                    ui.text_edit_singleline(&mut self.new_server_port);
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        if ui.button("Create").clicked() {
                            // Handle create
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_create_dialog = false;
                        }
                    });
                });
self.show_create_dialog = show;
        }
    }
    
    fn render_server_card(&self, ui: &mut egui::Ui, server: &ServerMetadata) {
        egui::Frame::default()
            .fill(Color32::from_rgb(30, 30, 35))
            .rounding(8.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading(&server.name);
                        ui.label(format!("Backend: {}", server.backend));
                        ui.label(format!("Version: {}", server.version));
                        ui.label(format!("Port: {}", server.port));
                        if !server.ip.is_empty() {
                            ui.label(format!("IP: {}", server.ip));
                        }
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if server.is_running {
                            if ui.button("Stop").clicked() {
                                // Handle stop
                            }
                        } else {
                            if ui.button("Start").clicked() {
                                // Handle start
                            }
                        }
                        if ui.button("Settings").clicked() {
                            // Handle settings
                        }
                        if ui.button("Delete").clicked() {
                            // Handle delete
                        }
                    });
                });
            });
        ui.add_space(8.0);
    }
}