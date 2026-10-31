use eframe::egui::{Ui, Vec2, Align2, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, DragValue, TextEdit};
use crate::lan::LanServer;
use crate::app::AppMessage;
use crate::ui::LanState;
use super::{card_frame, primary_button, PRIMARY, ON_SURFACE, ON_SURFACE_VARIANT, ERROR};

pub fn render_lan_tab(ui: &mut Ui, lan: &mut LanState, messages: &mut Vec<AppMessage>) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("Local Network").size(20.0).strong());
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            if primary_button(ui, "🔍 Discover").clicked() {
                messages.push(AppMessage::DiscoverLan);
            }
        });
    });
    ui.add_space(8.0);

    card_frame().show(ui, |ui| {
        ui.strong("Announce This Server");
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("Server Name:");
            ui.add_space(8.0);
            ui.add(TextEdit::singleline(&mut lan.announce_name).desired_width(200.0));
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("Port:");
            ui.add_space(8.0);
            ui.add(DragValue::new(&mut lan.announce_port).speed(1.0).range(1024..=65535));
        });
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.checkbox(&mut lan.announce_enabled, "Announce on LAN");
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if primary_button(ui, if lan.announce_enabled { "Stop Announcing" } else { "Start Announcing" }).clicked() {
                    if lan.announce_enabled {
                        // Stop announcing - could send a message to stop
                    } else {
                        messages.push(AppMessage::AnnounceLan { name: lan.announce_name.clone(), port: lan.announce_port });
                    }
                }
            });
        });
    });
    ui.add_space(16.0);

    ui.horizontal(|ui| {
        ui.strong(RichText::new("Discovered Servers").size(18.0));
        if lan.discovering {
            ui.add_space(8.0);
            ui.spinner();
        }
    });
    ui.add_space(8.0);

    if let Some(err) = &lan.error {
        ui.colored_label(ERROR, err);
        ui.add_space(8.0);
    }

    if lan.servers.is_empty() {
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
            for server in &lan.servers {
                render_lan_server_card(ui, server, messages);
            }
        });
    }
}

pub fn render_lan_server_card(ui: &mut Ui, server: &LanServer, messages: &mut Vec<AppMessage>) {
    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            let (rect, _response) = ui.allocate_exact_size(Vec2::splat(56.0), Sense::hover());
            let center = rect.center();
            ui.painter().circle_filled(center, 28.0, PRIMARY.linear_multiply(0.15));
            ui.painter().circle_stroke(center, 28.0, Stroke::new(2.0, PRIMARY));
            ui.painter().text(
                center,
                Align2::CENTER_CENTER,
                "📡",
                FontId::new(24.0, FontFamily::Proportional),
                PRIMARY,
            );
            
            ui.add_space(12.0);
            
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