use eframe::egui::*;
use crate::storage::ServerMetadata;
use crate::app::AppMessage;

pub struct IntmcUI {
    servers: Vec<ServerMetadata>,
    selected_server: Option<String>,
    show_create_dialog: bool,
    new_server_name: String,
    new_server_backend: usize,
    new_server_port: String,
    error_message: Option<String>,
    status_message: Option<String>,
    lan_servers: Vec<crate::lan::LanServer>,
    show_lan_dialog: bool,
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
            lan_servers: Vec::new(),
            show_lan_dialog: false,
        }
    }

    pub fn set_servers(&mut self, servers: Vec<ServerMetadata>) {
        self.servers = servers;
    }

    pub fn set_lan_servers(&mut self, servers: Vec<crate::lan::LanServer>) {
        self.lan_servers = servers;
    }

    pub fn set_error(&mut self, msg: Option<String>) {
        self.error_message = msg;
    }

    pub fn set_status(&mut self, msg: Option<String>) {
        self.status_message = msg;
    }

    /// Returns AppMessages triggered by user interactions
    pub fn render(&mut self, ui: &mut Ui) -> Vec<AppMessage> {
        let mut messages = Vec::new();
        
        // Apply dark theme styling
        let mut style = (*ui.style()).as_ref().clone();
        style.spacing.item_spacing = Vec2::splat(8.0);
        ui.set_style(style);
        
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

        // Action buttons row
        ui.horizontal(|ui| {
            if ui.button("Create Server").clicked() {
                self.show_create_dialog = true;
            }
            if ui.button("Refresh").clicked() {
                messages.push(AppMessage::RefreshServers);
            }
            if ui.button("LAN").clicked() {
                self.show_lan_dialog = true;
                messages.push(AppMessage::DiscoverLan);
            }
        });
        ui.add_space(8.0);

        // Server list
        if self.servers.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("No servers yet. Create one to get started!");
            });
        } else {
            // Clone servers to avoid borrowing issues
            let servers = self.servers.clone();
            ScrollArea::vertical().show(ui, |ui| {
                for server in &servers {
                    if let Some(msg) = self.render_server_card(ui, server) {
                        messages.push(msg);
                    }
                }
            });
        }

        // Create dialog
        if self.show_create_dialog {
            let mut should_close = false;
            Window::new("Create New Server")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ui.ctx(), |ui| {
                    ui.label("Server Name");
                    ui.text_edit_singleline(&mut self.new_server_name);
                    ui.add_space(4.0);

                    ui.label("Backend");
                    ComboBox::from_label("").selected_text(match self.new_server_backend {
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
                            if !self.new_server_name.is_empty() {
                                let backend = match self.new_server_backend {
                                    0 => crate::server::ServerBackendType::Pumpkin,
                                    1 => crate::server::ServerBackendType::Steel,
                                    2 => crate::server::ServerBackendType::Rosewater,
                                    _ => crate::server::ServerBackendType::Glowstone,
                                };
                                let port: u16 = self.new_server_port.parse().unwrap_or(25565);
                                messages.push(AppMessage::CreateServer {
                                    name: self.new_server_name.clone(),
                                    backend,
                                    port,
                                });
                                self.new_server_name.clear();
                                self.new_server_port = "25565".to_string();
                            }
                            should_close = true;
                        }
                        if ui.button("Cancel").clicked() {
                            self.new_server_name.clear();
                            should_close = true;
                        }
                    });
                });
            if should_close {
                self.show_create_dialog = false;
            }
        }

        // LAN dialog
        if self.show_lan_dialog && !self.lan_servers.is_empty() {
            let mut open = true;
            Window::new("LAN Servers")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
                .open(&mut open)
                .show(ui.ctx(), |ui| {
                    ui.label("Servers on local network:");
                    ui.add_space(4.0);
                    for lan_server in &self.lan_servers {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(&lan_server.name);
                                ui.label(format!("{}:{}", lan_server.ip, lan_server.port));
                                ui.label(format!("{} players/{}", lan_server.players_online, lan_server.max_players));
                            });
                            if ui.button("Join").clicked() {
                                messages.push(AppMessage::JoinLanServer {
                                    name: lan_server.name.clone(),
                                    ip: lan_server.ip.clone(),
                                    port: lan_server.port,
                                });
                            }
                        });
                    }
                });
            self.show_lan_dialog = open;
        }

        messages
    }

    fn render_server_card(&mut self, ui: &mut Ui, server: &ServerMetadata) -> Option<AppMessage> {
        let mut clicked_msg: Option<AppMessage> = None;
        
        Frame::default()
            .fill(Color32::from_rgb(30, 30, 35))
            .corner_radius(CornerRadius::same(8))
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
                        if server.is_running {
                            ui.colored_label(Color32::GREEN, "● Running");
                        } else {
                            ui.colored_label(Color32::RED, "● Stopped");
                        }
                    });
                    ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                        if server.is_running {
                            if ui.button("Stop").clicked() {
                                clicked_msg = Some(AppMessage::StopServer { server_id: server.id.clone() });
                            }
                        } else {
                            if ui.button("Start").clicked() {
                                clicked_msg = Some(AppMessage::StartServer { server_id: server.id.clone() });
                            }
                        }
                        if ui.button("Settings").clicked() {
                            clicked_msg = Some(AppMessage::OpenSettings { server_id: server.id.clone() });
                        }
                        if ui.button("Delete").clicked() {
                            clicked_msg = Some(AppMessage::DeleteServer { server_id: server.id.clone() });
                        }
                    });
                });
            });
        ui.add_space(8.0);
        clicked_msg
    }
}