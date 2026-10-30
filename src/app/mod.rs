use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::storage::{Storage, ServerMetadata};
use crate::ui::IntmcUI;
use crate::playit::PlayitManager;
use crate::lan::LanDiscovery;
use crate::server::{ServerConfig, ServerBackendType, get_backend};
use log::{info, warn};
use eframe::egui::Context;
use eframe::Frame;

pub struct IntmcApp {
    ui: IntmcUI,
    playit: PlayitManager,
    lan: Option<LanDiscovery>,
    storage: Arc<Mutex<Storage>>,
}

impl IntmcApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let db_path = Self::get_db_path();
        let storage = Storage::open(&db_path).expect("Failed to open database");
        
        let playit = PlayitManager::new();
        let lan = LanDiscovery::new().ok();
        
        let storage_arc = Arc::new(Mutex::new(storage));
        let ui = IntmcUI::new();
        
        Self {
            ui,
            playit,
            lan,
            storage: storage_arc,
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
        
        backend_impl.start(&config)
            .map_err(|e| format!("Failed to start server: {}", e))?;
        
        storage.update_running(server_id, true)
            .map_err(|e| format!("Failed to update status: {}", e))?;
        
        info!("Server {} started", server_id);
        Ok(())
    }

    pub async fn stop_server(&self, server_id: &str) -> Result<(), String> {
        let storage = self.storage.lock().await;
        storage.update_running(server_id, false)
            .map_err(|e| format!("Failed to update status: {}", e))?;
        
        info!("Server {} stopped", server_id);
        Ok(())
    }

    pub async fn delete_server(&self, server_id: &str) -> Result<(), String> {
        if let Err(e) = self.playit.remove_tunnel(server_id).await {
            warn!("Failed to remove tunnel for server {}: {}", server_id, e);
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

    pub async fn refresh_servers(&mut self) {
        let storage = self.storage.lock().await;
        if let Ok(servers) = storage.get_all_servers() {
            self.ui.set_servers(servers);
        }
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
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui.render(ctx, ui);
        });
    }
}