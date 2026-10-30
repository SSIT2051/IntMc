use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Child;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::storage::{Storage, ServerMetadata, PluginInfo, PluginFormat};
use crate::ui::{IntmcUI, Tab, ServerDetailTab, CreateServerState, CreateServerStep, MarketplaceState, LanState, SettingsState, ServerDetailState};
use crate::playit::PlayitManager;
use crate::lan::LanDiscovery;
use crate::server::{ServerConfig, ServerBackendType, get_backend, ServerProcess, ServerBackend};
use crate::plugins::{PluginMarketplace, PluginEntry};
use log::{info, warn, error};
use eframe::egui::{Context, Ui, FontDefinitions, FontFamily, FontData, Vec2, Align2, Color32, Rounding, Stroke, RichText, FontId, ScrollArea, Layout, Sense, TextStyle, DragValue, TextEdit};
use eframe::Frame;
use crate::ui::theme::{setup_material3_theme, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING, card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, status_chip, ServerStatus};
use crate::ui::components::*;
use crate::ui::logs::{LogViewer, render_log_viewer};
use crate::ui::resources::{ResourceMonitor, ResourceMetrics, render_resources_tab, collect_system_metrics};
use crate::ui::file_browser::{render_file_browser, render_file_viewer, FileEntry};
use crate::ui::create_server::{render_create_server_dialog, CreateServerData};

#[derive(Debug, Clone)]
pub enum AppMessage {
    ShowCreateServerDialog,
    CreateServer { name: String, backend: ServerBackendType, port: u16 },
    StartServer { server_id: String },
    StopServer { server_id: String },
    RestartServer { server_id: String },
    DeleteServer { server_id: String },
    RefreshServers,
    RefreshMarketplace,
    DiscoverLan,
    JoinLanServer { name: String, ip: String, port: u16 },
    OpenSettings { server_id: String },
    OpenServerDetail { server_id: String },
    CloseServerDetail,
    InstallPlugin { server_id: String, plugin_name: String, plugin_url: String },
    UninstallPlugin { server_id: String, plugin_name: String },
    TogglePlugin { server_id: String, plugin_name: String },
    CreateTunnel { server_id: String, port: u16 },
    RemoveTunnel { server_id: String },
    AnnounceLan { name: String, port: u16 },
    ViewFile { server_id: String, path: PathBuf },
    SaveFile { server_id: String, path: PathBuf, content: String },
    SendCommand { server_id: String, command: String },
    UpdateServerConfig { server_id: String, config: ServerConfig },
    BackupServer { server_id: String },
    RestoreServer { server_id: String, backup_path: PathBuf },
    ShowError { message: String },
    ShowStatus { message: String },
}

pub struct IntmcApp {
    ui: IntmcUI,
    playit: PlayitManager,
    lan: Option<LanDiscovery>,
    storage: Arc<Mutex<Storage>>,
    running_servers: Arc<Mutex<HashMap<String, ServerProcess>>>,
    plugin_marketplace: PluginMarketplace,
    resource_monitors: Arc<Mutex<HashMap<String, ResourceMonitor>>>,
    log_viewers: Arc<Mutex<HashMap<String, LogViewer>>>,
    error_message: Option<String>,
    status_message: Option<String>,
    lan_servers: Vec<crate::lan::LanServer>,
    show_create_dialog: bool,
    create_server_data: Option<CreateServerData>,
}

impl IntmcApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load embedded font for Android (no system fonts available)
        let font_bytes = include_bytes!("../../assets/DejaVuSans.ttf");
        info!("Embedded font size: {} bytes", font_bytes.len());
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "dejavu_sans".to_owned(),
            Arc::new(FontData::from_static(font_bytes)),
        );
        fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "dejavu_sans".to_owned());
        fonts.families.entry(FontFamily::Monospace).or_default().insert(0, "dejavu_sans".to_owned());
        cc.egui_ctx.set_fonts(fonts);
        
        // Apply Material 3 theme
        setup_material3_theme(&cc.egui_ctx);
        
        let db_path = Self::get_db_path();
        
        // Ensure parent directory exists
        if let Some(parent) = db_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        
        let storage = match Storage::open(&db_path) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to open database: {}. Using in-memory fallback.", e);
                Storage::open(&db_path).unwrap_or_else(|_| {
                    log::error!("Critical: Could not create database at {:?}", db_path);
                    panic!("Failed to initialize database")
                })
            }
        };
        
        let playit = PlayitManager::new();
        let lan = LanDiscovery::new().ok();
        let plugin_marketplace = PluginMarketplace::new(Self::get_cache_dir());
        
        let storage_arc = Arc::new(Mutex::new(storage));
        let running_servers = Arc::new(Mutex::new(HashMap::new()));
        let resource_monitors = Arc::new(Mutex::new(HashMap::new()));
        let log_viewers = Arc::new(Mutex::new(HashMap::new()));
        let ui = IntmcUI::new();
        
        Self {
            ui,
            playit,
            lan,
            storage: storage_arc,
            running_servers,
            plugin_marketplace,
            resource_monitors,
            log_viewers,
            error_message: None,
            status_message: None,
            lan_servers: Vec::new(),
            show_create_dialog: false,
            create_server_data: None,
        }
    }

    fn get_db_path() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            PathBuf::from("/data/data/rust.intmc/intmc.db")
        }
        #[cfg(not(target_os = "android"))]
        {
            PathBuf::from("intmc.db")
        }
    }

    fn get_server_path(server_id: &str) -> PathBuf {
        #[cfg(target_os = "android")]
        {
            PathBuf::from(format!("/data/data/rust.intmc/servers/{}", server_id))
        }
        #[cfg(not(target_os = "android"))]
        {
            PathBuf::from(format!("servers/{}", server_id))
        }
    }

    fn get_cache_dir() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            PathBuf::from("/data/data/rust.intmc/cache")
        }
        #[cfg(not(target_os = "android"))]
        {
            PathBuf::from("cache")
        }
    }

    /// Process UI messages synchronously (called from egui ui callback)
    pub fn process_messages(&mut self, messages: Vec<AppMessage>, ctx: &Context) {
        for msg in messages {
            match msg {
                AppMessage::ShowCreateServerDialog => {
                    self.ui.start_create_server();
                }
                AppMessage::RefreshMarketplace => {
                    self.refresh_marketplace_sync();
                }
                AppMessage::CreateServer { name, backend, port } => {
                    let result = self.create_server_sync(name, backend, port);
                    match &result {
                        Ok(_) => self.status_message = Some("Server created".to_string()),
                        Err(e) => self.error_message = Some(format!("Create failed: {}", e)),
                    }
                }
                AppMessage::StartServer { server_id } => {
                    let result = self.start_server_sync(&server_id);
                    match &result {
                        Ok(_) => self.status_message = Some("Server started".to_string()),
                        Err(e) => self.error_message = Some(format!("Start failed: {}", e)),
                    }
                }
                AppMessage::StopServer { server_id } => {
                    let result = self.stop_server_sync(&server_id);
                    match &result {
                        Ok(_) => self.status_message = Some("Server stopped".to_string()),
                        Err(e) => self.error_message = Some(format!("Stop failed: {}", e)),
                    }
                }
                AppMessage::RestartServer { server_id } => {
                    let _ = self.stop_server_sync(&server_id);
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    let result = self.start_server_sync(&server_id);
                    match &result {
                        Ok(_) => self.status_message = Some("Server restarted".to_string()),
                        Err(e) => self.error_message = Some(format!("Restart failed: {}", e)),
                    }
                }
                AppMessage::DeleteServer { server_id } => {
                    let result = self.delete_server_sync(&server_id);
                    match &result {
                        Ok(_) => self.status_message = Some("Server deleted".to_string()),
                        Err(e) => self.error_message = Some(format!("Delete failed: {}", e)),
                    }
                    self.ui.close_server_detail();
                }
                AppMessage::RefreshServers => {
                    self.refresh_servers_sync();
                }
                AppMessage::DiscoverLan => {
                    self.lan_servers = self.discover_lan_servers_sync();
                    self.ui.set_lan_servers(self.lan_servers.clone());
                }
                AppMessage::JoinLanServer { name, ip, port } => {
                    self.status_message = Some(format!("Joining {} at {}:{}", name, ip, port));
                }
                AppMessage::OpenSettings { server_id } => {
                    self.ui.open_server_detail(server_id, &self.storage);
                }
                AppMessage::OpenServerDetail { server_id } => {
                    self.ui.open_server_detail(server_id, &self.storage);
                }
                AppMessage::CloseServerDetail => {
                    self.ui.close_server_detail();
                }
                AppMessage::InstallPlugin { server_id, plugin_name, plugin_url } => {
                    let result = self.install_plugin_sync(&server_id, &plugin_name, &plugin_url);
                    match &result {
                        Ok(_) => self.status_message = Some(format!("Plugin {} installed", plugin_name)),
                        Err(e) => self.error_message = Some(format!("Install failed: {}", e)),
                    }
                }
                AppMessage::UninstallPlugin { server_id, plugin_name } => {
                    let result = self.uninstall_plugin_sync(&server_id, &plugin_name);
                    match &result {
                        Ok(_) => self.status_message = Some(format!("Plugin {} removed", plugin_name)),
                        Err(e) => self.error_message = Some(format!("Uninstall failed: {}", e)),
                    }
                }
                AppMessage::TogglePlugin { server_id, plugin_name } => {
                    let result = self.toggle_plugin_sync(&server_id, &plugin_name);
                    match &result {
                        Ok(_) => self.status_message = Some(format!("Plugin {} toggled", plugin_name)),
                        Err(e) => self.error_message = Some(format!("Toggle failed: {}", e)),
                    }
                }
                AppMessage::CreateTunnel { server_id, port } => {
                    let result = self.create_tunnel_sync(&server_id, port);
                    match &result {
                        Ok(_) => self.status_message = Some("Tunnel created".to_string()),
                        Err(e) => self.error_message = Some(format!("Tunnel failed: {}", e)),
                    }
                }
                AppMessage::RemoveTunnel { server_id } => {
                    let result = self.remove_tunnel_sync(&server_id);
                    match &result {
                        Ok(_) => self.status_message = Some("Tunnel removed".to_string()),
                        Err(e) => self.error_message = Some(format!("Remove tunnel failed: {}", e)),
                    }
                }
                AppMessage::AnnounceLan { name, port } => {
                    if let Some(lan) = &self.lan {
                        let runtime = tokio::runtime::Runtime::new().expect("runtime");
                        if let Err(e) = runtime.block_on(lan.broadcast_announcement(&name, port)) {
                            self.error_message = Some(format!("LAN announce failed: {}", e));
                        } else {
                            self.status_message = Some("LAN announcement sent".to_string());
                        }
                    }
                }
                AppMessage::ViewFile { server_id, path } => {
                    // File viewing handled in UI
                    self.status_message = Some(format!("Viewing {}", path.display()));
                }
                AppMessage::SaveFile { server_id, path, content } => {
                    let result = self.save_file_sync(&server_id, &path, &content);
                    match &result {
                        Ok(_) => self.status_message = Some("File saved".to_string()),
                        Err(e) => self.error_message = Some(format!("Save failed: {}", e)),
                    }
                }
                AppMessage::SendCommand { server_id, command } => {
                    let result = self.send_command_sync(&server_id, &command);
                    match &result {
                        Ok(_) => self.status_message = Some("Command sent".to_string()),
                        Err(e) => self.error_message = Some(format!("Command failed: {}", e)),
                    }
                }
                AppMessage::UpdateServerConfig { server_id, config } => {
                    let result = self.update_server_config_sync(&server_id, config);
                    match &result {
                        Ok(_) => self.status_message = Some("Configuration updated".to_string()),
                        Err(e) => self.error_message = Some(format!("Update failed: {}", e)),
                    }
                }
                AppMessage::BackupServer { server_id } => {
                    let result = self.backup_server_sync(&server_id);
                    match &result {
                        Ok(path) => self.status_message = Some(format!("Backup saved to {}", path.display())),
                        Err(e) => self.error_message = Some(format!("Backup failed: {}", e)),
                    }
                }
                AppMessage::RestoreServer { server_id, backup_path } => {
                    let result = self.restore_server_sync(&server_id, &backup_path);
                    match &result {
                        Ok(_) => self.status_message = Some("Server restored".to_string()),
                        Err(e) => self.error_message = Some(format!("Restore failed: {}", e)),
                    }
                }
                AppMessage::ShowError { message } => {
                    self.error_message = Some(message);
                }
                AppMessage::ShowStatus { message } => {
                    self.status_message = Some(message);
                }
            }
        }
    }

    // Sync wrappers for async operations
    fn create_server_sync(&mut self, name: String, backend: ServerBackendType, port: u16) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        runtime.block_on(self.create_server(name, backend, port))
    }

    fn start_server_sync(&mut self, server_id: &str) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        let result = runtime.block_on(self.start_server(server_id));
        if result.is_ok() {
            info!("Server {} marked as running", server_id);
        }
        result
    }

    fn stop_server_sync(&self, server_id: &str) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        runtime.block_on(self.stop_server(server_id))
    }

    fn delete_server_sync(&self, server_id: &str) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        runtime.block_on(self.delete_server(server_id))
    }

    fn refresh_servers_sync(&mut self) {
        let runtime = tokio::runtime::Runtime::new().expect("runtime");
        let servers = runtime.block_on(self.refresh_servers());
        self.ui.set_servers(servers);
    }

    fn discover_lan_servers_sync(&self) -> Vec<crate::lan::LanServer> {
        let runtime = tokio::runtime::Runtime::new().expect("runtime");
        runtime.block_on(self.discover_lan_servers())
    }

    fn refresh_marketplace_sync(&mut self) {
        let runtime = tokio::runtime::Runtime::new().expect("runtime");
        let plugins = runtime.block_on(self.refresh_marketplace());
        self.ui.marketplace.loading = false;
        self.ui.marketplace.plugins = plugins.clone();
        self.ui.marketplace.filtered_plugins = plugins;
    }

    // New sync wrappers for additional functionality
    fn install_plugin_sync(&self, server_id: &str, plugin_name: &str, plugin_url: &str) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        runtime.block_on(self.install_plugin(server_id, plugin_name, plugin_url))
    }

    fn uninstall_plugin_sync(&self, server_id: &str, plugin_name: &str) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        runtime.block_on(self.uninstall_plugin(server_id, plugin_name))
    }

    fn toggle_plugin_sync(&self, server_id: &str, plugin_name: &str) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        runtime.block_on(self.toggle_plugin(server_id, plugin_name))
    }

    fn create_tunnel_sync(&self, server_id: &str, port: u16) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        runtime.block_on(self.create_tunnel(server_id, port))
    }

    fn remove_tunnel_sync(&self, server_id: &str) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        runtime.block_on(self.remove_tunnel(server_id))
    }

    fn save_file_sync(&self, server_id: &str, path: &PathBuf, content: &str) -> Result<(), String> {
        std::fs::write(path, content).map_err(|e| format!("Failed to write file: {}", e))
    }

    fn send_command_sync(&self, server_id: &str, command: &str) -> Result<(), String> {
        // Send command to running server process
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        runtime.block_on(self.send_command(server_id, command))
    }

    fn update_server_config_sync(&self, server_id: &str, config: ServerConfig) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        runtime.block_on(self.update_server_config(server_id, config))
    }

    fn backup_server_sync(&self, server_id: &str) -> Result<PathBuf, String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        runtime.block_on(self.backup_server(server_id))
    }

    fn restore_server_sync(&self, server_id: &str, backup_path: &PathBuf) -> Result<(), String> {
        let runtime = tokio::runtime::Runtime::new().map_err(|e| format!("Runtime error: {}", e))?;
        runtime.block_on(self.restore_server(server_id, backup_path))
    }

    pub async fn create_server(&self, name: String, backend: ServerBackendType, port: u16) -> Result<(), String> {
        let server_id = uuid::Uuid::new_v4().to_string();
        let path = Self::get_server_path(&server_id);
        
        std::fs::create_dir_all(&path)
            .map_err(|e| format!("Failed to create server directory: {}", e))?;
        
        let config = ServerConfig {
            id: server_id.clone(),
            name: name.clone(),
            backend,
            version: "1.21".to_string(),
            path,
            port,
            memory_mb: 512,
            auto_tunnel: true,
        };
        
        let backend_impl = get_backend(&config.backend);
        backend_impl.prepare(&config)
            .map_err(|e| format!("Failed to prepare server: {}", e))?;
        
        let metadata = ServerMetadata {
            id: server_id.clone(),
            name,
            backend: config.backend.to_string(),
            version: config.version.clone(),
            path: config.path.clone(),
            port: config.port,
            ip: String::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_started: None,
            is_running: false,
        };
        
        let storage = self.storage.lock().await;
        storage.insert_server(&metadata)
            .map_err(|e| format!("Failed to save server: {}", e))?;
        
        // Auto-create playit tunnel if enabled
        if config.auto_tunnel {
            if let Err(e) = self.playit.create_tunnel(&server_id, config.port).await {
                warn!("Failed to create playit tunnel: {}", e);
            }
        }
        
        info!("Server {} created successfully", server_id);
        Ok(())
    }

    pub async fn start_server(&self, server_id: &str) -> Result<(), String> {
        let storage = self.storage.lock().await;
        let servers = storage.get_all_servers()
            .map_err(|e| format!("Failed to get servers: {}", e))?;
        
        let server = servers.iter()
            .find(|s| s.id == server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;
        
        let backend = match server.backend.as_str() {
            "Pumpkin" => ServerBackendType::Pumpkin,
            "Steel" => ServerBackendType::Steel,
            "Rosewater" => ServerBackendType::Rosewater,
            "Glowstone" => ServerBackendType::Glowstone,
            _ => return Err("Unknown backend".to_string()),
        };
        
        drop(storage); // Release lock before spawning process
        
        let backend_impl = get_backend(&backend);
        
        let config = ServerConfig {
            id: server.id.clone(),
            name: server.name.clone(),
            backend,
            version: server.version.clone(),
            path: server.path.clone(),
            port: server.port,
            memory_mb: 512,
            auto_tunnel: true,
        };
        
        let child = backend_impl.start(&config)
            .map_err(|e| format!("Failed to start server: {}", e))?;
        
        // Track the running process
        {
            let mut running = self.running_servers.lock().await;
            running.insert(server_id.to_string(), child);
        }
        
        let storage = self.storage.lock().await;
        storage.update_running(server_id, true)
            .map_err(|e| format!("Failed to update status: {}", e))?;
        
        info!("Server {} started", server_id);
        Ok(())
    }

    pub async fn stop_server(&self, server_id: &str) -> Result<(), String> {
        // Kill the process
        {
            let mut running = self.running_servers.lock().await;
            if let Some(mut child) = running.remove(server_id) {
                child.kill().map_err(|e| format!("Failed to kill process: {}", e))?;
            }
        }
        
        let storage = self.storage.lock().await;
        storage.update_running(server_id, false)
            .map_err(|e| format!("Failed to update status: {}", e))?;
        
        info!("Server {} stopped", server_id);
        Ok(())
    }

    pub async fn delete_server(&self, server_id: &str) -> Result<(), String> {
        // Remove tunnel if exists
        if let Err(e) = self.playit.remove_tunnel(server_id).await {
            warn!("Failed to remove tunnel for server {}: {}", server_id, e);
        }
        
        // Kill process if running
        {
            let mut running = self.running_servers.lock().await;
            if let Some(mut child) = running.remove(server_id) {
                let _ = child.kill();
            }
        }
        
        let storage = self.storage.lock().await;
        storage.delete_server(server_id)
            .map_err(|e| format!("Failed to delete server: {}", e))?;
        
        let path = Self::get_server_path(server_id);
        if path.exists() {
            std::fs::remove_dir_all(&path)
                .map_err(|e| format!("Failed to delete server files: {}", e))?;
        }
        
        info!("Server {} deleted", server_id);
        Ok(())
    }

    pub async fn refresh_servers(&mut self) -> Vec<ServerMetadata> {
        let storage = self.storage.lock().await;
        storage.get_all_servers().unwrap_or_default()
    }

    pub async fn discover_lan_servers(&self) -> Vec<crate::lan::LanServer> {
        if let Some(lan) = &self.lan {
            lan.discover().await.unwrap_or_else(|e| {
                warn!("LAN discovery failed: {}", e);
                Vec::new()
            })
        } else {
            Vec::new()
        }
    }

    pub async fn refresh_marketplace(&self) -> Vec<crate::plugins::PluginEntry> {
        self.plugin_marketplace.search_plugins("", None).await.unwrap_or_default()
    }

    // New async methods for additional functionality
    pub async fn install_plugin(&self, server_id: &str, plugin_name: &str, plugin_url: &str) -> Result<(), String> {
        let storage = self.storage.lock().await;
        let servers = storage.get_all_servers()
            .map_err(|e| format!("Failed to get servers: {}", e))?;
        
        let server = servers.iter()
            .find(|s| s.id == server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;
        
        let plugins_dir = server.path.join("plugins");
        std::fs::create_dir_all(&plugins_dir)
            .map_err(|e| format!("Failed to create plugins directory: {}", e))?;
        
        // Download plugin using reqwest (pure Rust, works on Android)
        let client = reqwest::Client::new();
        let response = client.get(plugin_url)
            .send()
            .await
            .map_err(|e| format!("Download failed: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("Download failed: HTTP {}", response.status()));
        }
        
        let bytes = response.bytes()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;
        
        // Determine file extension from URL or default to .jar
        let extension = if plugin_url.ends_with(".wasm") { "wasm" } else { "jar" };
        let filename = format!("{}.{}", plugin_name, extension);
        let dest = plugins_dir.join(&filename);
        
        tokio::fs::write(&dest, &bytes)
            .await
            .map_err(|e| format!("Failed to write plugin file: {}", e))?;
        
        // Save plugin info to database
        let plugin_info = crate::storage::PluginInfo {
            name: plugin_name.to_string(),
            version: "1.0.0".to_string(),
            source: plugin_url.to_string(),
            format: if extension == "wasm" { 
                crate::storage::PluginFormat::Wasm 
            } else { 
                crate::storage::PluginFormat::Jar 
            },
            enabled: true,
        };
        
        storage.insert_plugin(server_id, &plugin_info)
            .map_err(|e| format!("Failed to save plugin info: {}", e))?;
        
        info!("Plugin {} installed for server {}", plugin_name, server_id);
        Ok(())
    }

    pub async fn uninstall_plugin(&self, server_id: &str, plugin_name: &str) -> Result<(), String> {
        let storage = self.storage.lock().await;
        // Note: We'd need a delete_plugin method in storage
        // For now, just remove the file
        let servers = storage.get_all_servers()
            .map_err(|e| format!("Failed to get servers: {}", e))?;
        
        let server = servers.iter()
            .find(|s| s.id == server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;
        
        let plugin_file = server.path.join("plugins").join(format!("{}.jar", plugin_name));
        if plugin_file.exists() {
            std::fs::remove_file(&plugin_file)
                .map_err(|e| format!("Failed to delete plugin: {}", e))?;
        }
        
        info!("Plugin {} uninstalled from server {}", plugin_name, server_id);
        Ok(())
    }

    pub async fn toggle_plugin(&self, server_id: &str, plugin_name: &str) -> Result<(), String> {
        let storage = self.storage.lock().await;
        let plugins = storage.get_plugins(server_id)
            .map_err(|e| format!("Failed to get plugins: {}", e))?;
        
        if let Some(plugin) = plugins.iter().find(|p| p.name == plugin_name) {
            let mut updated = plugin.clone();
            updated.enabled = !updated.enabled;
            storage.insert_plugin(server_id, &updated)
                .map_err(|e| format!("Failed to update plugin: {}", e))?;
        }
        
        Ok(())
    }

    pub async fn create_tunnel(&self, server_id: &str, port: u16) -> Result<(), String> {
        match self.playit.create_tunnel(server_id, port).await {
            Ok(tunnel) => {
                // Update server with tunnel info
                let storage = self.storage.lock().await;
                storage.update_server_ip(server_id, &tunnel.public_url)
                    .map_err(|e| format!("Failed to update server IP: {}", e))?;
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn remove_tunnel(&self, server_id: &str) -> Result<(), String> {
        self.playit.remove_tunnel(server_id).await.map_err(|e| e.to_string())
    }

    pub async fn send_command(&self, server_id: &str, command: &str) -> Result<(), String> {
        let processes = self.running_servers.lock().await;
        if let Some(process) = processes.get(server_id) {
            let mut child = process.lock().await;
            if let Some(stdin) = child.stdin.as_mut() {
                use tokio::io::AsyncWriteExt;
                stdin.write_all(command.as_bytes()).await
                    .map_err(|e| format!("Failed to write command: {}", e))?;
                stdin.write_all(b"\n").await
                    .map_err(|e| format!("Failed to write newline: {}", e))?;
                Ok(())
            } else {
                Err("Server stdin not available".to_string())
            }
        } else {
            Err("Server not running".to_string())
        }
    }
    
    fn start_log_streaming(&self, server_id: &str) {
        let processes = self.running_servers.clone();
        let log_viewers = self.log_viewers.clone();
        let server_id = server_id.to_string();
        
        tokio::spawn(async move {
            let process = {
                let processes = processes.lock().await;
                processes.get(&server_id).cloned()
            };
            
            if let Some(process) = process {
                let mut child = process.lock().await;
                
                // Read stdout
                if let Some(stdout) = child.stdout.take() {
                    let log_viewers = log_viewers.clone();
                    let server_id = server_id.clone();
                    tokio::spawn(async move {
                        let mut reader = tokio::io::BufReader::new(stdout);
                        let mut line = String::new();
                        loop {
                            line.clear();
                            match reader.read_line(&mut line).await {
                                Ok(0) => break, // EOF
                                Ok(_) => {
                                    let mut viewers = log_viewers.lock().await;
                                    if let Some(viewer) = viewers.get_mut(&server_id) {
                                        viewer.add_log(line.trim_end().to_string());
                                    }
                                }
                                Err(_) => break,
                            }
                        }
                    });
                }
                
                // Read stderr
                if let Some(stderr) = child.stderr.take() {
                    let log_viewers = log_viewers.clone();
                    let server_id = server_id.clone();
                    tokio::spawn(async move {
                        let mut reader = tokio::io::BufReader::new(stderr);
                        let mut line = String::new();
                        loop {
                            line.clear();
                            match reader.read_line(&mut line).await {
                                Ok(0) => break, // EOF
                                Ok(_) => {
                                    let mut viewers = log_viewers.lock().await;
                                    if let Some(viewer) = viewers.get_mut(&server_id) {
                                        viewer.add_log(format!("[STDERR] {}", line.trim_end()));
                                    }
                                }
                                Err(_) => break,
                            }
                        }
                    });
                }
            }
        });
    }

    pub async fn update_server_config(&self, server_id: &str, config: ServerConfig) -> Result<(), String> {
        let metadata = crate::storage::ServerMetadata {
            id: config.id.clone(),
            name: config.name.clone(),
            backend: config.backend.to_string(),
            version: config.version.clone(),
            path: config.path.clone(),
            port: config.port,
            ip: String::new(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_started: None,
            is_running: false,
        };
        
        let storage = self.storage.lock().await;
        storage.insert_server(&metadata)
            .map_err(|e| format!("Failed to update server: {}", e))?;
        
        Ok(())
    }

pub async fn backup_server(&self, server_id: &str) -> Result<PathBuf, String> {
        let storage = self.storage.lock().await;
        let servers = storage.get_all_servers()
            .map_err(|e| format!("Failed to get servers: {}", e))?;
        
        let server = servers.iter()
            .find(|s| s.id == server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;
        
        let backup_dir = Self::get_cache_dir().join("backups");
        std::fs::create_dir_all(&backup_dir)
            .map_err(|e| format!("Failed to create backup directory: {}", e))?;
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let backup_name = format!("{}_{}.tar.gz", server.name, timestamp);
        let backup_path = backup_dir.join(&backup_name);
        
        // Create tar.gz archive using pure Rust (flate2 + tar)
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use tar::Builder;
        use tokio::fs::File;
        use tokio::io::AsyncWriteExt;
        
        let file = File::create(&backup_path).await
            .map_err(|e| format!("Failed to create backup file: {}", e))?;
        let encoder = GzEncoder::new(file, Compression::default());
        let mut builder = Builder::new(encoder);
        
        // Add all files from server directory
        let server_path = &server.path;
        Self::add_dir_to_tar(&mut builder, server_path, std::path::Path::new(""))
            .map_err(|e| format!("Failed to create archive: {}", e))?;
        
        builder.into_inner()
            .map_err(|e| format!("Failed to finalize archive: {}", e))?
            .finish()
            .map_err(|e| format!("Failed to finish compression: {}", e))?;
        
        Ok(backup_path)
    }
    
    fn add_dir_to_tar<W: std::io::Write>(builder: &mut Builder<W>, base_path: &std::path::Path, relative_path: &std::path::Path) -> std::io::Result<()> {
        let full_path = base_path.join(relative_path);
        let entries = std::fs::read_dir(&full_path)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();
            let relative_entry_path = relative_path.join(&name);
            
            if path.is_dir() {
                Self::add_dir_to_tar(builder, base_path, &relative_entry_path)?;
            } else {
                let mut file = std::fs::File::open(&path)?;
                let mut header = tar::Header::new_gnu();
                header.set_path(&relative_entry_path)?;
                header.set_size(entry.metadata()?.len());
                header.set_cksum();
                builder.append(&header, &mut file)?;
            }
        }
        Ok(())
    }

    pub async fn restore_server(&self, server_id: &str, backup_path: &PathBuf) -> Result<(), String> {
        let storage = self.storage.lock().await;
        let servers = storage.get_all_servers()
            .map_err(|e| format!("Failed to get servers: {}", e))?;
        
        let server = servers.iter()
            .find(|s| s.id == server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;
        
        // Stop server if running
        let _ = self.stop_server(server_id).await;
        
        // Extract backup using pure Rust (flate2 + tar)
        use flate2::read::GzDecoder;
        use tar::Archive;
        use tokio::fs::File;
        
        let file = File::open(backup_path).await
            .map_err(|e| format!("Failed to open backup file: {}", e))?;
        let decoder = GzDecoder::new(file);
        let mut archive = Archive::new(decoder);
        
        // Extract to server directory
        archive.unpack(&server.path)
            .map_err(|e| format!("Failed to extract backup: {}", e))?;
        
        Ok(())
    }

    fn render_server_detail_dialog(
        &mut self,
        ctx: &egui::Context,
        server: &ServerMetadata,
        detail: &mut ServerDetailState,
        messages: &mut Vec<AppMessage>,
    ) {
        use crate::ui::theme::*;
        use crate::ui::components::*;
        
        egui::Window::new(format!("{} - {}", server.name, server.backend))
            .collapsible(false)
            .resizable(true)
            .default_size([800.0, 600.0])
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                // Tab bar
                ui.horizontal(|ui| {
                    let tabs = [
                        (ServerDetailTab::Overview, "Overview"),
                        (ServerDetailTab::Console, "Console"),
                        (ServerDetailTab::Files, "Files"),
                        (ServerDetailTab::Plugins, "Plugins"),
                        (ServerDetailTab::Resources, "Resources"),
                        (ServerDetailTab::Settings, "Settings"),
                    ];
                    
                    for (tab, label) in tabs {
                        let selected = detail.active_tab == tab;
                        let response = ui.selectable_label(selected, label);
                        if response.clicked() {
                            detail.active_tab = tab;
                        }
                        ui.add_space(8.0);
                    }
                    
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if danger_button(ui, "Delete").clicked() {
                            messages.push(AppMessage::DeleteServer { server_id: server.id.clone() });
                        }
                    });
                });
                
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);
                
                match detail.active_tab {
                    ServerDetailTab::Overview => self.render_overview_tab(ui, server, detail, messages),
                    ServerDetailTab::Console => self.render_console_tab(ui, server, detail, messages),
                    ServerDetailTab::Files => self.render_files_tab(ui, server, detail, messages),
                    ServerDetailTab::Plugins => self.render_plugins_tab(ui, server, detail, messages),
                    ServerDetailTab::Resources => self.render_resources_tab(ui, server, detail, messages),
                    ServerDetailTab::Settings => self.render_server_settings_tab(ui, server, detail, messages),
                }
            });
    }

    fn render_overview_tab(&mut self, ui: &mut Ui, server: &ServerMetadata, detail: &mut ServerDetailState, messages: &mut Vec<AppMessage>) {
        use crate::ui::theme::*;
        use crate::ui::components::*;
        
        ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            // Status card
            card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Status").size(14.0).color(ON_SURFACE_VARIANT));
                        ui.add_space(4.0);
                        status_chip(ui, 
                            if server.is_running { "Running" } else { "Stopped" }, 
                            ServerStatus::from_bool(server.is_running)
                        );
                    });
                    
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        if server.is_running {
                            if danger_button(ui, "Stop").clicked() {
                                messages.push(AppMessage::StopServer { server_id: server.id.clone() });
                            }
                            ui.add_space(8.0);
                            if secondary_button(ui, "Restart").clicked() {
                                messages.push(AppMessage::RestartServer { server_id: server.id.clone() });
                            }
                        } else {
                            if primary_button(ui, "Start").clicked() {
                                messages.push(AppMessage::StartServer { server_id: server.id.clone() });
                            }
                        }
                    });
                });
            });
            ui.add_space(16.0);
            
            // Info grid
            ui.horizontal(|ui| {
                // Left column
                ui.vertical(|ui| {
                    card_frame().show(ui, |ui| {
                        render_info_row(ui, "Backend", &server.backend);
                        render_info_row(ui, "Version", &server.version);
                        render_info_row(ui, "Port", &server.port.to_string());
                    });
                    ui.add_space(12.0);
                    card_frame().show(ui, |ui| {
                        render_info_row(ui, "Path", &server.path.to_string_lossy());
                        render_info_row(ui, "Created", &format_timestamp(server.created_at));
                    });
                });
                
                ui.add_space(12.0);
                
                // Right column
                ui.vertical(|ui| {
                    card_frame().show(ui, |ui| {
                        render_info_row(ui, "Playit Tunnel", if server.ip.is_empty() { "Not configured" } else { &server.ip });
                        render_info_row(ui, "Last Started", &server.last_started.map_or("Never".to_string(), |t| format_timestamp(t)));
                    });
                    ui.add_space(12.0);
                    
                    // Quick actions
                    card_frame().show(ui, |ui| {
                        ui.strong("Quick Actions");
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            if outlined_button(ui, "Open Folder").clicked() {
                                // Open folder
                            }
                            ui.add_space(8.0);
                            if outlined_button(ui, "Backup").clicked() {
                                messages.push(AppMessage::BackupServer { server_id: server.id.clone() });
                            }
                            ui.add_space(8.0);
                            if outlined_button(ui, "Console").clicked() {
                                detail.active_tab = ServerDetailTab::Console;
                            }
                        });
                    });
                });
            });
        });
    }

    fn render_console_tab(&mut self, ui: &mut Ui, server: &ServerMetadata, detail: &mut ServerDetailState, _messages: &mut Vec<AppMessage>) {
        use crate::ui::theme::*;
        use crate::ui::components::*;
        use crate::ui::logs::LogViewer;
        
        // Get or create log viewer for this server
        let mut log_viewer = self.log_viewers.lock().await
            .entry(server.id.clone())
            .or_insert_with(LogViewer::default)
            .clone();
        
        // If server is running, start log streaming
        if server.is_running {
            self.start_log_streaming(&server.id);
        }
        
        // Render log viewer
        render_log_viewer(ui, &mut log_viewer);
        
        // Save updated log viewer back
        self.log_viewers.lock().await.insert(server.id.clone(), log_viewer);
        
        // Command input
        ui.add_space(8.0);
        card_frame().show(ui, |ui| {
            ui.horizontal(|ui| {
                let mut command = String::new();
                let response = ui.add(TextEdit::singleline(&mut command)
                    .hint_text("Enter command...")
                    .desired_width(f32::INFINITY));
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if !command.trim().is_empty() {
                        // Send command
                    }
                }
                ui.add_space(8.0);
                if primary_button(ui, "Send").clicked() {
                    if !command.trim().is_empty() {
                        // Send command
                    }
                }
            });
        });
    }

    fn render_files_tab(&mut self, ui: &mut Ui, server: &ServerMetadata, detail: &mut ServerDetailState, messages: &mut Vec<AppMessage>) {
        use crate::ui::theme::*;
        use crate::ui::components::*;
        use crate::ui::file_browser::{render_file_browser, render_file_viewer, FileEntry};
        
        ui.horizontal(|ui| {
            ui.strong("Files");
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if outlined_button(ui, "New File").clicked() {
                    // Create new file
                }
                ui.add_space(8.0);
                if outlined_button(ui, "New Folder").clicked() {
                    // Create new folder
                }
                ui.add_space(8.0);
                if outlined_button(ui, "Upload").clicked() {
                    // Upload file
                }
            });
        });
        ui.add_space(8.0);
        
        let mut current_path = detail.file_browser_path.clone();
        let mut selected_file = detail.selected_file.clone();
        
        render_file_browser(
            ui,
            &mut current_path,
            &mut selected_file,
            |path| {
                // View file
                if let Ok(content) = std::fs::read_to_string(path) {
                    // Show file viewer
                }
            },
            |path| {
                // Navigate folder
                detail.file_browser_path = path.clone();
            },
        );
        
        detail.file_browser_path = current_path;
        detail.selected_file = selected_file;
    }

    fn render_plugins_tab(&mut self, ui: &mut Ui, server: &ServerMetadata, _detail: &mut ServerDetailState, messages: &mut Vec<AppMessage>) {
        use crate::ui::theme::*;
        use crate::ui::components::*;
        
        ui.horizontal(|ui| {
            ui.strong("Plugins");
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if primary_button(ui, "Install Plugin").clicked() {
                    // Open marketplace with this server selected
                    self.ui.marketplace.server_id = Some(server.id.clone());
                    self.ui.active_tab = crate::ui::Tab::Marketplace;
                }
            });
        });
        ui.add_space(8.0);
        
        // Load plugins from storage
        let plugins = self.storage.lock().await.get_plugins(&server.id).unwrap_or_default();
        
        if plugins.is_empty() {
            render_empty_state(ui, "📦", "No plugins installed", "Install plugins from the Marketplace tab", Some("Open Marketplace"), || {
                self.ui.active_tab = crate::ui::Tab::Marketplace;
                self.ui.marketplace.server_id = Some(server.id.clone());
            });
        } else {
            ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                for plugin in plugins {
                    card_frame().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(&plugin.name).size(16.0).color(ON_SURFACE).strong());
                                    ui.add_space(8.0);
                                    format_badge(ui, &plugin.format);
                                    ui.add_space(8.0);
                                    version_badge(ui, &plugin.version);
                                });
                                ui.add_space(4.0);
                                ui.label(RichText::new(&plugin.source).size(11.0).color(ON_SURFACE_VARIANT).monospace());
                            });
                            
                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                let enabled = plugin.enabled;
                                let mut toggle = enabled;
                                if ui.checkbox(&mut toggle, "").changed() {
                                    messages.push(AppMessage::TogglePlugin { 
                                        server_id: server.id.clone(), 
                                        plugin_name: plugin.name.clone() 
                                    });
                                }
                                ui.add_space(8.0);
                                if outlined_button(ui, "Remove").clicked() {
                                    messages.push(AppMessage::UninstallPlugin { 
                                        server_id: server.id.clone(), 
                                        plugin_name: plugin.name.clone() 
                                    });
                                }
                            });
                        });
                    });
                    ui.add_space(8.0);
                }
            });
        }
    }

    fn render_resources_tab(&mut self, ui: &mut Ui, server: &ServerMetadata, _detail: &mut ServerDetailState, _messages: &mut Vec<AppMessage>) {
        use crate::ui::resources::{render_resources_tab, ResourceMonitor, collect_system_metrics};
        
        // Get or create resource monitor
        let monitor = self.resource_monitors.lock().await
            .entry(server.id.clone())
            .or_insert_with(ResourceMonitor::default)
            .clone();
        
        // Update metrics if needed
        if monitor.should_update() {
            let metrics = collect_system_metrics(&server.path);
            // Update would happen here
        }
        
        render_resources_tab(ui, &monitor, &server.id);
    }

    fn render_server_settings_tab(&mut self, ui: &mut Ui, server: &ServerMetadata, _detail: &mut ServerDetailState, messages: &mut Vec<AppMessage>) {
        use crate::ui::theme::*;
        use crate::ui::components::*;
        
        ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            // Server properties
            card_frame().show(ui, |ui| {
                ui.strong("Server Properties");
                ui.add_space(12.0);
                
                let mut name = server.name.clone();
                let mut port = server.port;
                let mut memory_mb = 1024; // Would come from config
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Name").size(14.0).color(ON_SURFACE));
                    ui.add_space(12.0);
                    ui.add(TextEdit::singleline(&mut name).desired_width(200.0));
                });
                ui.add_space(12.0);
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Port").size(14.0).color(ON_SURFACE));
                    ui.add_space(12.0);
                    ui.add(DragValue::new(&mut port).speed(1.0).range(1024..=65535));
                });
                ui.add_space(12.0);
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Memory (MB)").size(14.0).color(ON_SURFACE));
                    ui.add_space(12.0);
                    ui.add(DragValue::new(&mut memory_mb).speed(64.0).range(256..=8192).suffix(" MB"));
                });
                ui.add_space(12.0);
                
                if primary_button(ui, "Save Changes").clicked() {
                    let config = crate::server::ServerConfig {
                        id: server.id.clone(),
                        name: name.clone(),
                        backend: match server.backend.as_str() {
                            "Pumpkin" => crate::server::ServerBackendType::Pumpkin,
                            "Steel" => crate::server::ServerBackendType::Steel,
                            "Rosewater" => crate::server::ServerBackendType::Rosewater,
                            "Glowstone" => crate::server::ServerBackendType::Glowstone,
                            _ => crate::server::ServerBackendType::Pumpkin,
                        },
                        version: server.version.clone(),
                        path: server.path.clone(),
                        port,
                        memory_mb,
                        auto_tunnel: true,
                    };
                    messages.push(AppMessage::UpdateServerConfig { server_id: server.id.clone(), config });
                }
            });
            ui.add_space(16.0);
            
            // Tunnel settings
            card_frame().show(ui, |ui| {
                ui.strong("Playit.gg Tunnel");
                ui.add_space(12.0);
                
                if server.ip.is_empty() {
                    ui.label(RichText::new("No active tunnel").size(14.0).color(ON_SURFACE_VARIANT));
                    ui.add_space(8.0);
                    if primary_button(ui, "Create Tunnel").clicked() {
                        messages.push(AppMessage::CreateTunnel { server_id: server.id.clone(), port: server.port });
                    }
                } else {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Public Address").size(14.0).color(ON_SURFACE));
                        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(&server.ip).size(14.0).color(PRIMARY).monospace().strong());
                        });
                    });
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if outlined_button(ui, "Copy").clicked() {
                            ui.ctx().copy_text(server.ip.clone());
                        }
                        ui.add_space(8.0);
                        if danger_button(ui, "Remove Tunnel").clicked() {
                            messages.push(AppMessage::RemoveTunnel { server_id: server.id.clone() });
                        }
                    });
                }
            });
            ui.add_space(16.0);
            
            // Danger zone
            card_frame().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("⚠ Danger Zone").size(16.0).color(ERROR).strong());
                });
                ui.add_space(8.0);
                ui.label(RichText::new("These actions are irreversible").size(13.0).color(ON_SURFACE_VARIANT));
                ui.add_space(12.0);
                
                ui.horizontal(|ui| {
                    if danger_button(ui, "Delete Server").clicked() {
                        messages.push(AppMessage::DeleteServer { server_id: server.id.clone() });
                    }
                });
            });
        });
    }
}

impl eframe::App for IntmcApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        let messages = self.ui.render(ui, &self.storage, &self.playit);
        self.process_messages(messages, ui.ctx());
        
        // Update UI state after the panel
        self.ui.set_error(self.error_message.take());
        self.ui.set_status(self.status_message.take());
        self.ui.set_lan_servers(self.lan_servers.clone());
        
        // Handle create server dialog
        if self.show_create_dialog {
            if let Some(create_data) = &mut self.create_server_data {
                let mut should_close = false;
                let mut create_messages = Vec::new();
                
                render_create_server_dialog(ui.ctx(), create_data, &mut should_close, &mut create_messages);
                
                if should_close {
                    self.show_create_dialog = false;
                    self.create_server_data = None;
                }
                self.process_messages(create_messages, ui.ctx());
            }
        }
        
        // Handle server detail dialog
        if let Some(detail) = &mut self.ui.server_detail.clone() {
            if let Some(server) = self.ui.servers.iter().find(|s| s.id == detail.server_id).cloned() {
                let mut detail = detail.clone();
                let mut detail_messages = Vec::new();
                
                self.render_server_detail_dialog(ui.ctx(), &server, &mut detail, &mut detail_messages);
                
                if detail_messages.iter().any(|m| matches!(m, AppMessage::DeleteServer { .. })) {
                    self.ui.server_detail = None;
                    self.ui.selected_server = None;
                } else {
                    self.ui.server_detail = Some(detail);
                }
                self.process_messages(detail_messages, ui.ctx());
            } else {
                self.ui.server_detail = None;
                self.ui.selected_server = None;
            }
        }
        
        ui.ctx().request_repaint();
    }
}