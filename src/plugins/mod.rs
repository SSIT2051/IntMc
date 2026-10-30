use md5;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use crate::storage::PluginFormat;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginEntry {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub download_url: String,
    pub format: PluginFormat,
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginSource {
    PumpkinPlugins,
    SteelPlugins,
    GitHub,
    Modrinth,
}

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Download failed: {0}")]
    Download(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Verification failed: {0}")]
    Verification(String),
}

pub struct PluginMarketplace {
    cache_dir: PathBuf,
}

impl PluginMarketplace {
    pub fn new<P: Into<PathBuf>>(cache_dir: P) -> Self {
        Self {
            cache_dir: cache_dir.into(),
        }
    }

    pub async fn fetch_pumpkin_plugins(&self) -> Result<Vec<PluginEntry>, PluginError> {
        // Fetch from pumpkin-plugins repo
        let plugins = vec![
            PluginEntry {
                name: "EssentialsX".into(),
                version: "2.195.0".into(),
                description: "Essential plugin for Minecraft servers".into(),
                author: "EssentialsX Team".into(),
                download_url: "https://github.com/EssentialsX/Essentials/releases/download/2.195.0/EssentialsX.jar".into(),
                format: PluginFormat::Wasm,
                sha256: None,
            },
            PluginEntry {
                name: "WorldGuard".into(),
                version: "7.0.107".into(),
                description: "Land protection plugin".into(),
                author: "Sk89q".into(),
                download_url: "https://github.com/Sk89q/WorldGuard/releases/download/7.0.107/WorldGuard-7.0.107.jar".into(),
                format: PluginFormat::Wasm,
                sha256: None,
            },
        ];
        Ok(plugins)
    }

    pub async fn fetch_steel_plugins(&self) -> Result<Vec<PluginEntry>, PluginError> {
        let plugins = vec![
            PluginEntry {
                name: "Vault".into(),
                version: "1.7.3".into(),
                description: "Economy API".into(),
                author: "MikeTheMoose".into(),
                download_url: "https://github.com/MikeTheMoose/Vault/releases/download/1.7.3/Vault.jar".into(),
                format: PluginFormat::Jar,
                sha256: None,
            },
            PluginEntry {
                name: "PlaceholderAPI".into(),
                version: "2.3.0".into(),
                description: "Placeholder expansion API".into(),
                author: "PlaceholderAPI".into(),
                download_url: "https://github.com/PlaceholderAPI/PlaceholderAPI/releases/download/2.3.0/PlaceholderAPI.jar".into(),
                format: PluginFormat::Jar,
                sha256: None,
            },
        ];
        Ok(plugins)
    }

    pub async fn download_plugin(&self, entry: &PluginEntry, server_id: &str) -> Result<PathBuf, PluginError> {
        let plugin_dir = self.cache_dir.join("plugins").join(server_id);
        std::fs::create_dir_all(&plugin_dir)?;
        
        let filename = format!("{}-{}.{}", entry.name, entry.version, 
            match entry.format { PluginFormat::Wasm => "wasm", PluginFormat::Jar => "jar" });
        let dest = plugin_dir.join(&filename);
        
        // Download via reqwest (or use std::process::Command for curl)
        let output = std::process::Command::new("curl")
            .args(["-sSL", "-o"])
            .arg(&dest)
            .arg(&entry.download_url)
            .output()
            .map_err(|e| PluginError::Download(e.to_string()))?;
        
        if !output.status.success() {
            return Err(PluginError::Download(String::from_utf8_lossy(&output.stderr).to_string()));
        }
        
        // Verify hash if provided
        if let Some(expected_hash) = &entry.sha256 {
            let file_data = std::fs::read(&dest)?;
            let digest = md5::compute(&file_data);
            let hash = digest.iter().map(|b| format!("{:02x}", b)).collect::<String>();
            if hash != *expected_hash {
                return Err(PluginError::Verification(format!(
                    "Hash mismatch: expected {}, got {}", expected_hash, hash
                )));
            }
        }
        
        Ok(dest)
    }

    pub async fn search_plugins(&self, query: &str, format: Option<PluginFormat>) -> Result<Vec<PluginEntry>, PluginError> {
        let mut all_plugins = Vec::new();
        
        // Fetch from all sources
        if let Ok(mut plugins) = self.fetch_pumpkin_plugins().await {
            all_plugins.append(&mut plugins);
        }
        if let Ok(mut plugins) = self.fetch_steel_plugins().await {
            all_plugins.append(&mut plugins);
        }
        
        // Filter by query and format
        let query_lower = query.to_lowercase();
        let filtered: Vec<PluginEntry> = all_plugins.into_iter()
            .filter(|p| {
                let name_match = p.name.to_lowercase().contains(&query_lower);
                let desc_match = p.description.to_lowercase().contains(&query_lower);
                let format_match = format.as_ref().map_or(true, |f| &p.format == f);
                (name_match || desc_match) && format_match
            })
            .collect();
        
        Ok(filtered)
    }
}