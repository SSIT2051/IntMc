use slint::{ModelRc, VecModel, SharedString};
use std::sync::Arc;

use crate::app::{AppState, PluginEntry};
use crate::storage::{ServerMetadata, PluginInfo, PluginFormat};
use crate::lan::LanServer;
use crate::server::ServerBackendType;

slint::include_modules!();

// ---------------------------------------------------------------------------
// Conversion helpers: Rust domain types -> Slint row types
// ---------------------------------------------------------------------------

fn server_to_row(server: &ServerMetadata) -> ServerRow {
    ServerRow {
        id: SharedString::from(&server.id),
        name: SharedString::from(&server.name),
        backend: SharedString::from(&server.backend),
        version: SharedString::from(&server.version),
        port: server.port as i32,
        ip: SharedString::from(&server.ip),
        is_running: server.is_running,
    }
}

fn lan_to_row(server: &LanServer) -> LanServerRow {
    LanServerRow {
        name: SharedString::from(&server.name),
        ip: SharedString::from(&server.ip),
        port: server.port as i32,
        version: SharedString::from(&server.version),
        motd: SharedString::from(&server.motd),
        players_online: server.players_online as i32,
        max_players: server.max_players as i32,
    }
}

fn plugin_to_row(plugin: &PluginInfo) -> PluginRow {
    PluginRow {
        name: SharedString::from(&plugin.name),
        version: SharedString::from(&plugin.version),
        enabled: plugin.enabled,
    }
}

fn entry_to_row(entry: &PluginEntry) -> PluginEntryRow {
    PluginEntryRow {
        name: SharedString::from(&entry.name),
        version: SharedString::from(&entry.version),
        description: SharedString::from(&entry.description),
        author: SharedString::from(&entry.author),
        download_url: SharedString::from(&entry.download_url),
        format: match entry.format {
            PluginFormat::Wasm => SharedString::from("wasm"),
            PluginFormat::Jar => SharedString::from("jar"),
        },
    }
}

// ---------------------------------------------------------------------------
// Model refresh helpers
// ---------------------------------------------------------------------------

fn refresh_servers_model(window: &MainWindow, state: &AppState) {
    let servers = state.get_servers();
    let rows: Vec<ServerRow> = servers.iter().map(server_to_row).collect();
    let model = ModelRc::new(VecModel::from(rows));
    window.set_servers(model);
}

fn refresh_lan_model(window: &MainWindow, state: &AppState) {
    let servers = state.get_lan_servers();
    let rows: Vec<LanServerRow> = servers.iter().map(lan_to_row).collect();
    let model = ModelRc::new(VecModel::from(rows));
    window.set_lan_servers(model);
}

fn refresh_plugins_model(window: &MainWindow, state: &AppState, server_id: &str) {
    match state.get_plugins(server_id) {
        Ok(plugins) => {
            let rows: Vec<PluginRow> = plugins.iter().map(plugin_to_row).collect();
            let model = ModelRc::new(VecModel::from(rows));
            window.set_plugins(model);
        }
        Err(_) => {
            let model = ModelRc::new(VecModel::from(Vec::new()));
            window.set_plugins(model);
        }
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

pub fn create_main_window() -> Result<MainWindow, slint::PlatformError> {
    let main_window = MainWindow::new()?;
    Ok(main_window)
}

pub fn connect_messages(main_window: &MainWindow, app_state: Arc<AppState>) {
    // ---- Initial data load ----
    refresh_servers_model(main_window, &app_state);
    refresh_lan_model(main_window, &app_state);

    // ---- load-servers ----
    main_window.on_load_servers({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move || {
            state.load_servers();
            if let Some(window) = window_weak.clone().upgrade() {
                refresh_servers_model(&window, &state);
                window.set_status_message(SharedString::from("Servers refreshed"));
            }
        }
    });

    // ---- show-create-dialog ----
    main_window.on_show_create_dialog({
        let window_weak = main_window.as_weak();
        move || {
            if let Some(window) = window_weak.clone().upgrade() {
                window.set_current_screen(SharedString::from("create"));
                window.set_error_message(SharedString::from(""));
            }
        }
    });

    // ---- hide-create-dialog ----
    main_window.on_hide_create_dialog({
        let window_weak = main_window.as_weak();
        move || {
            if let Some(window) = window_weak.clone().upgrade() {
                window.set_current_screen(SharedString::from("home"));
            }
        }
    });

    // ---- create-server ----
    main_window.on_create_server({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move |name, backend, port| {
            let backend_type = match backend.as_str() {
                "Pumpkin" => ServerBackendType::Pumpkin,
                "Steel" => ServerBackendType::Steel,
                "Rosewater" => ServerBackendType::Rosewater,
                "Glowstone" => ServerBackendType::Glowstone,
                _ => {
                    if let Some(window) = window_weak.clone().upgrade() {
                        window.set_error_message(SharedString::from("Unknown backend"));
                    }
                    return;
                }
            };
            let state = state.clone();
            let window_weak = window_weak.clone();
            std::thread::spawn(move || {
                let result = state.create_server(name.to_string(), backend_type, port as u16);
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.clone().upgrade() {
                        match result {
                            Ok(()) => {
                                window.set_current_screen(SharedString::from("home"));
                                refresh_servers_model(&window, &state);
                                window.set_status_message(SharedString::from("Server created"));
                            }
                            Err(e) => {
                                window.set_error_message(SharedString::from(&e));
                            }
                        }
                    }
                }).unwrap();
            });
        }
    });

    // ---- start-server ----
    main_window.on_start_server({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move |server_id| {
            let state = state.clone();
            let window_weak = window_weak.clone();
            let sid = server_id.to_string();
            std::thread::spawn(move || {
                let result = state.start_server(&sid);
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.clone().upgrade() {
                        match result {
                            Ok(()) => {
                                refresh_servers_model(&window, &state);
                                refresh_plugins_model(&window, &state, &sid);
                                window.set_status_message(SharedString::from("Server started"));
                            }
                            Err(e) => {
                                window.set_error_message(SharedString::from(&e));
                            }
                        }
                    }
                }).unwrap();
            });
        }
    });

    // ---- stop-server ----
    main_window.on_stop_server({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move |server_id| {
            let state = state.clone();
            let window_weak = window_weak.clone();
            let sid = server_id.to_string();
            std::thread::spawn(move || {
                let result = state.stop_server(&sid);
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.clone().upgrade() {
                        match result {
                            Ok(()) => {
                                refresh_servers_model(&window, &state);
                                refresh_plugins_model(&window, &state, &sid);
                                window.set_status_message(SharedString::from("Server stopped"));
                            }
                            Err(e) => {
                                window.set_error_message(SharedString::from(&e));
                            }
                        }
                    }
                }).unwrap();
            });
        }
    });

    // ---- delete-server ----
    main_window.on_delete_server({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move |server_id| {
            let state = state.clone();
            let window_weak = window_weak.clone();
            let sid = server_id.to_string();
            std::thread::spawn(move || {
                let result = state.delete_server(&sid);
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.clone().upgrade() {
                        match result {
                            Ok(()) => {
                                refresh_servers_model(&window, &state);
                                window.set_status_message(SharedString::from("Server deleted"));
                            }
                            Err(e) => {
                                window.set_error_message(SharedString::from(&e));
                            }
                        }
                    }
                }).unwrap();
            });
        }
    });

    // ---- open-server-detail ----
    main_window.on_open_server_detail({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move |server_id| {
            if let Some(window) = window_weak.clone().upgrade() {
                window.set_selected_server_id(server_id.clone());
                window.set_current_screen(SharedString::from("detail"));
                refresh_plugins_model(&window, &state, &server_id);
                window.set_error_message(SharedString::from(""));
            }
        }
    });

    // ---- discover-lan ----
    main_window.on_discover_lan({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move || {
            let state = state.clone();
            let window_weak = window_weak.clone();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let result = rt.block_on(state.discover_lan_async());
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.clone().upgrade() {
                        match result {
                            Ok(_) => {
                                refresh_lan_model(&window, &state);
                                window.set_current_screen(SharedString::from("lan"));
                                window.set_status_message(SharedString::from("LAN scan complete"));
                            }
                            Err(e) => {
                                window.set_error_message(SharedString::from(&e));
                            }
                        }
                    }
                }).unwrap();
            });
        }
    });

    // ---- join-lan ----
    main_window.on_join_lan({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move |name, ip, port| {
            let state = state.clone();
            let window_weak = window_weak.clone();
            let sname = name.to_string();
            let _sip = ip.to_string();
            std::thread::spawn(move || {
                let backend = ServerBackendType::Pumpkin;
                let result = state.create_server(sname, backend, port as u16);
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.clone().upgrade() {
                        match result {
                            Ok(()) => {
                                window.set_current_screen(SharedString::from("home"));
                                refresh_servers_model(&window, &state);
                                window.set_status_message(SharedString::from("LAN server added"));
                            }
                            Err(e) => {
                                window.set_error_message(SharedString::from(&e));
                            }
                        }
                    }
                }).unwrap();
            });
        }
    });

    // ---- toggle-plugin ----
    main_window.on_toggle_plugin({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move |server_id, plugin_name| {
            let state = state.clone();
            let window_weak = window_weak.clone();
            let sid = server_id.to_string();
            let pname = plugin_name.to_string();
            std::thread::spawn(move || {
                let result = state.toggle_plugin(&sid, &pname);
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.clone().upgrade() {
                        match result {
                            Ok(()) => {
                                refresh_plugins_model(&window, &state, &sid);
                                window.set_status_message(SharedString::from("Plugin toggled"));
                            }
                            Err(e) => {
                                window.set_error_message(SharedString::from(&e));
                            }
                        }
                    }
                }).unwrap();
            });
        }
    });

    // ---- install-plugin ----
    main_window.on_install_plugin({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move |server_id, plugin_name| {
            let state = state.clone();
            let window_weak = window_weak.clone();
            let sid = server_id.to_string();
            let pname = plugin_name.to_string();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let result = rt.block_on(state.install_plugin_async(&sid, &pname));
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.clone().upgrade() {
                        match result {
                            Ok(()) => {
                                refresh_plugins_model(&window, &state, &sid);
                                window.set_status_message(SharedString::from("Plugin installed"));
                            }
                            Err(e) => {
                                window.set_error_message(SharedString::from(&e));
                            }
                        }
                    }
                }).unwrap();
            });
        }
    });

    // ---- refresh-marketplace ----
    main_window.on_refresh_marketplace({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move || {
            let state = state.clone();
            let window_weak = window_weak.clone();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let result = rt.block_on(state.refresh_marketplace_async());
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.clone().upgrade() {
                        match result {
                            Ok(plugins) => {
                                let rows: Vec<PluginEntryRow> = plugins.iter().map(entry_to_row).collect();
                                let model = ModelRc::new(VecModel::from(rows));
                                window.set_marketplace_plugins(model);
                                window.set_current_screen(SharedString::from("marketplace"));
                                window.set_status_message(SharedString::from("Marketplace refreshed"));
                            }
                            Err(e) => {
                                window.set_error_message(SharedString::from(&e));
                            }
                        }
                    }
                }).unwrap();
            });
        }
    });

    // ---- send-command ----
    main_window.on_send_command({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move |server_id, command| {
            let state = state.clone();
            let window_weak = window_weak.clone();
            let sid = server_id.to_string();
            let cmd = command.to_string();
            std::thread::spawn(move || {
                let result = state.send_command(&sid, &cmd);
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.clone().upgrade() {
                        match result {
                            Ok(()) => {
                                window.set_status_message(SharedString::from("Command sent"));
                            }
                            Err(e) => {
                                window.set_error_message(SharedString::from(&e));
                            }
                        }
                    }
                }).unwrap();
            });
        }
    });

    // ---- create-tunnel ----
    main_window.on_create_tunnel({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move |server_id, port| {
            let state = state.clone();
            let window_weak = window_weak.clone();
            let sid = server_id.to_string();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let result = rt.block_on(state.create_tunnel_async(&sid, port as u16));
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.clone().upgrade() {
                        match result {
                            Ok(()) => {
                                window.set_status_message(SharedString::from("Tunnel created"));
                            }
                            Err(e) => {
                                window.set_error_message(SharedString::from(&e));
                            }
                        }
                    }
                }).unwrap();
            });
        }
    });

    // ---- remove-tunnel ----
    main_window.on_remove_tunnel({
        let window_weak = main_window.as_weak();
        let state = app_state.clone();
        move |server_id| {
            let state = state.clone();
            let window_weak = window_weak.clone();
            let sid = server_id.to_string();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let result = rt.block_on(state.remove_tunnel_async(&sid));
                slint::invoke_from_event_loop(move || {
                    if let Some(window) = window_weak.clone().upgrade() {
                        match result {
                            Ok(()) => {
                                window.set_status_message(SharedString::from("Tunnel removed"));
                            }
                            Err(e) => {
                                window.set_error_message(SharedString::from(&e));
                            }
                        }
                    }
                }).unwrap();
            });
        }
    });

    // ---- save-settings ----
    main_window.on_save_settings({
        let window_weak = main_window.as_weak();
        move |_, _, _| {
            if let Some(window) = window_weak.clone().upgrade() {
                window.set_current_screen(SharedString::from("home"));
                window.set_status_message(SharedString::from("Settings saved"));
            }
        }
    });
}
