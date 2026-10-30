use std::net::UdpSocket;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanServer {
    pub name: String,
    pub ip: String,
    pub port: u16,
    pub version: String,
    pub motd: String,
    pub players_online: u32,
    pub max_players: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum LanError {
    #[error("Socket error: {0}")]
    Socket(String),
    #[error("Parse error: {0}")]
    Parse(String),
}

pub struct LanDiscovery {
    socket: Arc<Mutex<UdpSocket>>,
    broadcast_addr: String,
}

impl LanDiscovery {
    pub fn new() -> Result<Self, LanError> {
        let socket = UdpSocket::bind("0.0.0.0:19132")
            .map_err(|e| LanError::Socket(e.to_string()))?;
        socket.set_broadcast(true)
            .map_err(|e| LanError::Socket(e.to_string()))?;
        
        Ok(Self {
            socket: Arc::new(Mutex::new(socket)),
            broadcast_addr: "255.255.255.255:19132".to_string(),
        })
    }

    pub async fn discover(&self) -> Result<Vec<LanServer>, LanError> {
        let socket = self.socket.lock().await;
        let mut buf = [0u8; 2048];
        
        // Set non-blocking and try to receive discovery responses
        socket.set_nonblocking(true)
            .map_err(|e| LanError::Socket(e.to_string()))?;
        
        let mut servers = Vec::new();
        let timeout = std::time::Duration::from_millis(2000);
        let start = std::time::Instant::now();
        
        while start.elapsed() < timeout {
            match socket.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    if let Ok(server) = Self::parse_response(&buf[..size], addr) {
                        servers.push(server);
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
                    continue;
                }
                Err(e) => return Err(LanError::Socket(e.to_string())),
            }
        }
        
        Ok(servers)
    }

    pub async fn broadcast_announcement(&self, name: &str, port: u16) -> Result<(), LanError> {
        let socket = self.socket.lock().await;
        let packet = Self::build_announcement_packet(name, port);
        socket.send_to(&packet, &self.broadcast_addr)
            .map_err(|e| LanError::Socket(e.to_string()))?;
        Ok(())
    }

    fn parse_response(data: &[u8], addr: std::net::SocketAddr) -> Result<LanServer, LanError> {
        // Minecraft LAN discovery response format
        let data_str = String::from_utf8_lossy(data);
        
        // Try to parse as Minecraft ping response
        if let Ok(server) = Self::parse_minecraft_ping(&data_str, addr) {
            return Ok(server);
        }
        
        Err(LanError::Parse("Invalid response format".into()))
    }

    fn parse_minecraft_ping(data: &str, addr: std::net::SocketAddr) -> Result<LanServer, LanError> {
        // Minecraft LAN ping format: MOTD;Players;MaxPlayers;Version;ServerID
        let parts: Vec<&str> = data.split(';').collect();
        if parts.len() >= 5 {
            let motd = parts[0].replace('\u{00a7}', "§");
            let players_online = parts[1].parse().unwrap_or(0);
            let max_players = parts[2].parse().unwrap_or(0);
            let version = parts[3].to_string();
            
            Ok(LanServer {
                name: format!("Minecraft @ {}", addr.ip()),
                ip: addr.ip().to_string(),
                port: addr.port(),
                version,
                motd,
                players_online,
                max_players,
            })
        } else {
            Err(LanError::Parse("Not enough parts".into()))
        }
    }

    fn build_announcement_packet(name: &str, _port: u16) -> Vec<u8> {
        // Minecraft LAN announcement packet
        let mut packet = Vec::new();
        // Magic bytes
        packet.extend_from_slice(&[0xFE, 0xFD]);
        // Type: 0x09 for server list ping
        packet.push(0x09);
        // Padding
        packet.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);
        // MOTD;Players;MaxPlayers;Version;ServerID
        let payload = format!("{};0;20;1.21;1", name);
        packet.extend_from_slice(payload.as_bytes());
        packet
    }
}

impl Default for LanDiscovery {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback: create a socket that will fail gracefully
            LanDiscovery {
                socket: Arc::new(Mutex::new(
                    UdpSocket::bind("0.0.0.0:0").unwrap()
                )),
                broadcast_addr: "255.255.255.255:19132".to_string(),
            }
        })
    }
}