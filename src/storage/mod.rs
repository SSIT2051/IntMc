use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMetadata {
    pub id: String,
    pub name: String,
    pub backend: String,
    pub version: String,
    pub path: PathBuf,
    pub port: u16,
    pub ip: String,
    pub created_at: u64,
    pub last_started: Option<u64>,
    pub is_running: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub version: String,
    pub source: String,
    pub format: PluginFormat,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginFormat {
    Wasm,
    Jar,
}

pub struct Storage {
    conn: Connection,
}

impl Storage {
    pub fn open<P: Into<PathBuf>>(db_path: P) -> Result<Self> {
        let path = db_path.into();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        }
        let conn = Connection::open(path)?;
        Self::migrate(&conn)?;
        Ok(Storage { conn })
    }

    fn migrate(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS servers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                backend TEXT NOT NULL,
                version TEXT NOT NULL,
                path TEXT NOT NULL,
                port INTEGER NOT NULL,
                ip TEXT NOT NULL DEFAULT '',
                created_at INTEGER NOT NULL,
                last_started INTEGER,
                is_running INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS plugins (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                server_id TEXT NOT NULL,
                name TEXT NOT NULL,
                version TEXT NOT NULL,
                source TEXT NOT NULL,
                format TEXT NOT NULL,
                enabled INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY(server_id) REFERENCES servers(id)
            );
        ",
        )?;
        Ok(())
    }

    pub fn insert_server(&self, server: &ServerMetadata) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO servers (id, name, backend, version, path, port, ip, created_at, last_started, is_running)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![
                server.id,
                server.name,
                server.backend,
                server.version,
                server.path.to_string_lossy(),
                server.port,
                server.ip,
                server.created_at,
                server.last_started,
                server.is_running as i32
            ],
        )?;
        Ok(())
    }

    pub fn get_all_servers(&self) -> Result<Vec<ServerMetadata>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, backend, version, path, port, ip, created_at, last_started, is_running FROM servers ORDER BY created_at DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(ServerMetadata {
                id: row.get(0)?,
                name: row.get(1)?,
                backend: row.get(2)?,
                version: row.get(3)?,
                path: PathBuf::from(row.get::<_, String>(4)?),
                port: row.get(5)?,
                ip: row.get(6)?,
                created_at: row.get(7)?,
                last_started: row.get(8)?,
                is_running: row.get::<_, i32>(9)? != 0,
            })
        })?;
        let mut servers = Vec::new();
        for row in rows {
            servers.push(row?);
        }
        Ok(servers)
    }

    pub fn delete_server(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM servers WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn update_running(&self, id: &str, is_running: bool) -> Result<()> {
        self.conn.execute(
            "UPDATE servers SET is_running = ?1 WHERE id = ?2",
            rusqlite::params![is_running as i32, id],
        )?;
        Ok(())
    }

    pub fn update_server_ip(&self, id: &str, ip: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE servers SET ip = ?1 WHERE id = ?2",
            rusqlite::params![ip, id],
        )?;
        Ok(())
    }

    pub fn insert_plugin(&self, server_id: &str, plugin: &PluginInfo) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO plugins (server_id, name, version, source, format, enabled)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                server_id,
                plugin.name,
                plugin.version,
                plugin.source,
                match plugin.format {
                    PluginFormat::Wasm => "wasm",
                    PluginFormat::Jar => "jar",
                },
                plugin.enabled as i32
            ],
        )?;
        Ok(())
    }

    pub fn get_plugins(&self, server_id: &str) -> Result<Vec<PluginInfo>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, version, source, format, enabled FROM plugins WHERE server_id = ?1"
        )?;
        let rows = stmt.query_map([server_id], |row| {
            Ok(PluginInfo {
                name: row.get(0)?,
                version: row.get(1)?,
                source: row.get(2)?,
                format: match row.get::<_, String>(3)?.as_str() {
                    "wasm" => PluginFormat::Wasm,
                    _ => PluginFormat::Jar,
                },
                enabled: row.get::<_, i32>(4)? != 0,
            })
        })?;
        let mut plugins = Vec::new();
        for row in rows {
            plugins.push(row?);
        }
        Ok(plugins)
    }
}
