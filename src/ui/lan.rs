use eframe::egui::{Color32, Ui, Vec2, Align2, Rounding, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextStyle, TextEdit};
use crate::lan::LanServer;
use crate::app::AppMessage;
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};

pub fn render_lan_tab(ui: &mut Ui, _storage: &std::sync::Arc<tokio::sync::Mutex<crate::storage::Storage>>, _playit: &crate::playit::PlayitManager, messages: &mut Vec<AppMessage>) {
    // Header with discover button
    ui.horizontal(|ui| {
        ui.strong(RichText::new("Local Network").size(20.0));
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            if primary_button(ui, "🔍 Discover").clicked() {
                messages.push(AppMessage::DiscoverLan);
            }
        });
    });
    ui.add_space(8.0);

    // Announce section
    card_frame().show(ui, |ui| {
        ui.strong("Announce This Server");
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("Server Name:");
            ui.add_space(8.0);
            ui.text_edit_singleline(&mut String::from("intmc Server"));
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("Port:");
            ui.add_space(8.0);
            ui.add(egui::DragValue::new(&mut 25565u16).speed(1.0));
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            let mut enabled = false;
            ui.checkbox(&mut enabled, "Announce on LAN");
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if primary_button(ui, "Start Announcing").clicked() {
                    // Start LAN announcement
                }
            });
        });
    });
    ui.add_space(16.0);

    // Discovered servers
    ui.horizontal(|ui| {
        ui.strong(RichText::new("Discovered Servers").size(18.0));
    });
    ui.add_space(8.0);

    // Sample discovered servers
    let discovered = vec![
        LanServer {
            name: "Survival World".to_string(),
            ip: "192.168.1.45".to_string(),
            port: 25565,
            version: "1.21.1".to_string(),
            motd: "Welcome to our survival server!".to_string(),
            players_online: 3,
            max_players: 20,
        },
        LanServer {
            name: "Creative Build".to_string(),
            ip: "192.168.1.45".to_string(),
            port: 25566,
            version: "1.21".to_string(),
            motd: "Creative mode - build anything!".to_string(),
            players_online: 1,
            max_players: 10,
        },
    ];

    if discovered.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label(RichText::new("📡").size(48.0).color(ON_SURFACE_VARIANT));
                ui.add_space(16.0);
                ui.label(RichText::new("No LAN servers found").size(18.0).color(ON_SURFACE));
                ui.add_space(8.0);
                ui.label(RichText::new("Tap \"Discover\" to search for servers on your network").color(ON_SURFACE_VARIANT));
            });
        });
    } else {
        ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            for server in &discovered {
                render_lan_server_card(ui, server, messages);
            }
        });
    }
}

pub fn render_lan_server_card(ui: &mut Ui, server: &LanServer, messages: &mut Vec<AppMessage>) {
    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            // Server icon
            let rect = ui.allocate_exact_size(Vec2::splat(56.0), Sense::hover()).rect;
            ui.painter().circle_filled(rect.center(), 28.0, PRIMARY.linear_multiply(0.15));
            ui.painter().circle_stroke(rect.center(), 28.0, Stroke::new(2.0, PRIMARY));
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                "📡",
                FontId::new(24.0, FontFamily::Proportional),
                PRIMARY,
            );
            
            ui.add_space(12.0);
            
            // Server info
            ui.vertical(|ui| {
                ui.label(RichText::new(&server.name).size(16.0).color(ON_SURFACE).strong());
                ui.add_space(2.0);
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&server.motd).size(12.0).color(ON_SURFACE_VARIANT));
                });
                
                ui.add_space(4.0);
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("v{}", server.version)).size(12.0).color(ON_SURFACE_VARIANT));
                    ui.add_space(12.0);
                    ui.label(RichText::new(format!("👥 {}/{}", server.players_online, server.max_players)).size(12.0).color(ON_SURFACE_VARIANT));
                    ui.add_space(12.0);
                    ui.label(RichText::new(format!("{} : {}", server.ip, server.port)).size(12.0).color(PRIMARY));
                });
            });
            
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                if primary_button(ui, "Join").clicked() {
                    messages.push(AppMessage::JoinLanServer {
                        name: server.name.clone(),
                        ip: server.ip.clone(),
                        port: server.port,
                    });
                }
            });
        });
    });
    ui.add_space(8.0);
}