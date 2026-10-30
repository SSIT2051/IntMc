use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelInfo {
    pub server_id: String,
    pub local_port: u16,
    pub public_url: String,
    pub public_port: u16,
    pub process_id: Option<u32>,
}

#[derive(Debug, thiserror::Error)]
pub enum PlayitError {
    #[error("Playit binary not found")]
    BinaryNotFound,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Tunnel creation failed: {0}")]
    CreationFailed(String),
    #[error("Tunnel not found for server: {0}")]
    NotFound(String),
}

#[derive(Debug, Clone)]
pub struct PlayitManager {
    binary_path: Option<String>,
    tunnels: Arc<Mutex<Vec<TunnelInfo>>>,
}

impl PlayitManager {
    pub fn new() -> Self {
        // Try to find playit binary in PATH or bundled location
        let binary_path = Self::find_playit_binary();
        Self {
            binary_path,
            tunnels: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn find_playit_binary() -> Option<String> {
        // Check common locations
        let candidates = [
            "playit",
            "/usr/local/bin/playit",
            "/opt/playit/playit",
            "./playit",
        ];
        for path in candidates {
            if Command::new(path).arg("--version").output().is_ok() {
                return Some(path.to_string());
            }
        }
        None
    }

    pub async fn create_tunnel(&self, server_id: &str, local_port: u16) -> Result<TunnelInfo, PlayitError> {
        let binary = self.binary_path.as_ref()
            .ok_or(PlayitError::BinaryNotFound)?;

        // Start playit with auto tunnel for this port
        let output = Command::new(binary)
            .args(["tunnel", "tcp", &format!(":{local_port}")])
            .output()
            .map_err(|e| PlayitError::CreationFailed(e.to_string()))?;

        // Parse output for public URL/port
        let stdout = String::from_utf8_lossy(&output.stdout);
        let (public_url, public_port) = Self::parse_tunnel_output(&stdout)
            .ok_or_else(|| PlayitError::CreationFailed("Failed to parse tunnel output".into()))?;

        let tunnel = TunnelInfo {
            server_id: server_id.to_string(),
            local_port,
            public_url,
            public_port,
            process_id: None, // We don't track the process in this simple implementation
        };

        self.tunnels.lock().await.push(tunnel.clone());
        Ok(tunnel)
    }

    pub async fn remove_tunnel(&self, server_id: &str) -> Result<(), PlayitError> {
        let mut tunnels = self.tunnels.lock().await;
        tunnels.retain(|t| t.server_id != server_id);
        Ok(())
    }

    pub async fn get_tunnel(&self, server_id: &str) -> Option<TunnelInfo> {
        self.tunnels.lock().await.iter()
            .find(|t| t.server_id == server_id)
            .cloned()
    }

    fn parse_tunnel_output(output: &str) -> Option<(String, u16)> {
        // playit output format: "tcp://your-domain.playit.gg:12345 -> localhost:25565"
        for line in output.lines() {
            if line.contains("->") {
                let parts: Vec<&str> = line.split("->").map(|s| s.trim()).collect();
                if parts.len() == 2 {
                    let public = parts[0].trim();
                    if let Some(url_port) = public.strip_prefix("tcp://") {
                        let port = url_port.split(':').last()?.parse().ok()?;
                        return Some((url_port.to_string(), port));
                    }
                }
            }
        }
        None
    }

    pub async fn is_available(&self) -> bool {
        self.binary_path.is_some()
    }
}

impl Default for PlayitManager {
    fn default() -> Self {
        Self::new()
    }
}