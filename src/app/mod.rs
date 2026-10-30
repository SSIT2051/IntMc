use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Child;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::storage::{Storage, ServerMetadata};
use crate::ui::IntmcUI;
use crate::playit::PlayitManager;
use crate::lan::LanDiscovery;
use crate::server::{ServerConfig, ServerBackendType, get_backend};
use log::{info, warn};
use eframe::egui::{Context, Ui};
use eframe::Frame;

#[derive(Debug, Clone)]
pub enum AppMessage {
    CreateServer { name: String, backend: ServerBackendType, port: u16 },
    StartServer { server_id: String },
    StopServer { server_id: String },
    DeleteServer { server_id: String },
    RefreshServers,
    DiscoverLan,
    JoinLanServer { name: String, ip: String, port: u16 },
    OpenSettings { server_id: String },
}

pub struct IntmcApp {
    ui: IntmcUI,
    playit: PlayitManager,
    lan: Option<LanDiscovery>,
    storage: Arc<Mutex<Storage>>,
    running_servers: Arc<Mutex<HashMap<String, Child>>>,
    error_message: Option<String>,
    status_message: Option<String>,
    lan_servers: Vec<crate::lan::LanServer>,
}

impl IntmcApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let db_path = Self::get_db_path();
        let storage = Storage::open(&db_path).expect("Failed to open database");
        
        let playit = PlayitManager::new();
        let lan = LanDiscovery::new().ok();
        
        let storage_arc = Arc::new(Mutex::new(storage));
        let running_servers = Arc::new(Mutex::new(HashMap::new()));
        let ui = IntmcUI::new();
        
        Self {
            ui,
            playit,
            lan,
            storage: storage_arc,
            running_servers,
            error_message: None,
            status_message: None,
            lan_servers: Vec::new(),
        }
    }

    fn get_db_path() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            PathBuf::from("/data/data/intmc/intmc.db")
        }
        #[cfg(not(target_os = "android"))]
        {
            PathBuf::from("intmc.db")
        }
    }

    fn get_server_path(server_id: &str) -> PathBuf {
        #[cfg(target_os = "android")]
        {
            PathBuf::from(format!("/data/data/intmc/servers/{}", server_id))
        }
        #[cfg(not(target_os = "android"))]
        {
            PathBuf::from(format!("servers/{}", server_id))
        }
    }

    /// Process UI messages synchronously (called from egui ui callback)
    pub fn process_messages(&mut self, messages: Vec<AppMessage>, ctx: &Context) {
        for msg in messages {
            match msg {
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
                AppMessage::DeleteServer { server_id } => {
                    let result = self.delete_server_sync(&server_id);
                    match &result {
                        Ok(_) => self.status_message = Some("Server deleted".to_string()),
                        Err(e) => self.error_message = Some(format!("Delete failed: {}", e)),
                    }
                }
                AppMessage::RefreshServers => {
                    self.refresh_servers_sync();
                }
                AppMessage::DiscoverLan => {
                    self.lan_servers = self.discover_lan_servers_sync();
                }
                AppMessage::JoinLanServer { name, ip, port } => {
                    self.status_message = Some(format!("Joining {} at {}:{}", name, ip, port));
                }
                AppMessage::OpenSettings { server_id } => {
                    self.status_message = Some(format!("Opening settings for {}", server_id));
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
}

impl eframe::App for IntmcApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        let messages = self.ui.render(ui);
        self.process_messages(messages, ui.ctx());
        
        // Update UI state after the panel
        self.ui.set_error(self.error_message.take());
        self.ui.set_status(self.status_message.take());
        self.ui.set_lan_servers(self.lan_servers.clone());
        
        ui.ctx().request_repaint();
    }
}