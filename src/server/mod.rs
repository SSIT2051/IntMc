use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Child;
use std::sync::Arc;
use tokio::sync::Mutex;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServerBackendType {
    Pumpkin,
    Steel,
    Rosewater,
    Glowstone,
}

impl std::fmt::Display for ServerBackendType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerBackendType::Pumpkin => write!(f, "Pumpkin"),
            ServerBackendType::Steel => write!(f, "Steel"),
            ServerBackendType::Rosewater => write!(f, "Rosewater"),
            ServerBackendType::Glowstone => write!(f, "Glowstone"),
        }
    }
}

impl ServerBackendType {
    pub fn to_string(&self) -> String {
        match self {
            ServerBackendType::Pumpkin => "Pumpkin".to_string(),
            ServerBackendType::Steel => "Steel".to_string(),
            ServerBackendType::Rosewater => "Rosewater".to_string(),
            ServerBackendType::Glowstone => "Glowstone".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub id: String,
    pub name: String,
    pub backend: ServerBackendType,
    pub version: String,
    pub path: PathBuf,
    pub port: u16,
    pub memory_mb: usize,
    pub auto_tunnel: bool,
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Backend not found: {0}")]
    BackendNotFound(String),
    #[error("Server already running")]
    AlreadyRunning,
    #[error("Server not running")]
    NotRunning,
    #[error("Start failed: {0}")]
    StartFailed(String),
    #[error("Stop failed: {0}")]
    StopFailed(String),
}

// Type alias for a process with captured stdout/stderr
pub type ServerProcess = Arc<Mutex<tokio::process::Child>>;

pub trait ServerBackend: Send + Sync {
    fn name(&self) -> &str;
    fn supported_versions(&self) -> Vec<&str>;
    fn binary_name(&self) -> &str;
    fn default_args(&self, config: &ServerConfig) -> Vec<String>;
    fn prepare(&self, config: &ServerConfig) -> Result<(), ServerError>;
    fn start(&self, config: &ServerConfig) -> Result<ServerProcess, ServerError>;
    fn stop(&self, process: &ServerProcess) -> Result<(), ServerError>;
    fn is_running(&self, process: &ServerProcess) -> bool;
    fn get_logs(&self, process: &ServerProcess) -> Vec<String>;
}

pub fn get_backend(backend: &ServerBackendType) -> Box<dyn ServerBackend + Send + Sync> {
    match backend {
        ServerBackendType::Pumpkin => Box::new(pumpkin::PumpkinBackend),
        ServerBackendType::Steel => Box::new(steel::SteelBackend),
        ServerBackendType::Rosewater => Box::new(rosewater::RosewaterBackend),
        ServerBackendType::Glowstone => Box::new(glowstone::GlowstoneBackend),
    }
}

pub mod pumpkin {
    use super::*;
    use tokio::process::Command;
    use tokio::io::{AsyncBufReadExt, BufReader};

    pub struct PumpkinBackend;

    impl ServerBackend for PumpkinBackend {
        fn name(&self) -> &str { "PumpkinMC" }
        fn supported_versions(&self) -> Vec<&str> { vec!["1.21", "1.21.1", "1.21.2"] }
        fn binary_name(&self) -> &str { "pumpkin" }
        fn default_args(&self, config: &ServerConfig) -> Vec<String> {
            vec![
                "--port".into(), config.port.to_string(),
                "--world".into(), config.path.join("world").to_string_lossy().into(),
            ]
        }
        fn prepare(&self, config: &ServerConfig) -> Result<(), ServerError> {
            std::fs::create_dir_all(config.path.join("world"))?;
            Ok(())
        }
        fn start(&self, config: &ServerConfig) -> Result<ServerProcess, ServerError> {
            let mut cmd = Command::new(self.binary_name());
            cmd.args(self.default_args(config));
            cmd.current_dir(&config.path);
            cmd.stdout(std::process::Stdio::piped());
            cmd.stderr(std::process::Stdio::piped());
            cmd.stdin(std::process::Stdio::piped());
            
            let child = cmd.spawn().map_err(|e| ServerError::StartFailed(e.to_string()))?;
            Ok(Arc::new(Mutex::new(child)))
        }
        fn stop(&self, process: &ServerProcess) -> Result<(), ServerError> {
            let mut child = process.blocking_lock();
            child.kill().map_err(|e| ServerError::StopFailed(e.to_string()))
        }
        fn is_running(&self, process: &ServerProcess) -> bool {
            let mut child = process.blocking_lock();
            child.try_wait().map(|opt| opt.is_none()).unwrap_or(false)
        }
        fn get_logs(&self, process: &ServerProcess) -> Vec<String> {
            // Note: For real-time logs, we'd need a separate log reader task
            // This is a placeholder - actual implementation would stream logs
            vec![]
        }
    }
}

pub mod steel {
    use super::*;
    use tokio::process::Command;

    pub struct SteelBackend;

    impl ServerBackend for SteelBackend {
        fn name(&self) -> &str { "SteelMC" }
        fn supported_versions(&self) -> Vec<&str> { vec!["1.12.2", "1.16.5", "1.18.2", "1.20.1"] }
        fn binary_name(&self) -> &str { "steel" }
        fn default_args(&self, config: &ServerConfig) -> Vec<String> {
            vec![
                "--port".into(), config.port.to_string(),
                "--world".into(), config.path.join("world").to_string_lossy().into(),
            ]
        }
        fn prepare(&self, config: &ServerConfig) -> Result<(), ServerError> {
            std::fs::create_dir_all(config.path.join("world"))?;
            Ok(())
        }
        fn start(&self, config: &ServerConfig) -> Result<ServerProcess, ServerError> {
            let mut cmd = Command::new(self.binary_name());
            cmd.args(self.default_args(config));
            cmd.current_dir(&config.path);
            cmd.stdout(std::process::Stdio::piped());
            cmd.stderr(std::process::Stdio::piped());
            cmd.stdin(std::process::Stdio::piped());
            
            let child = cmd.spawn().map_err(|e| ServerError::StartFailed(e.to_string()))?;
            Ok(Arc::new(Mutex::new(child)))
        }
        fn stop(&self, process: &ServerProcess) -> Result<(), ServerError> {
            let mut child = process.blocking_lock();
            child.kill().map_err(|e| ServerError::StopFailed(e.to_string()))
        }
        fn is_running(&self, process: &ServerProcess) -> bool {
            let mut child = process.blocking_lock();
            child.try_wait().map(|opt| opt.is_none()).unwrap_or(false)
        }
        fn get_logs(&self, process: &ServerProcess) -> Vec<String> {
            vec![]
        }
    }
}

pub mod rosewater {
    use super::*;
    use tokio::process::Command;

    pub struct RosewaterBackend;

    impl ServerBackend for RosewaterBackend {
        fn name(&self) -> &str { "Rosewater" }
        fn supported_versions(&self) -> Vec<&str> { vec!["1.21", "1.21.1"] }
        fn binary_name(&self) -> &str { "rosewater" }
        fn default_args(&self, config: &ServerConfig) -> Vec<String> {
            vec![
                "--port".into(), config.port.to_string(),
            ]
        }
        fn prepare(&self, config: &ServerConfig) -> Result<(), ServerError> {
            std::fs::create_dir_all(config.path.join("world"))?;
            Ok(())
        }
        fn start(&self, config: &ServerConfig) -> Result<ServerProcess, ServerError> {
            let mut cmd = Command::new(self.binary_name());
            cmd.args(self.default_args(config));
            cmd.current_dir(&config.path);
            cmd.stdout(std::process::Stdio::piped());
            cmd.stderr(std::process::Stdio::piped());
            cmd.stdin(std::process::Stdio::piped());
            
            let child = cmd.spawn().map_err(|e| ServerError::StartFailed(e.to_string()))?;
            Ok(Arc::new(Mutex::new(child)))
        }
        fn stop(&self, process: &ServerProcess) -> Result<(), ServerError> {
            let mut child = process.blocking_lock();
            child.kill().map_err(|e| ServerError::StopFailed(e.to_string()))
        }
        fn is_running(&self, process: &ServerProcess) -> bool {
            let mut child = process.blocking_lock();
            child.try_wait().map(|opt| opt.is_none()).unwrap_or(false)
        }
        fn get_logs(&self, process: &ServerProcess) -> Vec<String> {
            vec![]
        }
    }
}

pub mod glowstone {
    use super::*;
    use tokio::process::Command;

    pub struct GlowstoneBackend;

    impl ServerBackend for GlowstoneBackend {
        fn name(&self) -> &str { "Glowstone-rs" }
        fn supported_versions(&self) -> Vec<&str> { vec!["1.12.2", "1.16.5", "1.18.2"] }
        fn binary_name(&self) -> &str { "glowstone" }
        fn default_args(&self, config: &ServerConfig) -> Vec<String> {
            vec![
                "--port".into(), config.port.to_string(),
            ]
        }
        fn prepare(&self, config: &ServerConfig) -> Result<(), ServerError> {
            std::fs::create_dir_all(config.path.join("world"))?;
            Ok(())
        }
        fn start(&self, config: &ServerConfig) -> Result<ServerProcess, ServerError> {
            let mut cmd = Command::new(self.binary_name());
            cmd.args(self.default_args(config));
            cmd.current_dir(&config.path);
            cmd.stdout(std::process::Stdio::piped());
            cmd.stderr(std::process::Stdio::piped());
            cmd.stdin(std::process::Stdio::piped());
            
            let child = cmd.spawn().map_err(|e| ServerError::StartFailed(e.to_string()))?;
            Ok(Arc::new(Mutex::new(child)))
        }
        fn stop(&self, process: &ServerProcess) -> Result<(), ServerError> {
            let mut child = process.blocking_lock();
            child.kill().map_err(|e| ServerError::StopFailed(e.to_string()))
        }
        fn is_running(&self, process: &ServerProcess) -> bool {
            let mut child = process.blocking_lock();
            child.try_wait().map(|opt| opt.is_none()).unwrap_or(false)
        }
        fn get_logs(&self, process: &ServerProcess) -> Vec<String> {
            vec![]
        }
    }
}