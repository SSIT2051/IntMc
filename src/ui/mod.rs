pub mod theme;
pub mod servers;
pub mod marketplace;
pub mod lan;
pub mod settings;
pub mod file_browser;
pub mod logs;
pub mod resources;
pub mod create_server;
pub mod components;

pub use theme::*;
pub use servers::*;
pub use marketplace::*;
pub use lan::*;
pub use settings::*;
pub use file_browser::*;
pub use logs::*;
pub use resources::*;
pub use create_server::*;
pub use components::*;

use eframe::egui::{Ui, FontId, FontFamily, Vec2, Align2, Id, Stroke, CornerRadius, Margin, Layout, ScrollArea, RichText, TextStyle, TextEdit, DragValue};
use crate::storage::{Storage, ServerMetadata, PluginFormat};
use crate::app::AppMessage;
use crate::lan::LanServer;
use crate::plugins::PluginEntry;
use crate::playit::PlayitManager;
use crate::server::ServerBackendType;
use crate::ui::theme::{setup_material3_theme, ERROR_CONTAINER, ERROR, SUCCESS, ON_SURFACE, ON_SURFACE_VARIANT, PRIMARY, PRIMARY_CONTAINER};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Servers,
    Marketplace,
    LAN,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerDetailTab {
    Overview,
    Console,
    Files,
    Plugins,
    Resources,
    Settings,
}

#[derive(Debug, Clone)]
pub struct ServerDetailState {
    pub server_id: String,
    pub active_tab: ServerDetailTab,
    pub console_logs: Vec<String>,
    pub follow_console: bool,
    pub selected_file: Option<PathBuf>,
    pub file_browser_path: PathBuf,
    pub show_delete_confirm: bool,
}

#[derive(Debug, Clone)]
pub struct CreateServerState {
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

#[derive(Debug, Clone)]
pub struct MarketplaceState {
    pub search_query: String,
    pub selected_format: Option<PluginFormat>,
    pub selected_category: Option<String>,
    pub plugins: Vec<PluginEntry>,
    pub filtered_plugins: Vec<PluginEntry>,
    pub loading: bool,
    pub error: Option<String>,
    pub installing: HashMap<String, bool>,
    pub server_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LanState {
    pub servers: Vec<LanServer>,
    pub discovering: bool,
    pub error: Option<String>,
    pub announce_enabled: bool,
    pub announce_name: String,
    pub announce_port: u16,
}

#[derive(Debug, Clone)]
pub struct SettingsState {
    pub playit_email: String,
    pub playit_token: String,
    pub playit_logged_in: bool,
    pub theme_dark: bool,
    pub notifications_enabled: bool,
    pub auto_refresh: bool,
    pub refresh_interval_secs: u64,
    pub storage_path: String,
    pub max_log_lines: usize,
    pub show_advanced: bool,
    pub error: Option<String>,
    pub status: Option<String>,
}

pub struct IntmcUI {
    pub active_tab: Tab,
    pub servers: Vec<ServerMetadata>,
    pub selected_server: Option<String>,
    pub server_detail: Option<ServerDetailState>,
    pub create_server: Option<CreateServerState>,
    pub marketplace: MarketplaceState,
    pub lan: LanState,
    pub settings: SettingsState,
    pub error_message: Option<String>,
    pub status_message: Option<String>,
    pub show_global_error: bool,
    pub show_global_status: bool,
}

impl IntmcUI {
    pub fn new() -> Self {
        Self {
            active_tab: Tab::Servers,
            servers: Vec::new(),
            selected_server: None,
            server_detail: None,
            create_server: None,
            marketplace: MarketplaceState {
                search_query: String::new(),
                selected_format: None,
                selected_category: None,
                plugins: Vec::new(),
                filtered_plugins: Vec::new(),
                loading: false,
                error: None,
                installing: HashMap::new(),
                server_id: None,
            },
            lan: LanState {
                servers: Vec::new(),
                discovering: false,
                error: None,
                announce_enabled: false,
                announce_name: "intmc Server".to_string(),
                announce_port: 25565,
            },
            settings: SettingsState {
                playit_email: String::new(),
                playit_token: String::new(),
                playit_logged_in: false,
                theme_dark: true,
                notifications_enabled: true,
                auto_refresh: true,
                refresh_interval_secs: 30,
                storage_path: String::new(),
                max_log_lines: 1000,
                show_advanced: false,
                error: None,
                status: None,
            },
            error_message: None,
            status_message: None,
            show_global_error: false,
            show_global_status: false,
        }
    }

    pub fn set_servers(&mut self, servers: Vec<ServerMetadata>) {
        self.servers = servers;
    }

    pub fn set_lan_servers(&mut self, servers: Vec<LanServer>) {
        self.lan.servers = servers;
        self.lan.discovering = false;
    }

    pub fn set_error(&mut self, msg: Option<String>) {
        if let Some(msg) = msg {
            self.error_message = Some(msg);
            self.show_global_error = true;
        }
    }

    pub fn set_status(&mut self, msg: Option<String>) {
        if let Some(msg) = msg {
            self.status_message = Some(msg);
            self.show_global_status = true;
        }
    }

    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.status_message = None;
        self.show_global_error = false;
        self.show_global_status = false;
    }

    pub fn open_server_detail(&mut self, server_id: String, storage: &Arc<Mutex<Storage>>) {
        if let Some(server) = self.servers.iter().find(|s| s.id == server_id) {
            let file_browser_path = server.path.clone();
            self.server_detail = Some(ServerDetailState {
                server_id: server.id.clone(),
                active_tab: ServerDetailTab::Overview,
                console_logs: Vec::new(),
                follow_console: true,
                selected_file: None,
                file_browser_path,
                show_delete_confirm: false,
            });
            self.selected_server = Some(server.id.clone());
        }
    }

    pub fn close_server_detail(&mut self) {
        self.server_detail = None;
        self.selected_server = None;
    }

    pub fn start_create_server(&mut self) {
        self.create_server = Some(CreateServerState {
            step: CreateServerStep::Basic,
            name: String::new(),
            backend: ServerBackendType::Pumpkin,
            version: "1.21".to_string(),
            port: 25565,
            memory_mb: 1024,
            auto_tunnel: true,
            selected_version_index: 0,
            error: None,
        });
    }

    pub fn cancel_create_server(&mut self) {
        self.create_server = None;
    }

    /// Returns AppMessages triggered by user interactions
    pub fn render(&mut self, ui: &mut Ui, storage: &Arc<Mutex<Storage>>, playit: &PlayitManager) -> Vec<AppMessage> {
        let mut messages = Vec::new();

        // Apply theme
        setup_material3_theme(ui.ctx());

        // Global error/status toasts
        self.render_toasts(ui);

        // Main layout with bottom navigation
        self.render_main_layout(ui, storage, playit, &mut messages);

        messages
    }

    fn render_toasts(&mut self, ui: &mut Ui) {
        if self.show_global_error {
            if let Some(err) = self.error_message.clone() {
                egui::Area::new(Id::unique("global_error"))
                    .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 12.0))
                    .show(ui.ctx(), |ui| {
                        egui::Frame::default()
                            .fill(ERROR_CONTAINER)
                            .stroke(Stroke::new(1.0, ERROR))
                            .corner_radius(CornerRadius::same(12))
                            .inner_margin(Margin::symmetric(16, 12))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.colored_label(ERROR, "⚠");
                                    ui.label(egui::RichText::new(err).color(ON_SURFACE).size(14.0));
                                    if ui.small_button("✕").clicked() {
                                        self.show_global_error = false;
                                        self.error_message = None;
                                    }
                                });
                            });
                    });
            }
        }

        if self.show_global_status {
            if let Some(status) = self.status_message.clone() {
                egui::Area::new(Id::unique("global_status"))
                    .anchor(Align2::CENTER_TOP, Vec2::new(0.0, 12.0))
                    .show(ui.ctx(), |ui| {
                        egui::Frame::default()
                            .fill(SUCCESS.linear_multiply(0.15))
                            .stroke(Stroke::new(1.0, SUCCESS))
                            .corner_radius(CornerRadius::same(12))
                            .inner_margin(Margin::symmetric(16, 12))
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.colored_label(SUCCESS, "✓");
                                    ui.label(egui::RichText::new(status).color(ON_SURFACE).size(14.0));
                                    if ui.small_button("✕").clicked() {
                                        self.show_global_status = false;
                                        self.status_message = None;
                                    }
                                });
                            });
                    });
            }
        }
    }

    fn render_main_layout(&mut self, ui: &mut Ui, storage: &Arc<Mutex<Storage>>, playit: &PlayitManager, messages: &mut Vec<AppMessage>) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical(|ui| {
                // Top app bar
                self.render_top_bar(ui, messages);

                ui.add_space(8.0);

// Content area
                egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                    match self.active_tab {
                        Tab::Servers => crate::ui::servers::render_servers_tab(ui, &self.servers, messages),
                        Tab::Marketplace => crate::ui::marketplace::render_marketplace_tab(ui, &mut self.marketplace, messages),
                        Tab::LAN => crate::ui::lan::render_lan_tab(ui, storage, playit, messages),
                        Tab::Settings => crate::ui::settings::render_settings_tab(ui, storage, playit, messages),
                    }
                });

                ui.add_space(8.0);

                // Bottom navigation bar
                self.render_bottom_nav(ui);
            });
        });

        // Modal dialogs are handled by the app layer
    }

    fn render_top_bar(&mut self, ui: &mut Ui, _messages: &mut Vec<AppMessage>) {
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            ui.heading(egui::RichText::new("intmc").color(PRIMARY).size(28.0));
            ui.label(egui::RichText::new("Minecraft Server Hosting").color(ON_SURFACE_VARIANT).size(14.0));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(16.0);
                // Theme toggle
                if icon_button(ui, if self.settings.theme_dark { "☀" } else { "🌙" }, false).clicked() {
                    self.settings.theme_dark = !self.settings.theme_dark;
                }
                ui.add_space(8.0);
                // Refresh button
                if icon_button(ui, "↻", false).on_hover_text("Refresh").clicked() {
                    // Trigger refresh
                }
            });
        });
        ui.separator();
    }

    fn render_bottom_nav(&mut self, ui: &mut Ui) {
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            let tabs = [
                (Tab::Servers, "🖥", "Servers"),
                (Tab::Marketplace, "📦", "Marketplace"),
                (Tab::LAN, "📡", "LAN"),
                (Tab::Settings, "⚙", "Settings"),
            ];
            
            for (tab, icon, label) in tabs {
                let selected = self.active_tab == tab;
                let response = ui.allocate_response(Vec2::new(ui.available_width() / 4.0, 56.0), egui::Sense::click());
                
                if response.clicked() {
                    self.active_tab = tab;
                }
                
                let rect = response.rect;
                let center = rect.center();
                
                if selected {
                    ui.painter().rect_filled(
                        rect.expand(4.0),
                        CornerRadius::same(16),
                        PRIMARY_CONTAINER,
                    );
                }
                
                let icon_pos = center + Vec2::new(0.0, -10.0);
                let label_pos = center + Vec2::new(0.0, 10.0);
                
                ui.painter().text(
                    icon_pos,
                    Align2::CENTER_CENTER,
                    icon,
                    FontId::new(24.0, FontFamily::Proportional),
                    if selected { PRIMARY } else { ON_SURFACE_VARIANT },
                );
                
                ui.painter().text(
                    label_pos,
                    Align2::CENTER_CENTER,
                    label,
                    FontId::new(11.0, FontFamily::Proportional),
                    if selected { PRIMARY } else { ON_SURFACE_VARIANT },
                );
            }
        });
    }
}

impl Default for IntmcUI {
    fn default() -> Self {
        Self::new()
    }
}