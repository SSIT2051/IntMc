use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::Mutex as ParkingLotMutex;

use crate::storage::{Storage, ServerMetadata, PluginInfo, PluginFormat};
use crate::playit::PlayitManager;
use crate::lan::{LanDiscovery, LanServer};
use crate::server::{ServerConfig, ServerBackendType, ServerProcess, get_backend};

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

#[derive(Clone)]
pub struct AppState {
    pub playit: PlayitManager,
    pub lan: Option<LanDiscovery>,
    pub storage: Arc<ParkingLotMutex<Storage>>,
    pub running_servers: Arc<ParkingLotMutex<HashMap<String, ServerProcess>>>,
    pub plugin_marketplace: PluginMarketplace,
    pub error_message: Arc<ParkingLotMutex<Option<String>>>,
    pub status_message: Arc<ParkingLotMutex<Option<String>>>,
    pub lan_servers: Arc<ParkingLotMutex<Vec<LanServer>>>,
    pub show_create_dialog: Arc<ParkingLotMutex<bool>>,
    pub console_command: Arc<ParkingLotMutex<String>>,
    pub servers: Arc<ParkingLotMutex<Vec<ServerMetadata>>>,
    pub active_server_detail: Arc<ParkingLotMutex<Option<String>>>,
    pub console_output: Arc<ParkingLotMutex<String>>,
}

impl AppState {
    pub fn new(storage: Storage) -> Self {
        let storage_arc = Arc::new(ParkingLotMutex::new(storage));
        let running_servers = Arc::new(ParkingLotMutex::new(HashMap::new()));
        let playit = PlayitManager::new();
        let lan = LanDiscovery::new().ok();
        let plugin_marketplace = PluginMarketplace::new(Self::get_cache_dir());

        Self {
            playit,
            lan,
            storage: storage_arc,
            running_servers,
            plugin_marketplace,
            error_message: Arc::new(ParkingLotMutex::new(None)),
            status_message: Arc::new(ParkingLotMutex::new(None)),
            lan_servers: Arc::new(ParkingLotMutex::new(Vec::new())),
            show_create_dialog: Arc::new(ParkingLotMutex::new(false)),
            console_command: Arc::new(ParkingLotMutex::new(String::new())),
            servers: Arc::new(ParkingLotMutex::new(Vec::new())),
            active_server_detail: Arc::new(ParkingLotMutex::new(None)),
            console_output: Arc::new(ParkingLotMutex::new(String::new())),
        }
    }

    // ---- Server management ----

    pub fn load_servers(&self) {
        let storage = self.storage.lock();
        match storage.get_all_servers() {
            Ok(servers) => {
                *self.servers.lock() = servers;
            }
            Err(e) => {
                *self.error_message.lock() = Some(format!("Failed to load servers: {}", e));
            }
        }
    }

    pub fn get_servers(&self) -> Vec<ServerMetadata> {
        self.servers.lock().clone()
    }

    pub fn get_server(&self, server_id: &str) -> Option<ServerMetadata> {
        self.servers.lock().iter().find(|s| s.id == server_id).cloned()
    }

    pub fn create_server(&self, name: String, backend: ServerBackendType, port: u16) -> Result<(), String> {
        let config = ServerConfig {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.clone(),
            backend: backend.clone(),
            version: "1.21".to_string(),
            path: std::path::PathBuf::from(format!("servers/{}", name)),
            port,
            memory_mb: 1024,
            auto_tunnel: false,
        };

        // Prepare server directory
        let backend_impl = get_backend(&backend);
        if let Err(e) = backend_impl.prepare(&config) {
            return Err(format!("Prepare failed: {}", e));
        }

        let metadata = ServerMetadata {
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

        let storage = self.storage.lock();
        storage.insert_server(&metadata).map_err(|e| e.to_string())?;
        drop(storage);

        self.load_servers();
        *self.status_message.lock() = Some(format!("Server '{}' created", name));
        Ok(())
    }

    pub fn start_server(&self, server_id: &str) -> Result<(), String> {
        let server = self.get_server(server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;

        let backend_type: ServerBackendType = match server.backend.as_str() {
            "Pumpkin" => ServerBackendType::Pumpkin,
            "Steel" => ServerBackendType::Steel,
            "Rosewater" => ServerBackendType::Rosewater,
            "Glowstone" => ServerBackendType::Glowstone,
            _ => return Err(format!("Unknown backend: {}", server.backend)),
        };

        let config = ServerConfig {
            id: server.id.clone(),
            name: server.name.clone(),
            backend: backend_type,
            version: server.version.clone(),
            path: server.path.clone(),
            port: server.port,
            memory_mb: 1024,
            auto_tunnel: false,
        };

        let backend_impl = get_backend(&config.backend);
        match backend_impl.start(&config) {
            Ok(process) => {
                self.running_servers.lock().insert(server_id.to_string(), process);
                self.storage.lock().update_running(server_id, true).map_err(|e| e.to_string())?;
                self.load_servers();
                *self.status_message.lock() = Some(format!("Server '{}' started", server.name));
                Ok(())
            }
            Err(e) => Err(format!("Start failed: {}", e)),
        }
    }

    pub fn stop_server(&self, server_id: &str) -> Result<(), String> {
        let server = self.get_server(server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;

        let backend_type: ServerBackendType = match server.backend.as_str() {
            "Pumpkin" => ServerBackendType::Pumpkin,
            "Steel" => ServerBackendType::Steel,
            "Rosewater" => ServerBackendType::Rosewater,
            "Glowstone" => ServerBackendType::Glowstone,
            _ => return Err(format!("Unknown backend: {}", server.backend)),
        };

        let config = ServerConfig {
            id: server.id.clone(),
            name: server.name.clone(),
            backend: backend_type,
            version: server.version.clone(),
            path: server.path.clone(),
            port: server.port,
            memory_mb: 1024,
            auto_tunnel: false,
        };

        let backend_impl = get_backend(&config.backend);

        if let Some(process) = self.running_servers.lock().remove(server_id) {
            match backend_impl.stop(&process) {
                Ok(_) => {
                    self.storage.lock().update_running(server_id, false).map_err(|e| e.to_string())?;
                    self.load_servers();
                    *self.status_message.lock() = Some(format!("Server '{}' stopped", server.name));
                    Ok(())
                }
                Err(e) => Err(format!("Stop failed: {}", e)),
            }
        } else {
            Err("Server not running".to_string())
        }
    }

    pub fn delete_server(&self, server_id: &str) -> Result<(), String> {
        let server = self.get_server(server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;

        // Stop if running
        if self.running_servers.lock().contains_key(server_id) {
            self.stop_server(server_id)?;
        }

        let storage = self.storage.lock();
        storage.delete_server(server_id).map_err(|e| e.to_string())?;
        drop(storage);

        self.load_servers();
        *self.status_message.lock() = Some(format!("Server '{}' deleted", server.name));
        Ok(())
    }

    pub fn send_command(&self, server_id: &str, command: &str) -> Result<(), String> {
        let server = self.get_server(server_id)
            .ok_or_else(|| format!("Server not found: {}", server_id))?;

        if let Some(_process) = self.running_servers.lock().get(server_id) {
            *self.console_output.lock() = format!("{} > {}\n", server.name, command);
            *self.status_message.lock() = Some("Command sent".to_string());
            Ok(())
        } else {
            Err("Server not running".to_string())
        }
    }

    // ---- Tunnel management ----

    pub async fn create_tunnel_async(&self, server_id: &str, local_port: u16) -> Result<(), String> {
        self.playit.create_tunnel(server_id, local_port).await
            .map_err(|e| format!("Tunnel creation failed: {}", e))?;
        *self.status_message.lock() = Some("Tunnel created".to_string());
        Ok(())
    }

    pub async fn remove_tunnel_async(&self, server_id: &str) -> Result<(), String> {
        self.playit.remove_tunnel(server_id).await
            .map_err(|e| format!("Tunnel removal failed: {}", e))?;
        *self.status_message.lock() = Some("Tunnel removed".to_string());
        Ok(())
    }

    pub fn is_tunnel_available(&self) -> bool {
        self.playit.is_available()
    }

    // ---- LAN discovery ----

    pub async fn discover_lan_async(&self) -> Result<Vec<LanServer>, String> {
        let lan = self.lan.as_ref()
            .ok_or_else(|| "LAN discovery not available".to_string())?;
        let servers = lan.discover().await
            .map_err(|e| format!("LAN discovery failed: {}", e))?;
        *self.lan_servers.lock() = servers.clone();
        Ok(servers)
    }

    pub fn get_lan_servers(&self) -> Vec<LanServer> {
        self.lan_servers.lock().clone()
    }

    // ---- Plugin management ----

    pub fn get_plugins(&self, server_id: &str) -> Result<Vec<PluginInfo>, String> {
        let storage = self.storage.lock();
        storage.get_plugins(server_id).map_err(|e| e.to_string())
    }

    pub fn toggle_plugin(&self, server_id: &str, plugin_name: &str) -> Result<(), String> {
        let storage = self.storage.lock();
        let mut plugins = storage.get_plugins(server_id).map_err(|e| e.to_string())?;
        if let Some(plugin) = plugins.iter_mut().find(|p| p.name == plugin_name) {
            plugin.enabled = !plugin.enabled;
            storage.insert_plugin(server_id, plugin).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err(format!("Plugin not found: {}", plugin_name))
        }
    }

    pub async fn install_plugin_async(&self, server_id: &str, plugin_name: &str) -> Result<(), String> {
        // Search marketplace for the plugin
        let plugins = self.plugin_marketplace.search_plugins(plugin_name, None).await
            .map_err(|e| format!("Plugin search failed: {}", e))?;
        if let Some(entry) = plugins.into_iter().find(|p| p.name == plugin_name) {
            let _dest = self.plugin_marketplace.download_plugin(&entry, server_id).await
                .map_err(|e| format!("Plugin download failed: {}", e))?;
            let plugin_info = PluginInfo {
                name: entry.name.clone(),
                version: entry.version.clone(),
                source: entry.download_url.clone(),
                format: entry.format,
                enabled: true,
            };
            let storage = self.storage.lock();
            storage.insert_plugin(server_id, &plugin_info).map_err(|e| e.to_string())?;
            *self.status_message.lock() = Some(format!("Plugin '{}' installed", plugin_name));
            Ok(())
        } else {
            Err(format!("Plugin not found in marketplace: {}", plugin_name))
        }
    }

    // ---- Marketplace ----

    pub async fn refresh_marketplace_async(&self) -> Result<Vec<PluginEntry>, String> {
        let plugins = self.plugin_marketplace.search_plugins("", None).await
            .map_err(|e| format!("Marketplace refresh failed: {}", e))?;
        Ok(plugins)
    }

    // ---- Status / Error ----

    pub fn take_status(&self) -> Option<String> {
        self.status_message.lock().take()
    }

    pub fn take_error(&self) -> Option<String> {
        self.error_message.lock().take()
    }

    // ---- Path helpers ----

    pub fn get_db_path() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            PathBuf::from("/data/data/rust.intmc/intmc.db")
        }
        #[cfg(not(target_os = "android"))]
        {
            PathBuf::from("intmc.db")
        }
    }

    pub fn get_cache_dir() -> PathBuf {
        #[cfg(target_os = "android")]
        {
            PathBuf::from("/data/data/rust.intmc/cache")
        }
        #[cfg(not(target_os = "android"))]
        {
            PathBuf::from("cache")
        }
    }
}

pub fn get_db_path() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        PathBuf::from("/data/data/rust.intmc/intmc.db")
    }
    #[cfg(not(target_os = "android"))]
    {
        PathBuf::from("intmc.db")
    }
}

pub fn get_cache_dir() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        PathBuf::from("/data/data/rust.intmc/cache")
    }
    #[cfg(not(target_os = "android"))]
    {
        PathBuf::from("cache")
    }
}

#[derive(Clone)]
pub struct PluginMarketplace {
    cache_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct PluginEntry {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub download_url: String,
    pub format: PluginFormat,
    pub sha256: Option<String>,
}

impl PluginMarketplace {
    pub fn new<P: Into<PathBuf>>(cache_dir: P) -> Self {
        Self {
            cache_dir: cache_dir.into(),
        }
    }

    pub async fn search_plugins(&self, query: &str, _format: Option<PluginFormat>) -> Result<Vec<PluginEntry>, String> {
        // Fetch from all sources (mock data for now)
        let mut all_plugins = Vec::new();

        all_plugins.push(PluginEntry {
            name: "EssentialsX".into(),
            version: "2.195.0".into(),
            description: "Essential plugin for Minecraft servers".into(),
            author: "EssentialsX Team".into(),
            download_url: "https://github.com/EssentialsX/Essentials/releases/download/2.195.0/EssentialsX.jar".into(),
            format: PluginFormat::Jar,
            sha256: None,
        });
        all_plugins.push(PluginEntry {
            name: "WorldGuard".into(),
            version: "7.0.107".into(),
            description: "Land protection plugin".into(),
            author: "Sk89q".into(),
            download_url: "https://github.com/Sk89q/WorldGuard/releases/download/7.0.107/WorldGuard-7.0.107.jar".into(),
            format: PluginFormat::Jar,
            sha256: None,
        });
        all_plugins.push(PluginEntry {
            name: "Vault".into(),
            version: "1.7.3".into(),
            description: "Economy API".into(),
            author: "MikeTheMoose".into(),
            download_url: "https://github.com/MikeTheMoose/Vault/releases/download/1.7.3/Vault.jar".into(),
            format: PluginFormat::Jar,
            sha256: None,
        });

        let query_lower = query.to_lowercase();
        let filtered: Vec<PluginEntry> = all_plugins.into_iter()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower) ||
                p.description.to_lowercase().contains(&query_lower)
            })
            .collect();

        Ok(filtered)
    }

    pub async fn download_plugin(&self, entry: &PluginEntry, server_id: &str) -> Result<PathBuf, String> {
        let plugin_dir = self.cache_dir.join("plugins").join(server_id);
        std::fs::create_dir_all(&plugin_dir).map_err(|e| e.to_string())?;

        let filename = format!("{}-{}.jar", entry.name, entry.version);
        let dest = plugin_dir.join(&filename);

        // Download using curl (available on Android via termux or bundled)
        let output = std::process::Command::new("curl")
            .args(["-sSL", "-o"])
            .arg(&dest)
            .arg(&entry.download_url)
            .output()
            .map_err(|e| format!("Download failed: {}", e))?;

        if !output.status.success() {
            return Err(format!("Download failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(dest)
    }
}
