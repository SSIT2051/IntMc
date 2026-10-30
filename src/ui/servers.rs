use eframe::egui::{Color32, Ui, Vec2, Align2, Rounding, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextStyle, TextEdit};
use crate::storage::ServerMetadata;
use crate::app::AppMessage;
use crate::server::ServerBackendType;
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, status_chip, ServerStatus, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};

pub fn render_servers_tab(ui: &mut Ui, servers: &[ServerMetadata], messages: &mut Vec<AppMessage>) {
    // Header with create button
    ui.horizontal(|ui| {
        ui.strong(RichText::new("My Servers").size(20.0));
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            if primary_button(ui, "+ Create Server").clicked() {
                messages.push(AppMessage::ShowCreateServerDialog);
            }
        });
    });
    ui.add_space(8.0);

    if servers.is_empty() {
        // Empty state
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                ui.label(RichText::new("🖥").size(56.0).color(ON_SURFACE_VARIANT));
                ui.add_space(16.0);
                ui.label(RichText::new("No Servers Yet").size(20.0).color(ON_SURFACE).strong());
                ui.add_space(8.0);
                ui.label(RichText::new("Create your first Minecraft server to get started").size(14.0).color(ON_SURFACE_VARIANT));
                ui.add_space(24.0);
                if primary_button(ui, "Create Server").clicked() {
                    messages.push(AppMessage::ShowCreateServerDialog);
                }
            });
        });
    } else {
        // Server list
        ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            for server in servers {
                render_server_card(ui, server, messages);
            }
        });
    }
}

pub fn render_server_card(ui: &mut Ui, server: &ServerMetadata, messages: &mut Vec<AppMessage>) {
    let server_id = server.id.clone();
    let is_running = server.is_running;
    
    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            // Server icon/avatar
            let backend_color = match server.backend.as_str() {
                "Pumpkin" => Color32::from_rgb(100, 180, 255),
                "Steel" => Color32::from_rgb(180, 200, 100),
                "Rosewater" => Color32::from_rgb(255, 160, 100),
                "Glowstone" => Color32::from_rgb(200, 100, 255),
                _ => Color32::from_rgb(150, 150, 150),
            };
            
            let rect = ui.allocate_exact_size(Vec2::splat(56.0), Sense::hover());
            ui.painter().circle_filled(rect.center(), 28.0, backend_color.linear_multiply(0.15));
            ui.painter().circle_stroke(rect.center(), 28.0, Stroke::new(2.0, backend_color));
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                match server.backend.as_str() {
                    "Pumpkin" => "🎃",
                    "Steel" => "⚙",
                    "Rosewater" => "🌹",
                    "Glowstone" => "✨",
                    _ => "📦",
                },
                FontId::new(24.0, FontFamily::Proportional),
                backend_color,
            );
            
            ui.add_space(12.0);
            
            // Server info
            ui.vertical(|ui| {
                ui.label(RichText::new(&server.name).size(16.0).color(ON_SURFACE).strong());
                ui.add_space(2.0);
                
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&server.backend).size(12.0).color(ON_SURFACE_VARIANT));
                    ui.add_space(8.0);
                    ui.label(RichText::new(format!("v{}", server.version)).size(12.0).color(ON_SURFACE_VARIANT));
                    ui.add_space(8.0);
                    ui.label(RichText::new(format!("Port: {}", server.port)).size(12.0).color(ON_SURFACE_VARIANT));
                });
                
                ui.add_space(4.0);
                
                // Status row
                ui.horizontal(|ui| {
                    status_chip(ui, 
                        if server.is_running { "Running" } else { "Stopped" }, 
                        ServerStatus::from_bool(server.is_running)
                    );
                    
                    if !server.ip.is_empty() {
                        ui.add_space(12.0);
                        ui.label(RichText::new(format!("🌐 {}", server.ip)).size(12.0).color(PRIMARY));
                    }
                });
            });
            
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if is_running {
                    if danger_button(ui, "Stop").clicked() {
                        messages.push(AppMessage::StopServer { server_id: server_id.clone() });
                    }
                    ui.add_space(8.0);
                    if secondary_button(ui, "Restart").clicked() {
                        messages.push(AppMessage::RestartServer { server_id: server_id.clone() });
                    }
                } else {
                    if primary_button(ui, "Start").clicked() {
                        messages.push(AppMessage::StartServer { server_id: server_id.clone() });
                    }
                }
                ui.add_space(8.0);
                if outlined_button(ui, "Settings").clicked() {
                    messages.push(AppMessage::OpenServerDetail { server_id: server_id.clone() });
                }
                ui.add_space(8.0);
                if text_button(ui, "Delete").clicked() {
                    messages.push(AppMessage::DeleteServer { server_id: server_id.clone() });
                }
            });
        });
    });
    ui.add_space(8.0);
}

fn version_badge(ui: &mut Ui, version: &str) {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(60.0, 20.0), Sense::hover());
    ui.painter().rect_filled(rect, 4.0, PRIMARY_CONTAINER);
    ui.painter().rect_stroke(rect, 4.0, Stroke::new(1.0, PRIMARY), egui::epaint::StrokeKind::Continuous);
    ui.painter().text(rect.center(), Align2::CENTER_CENTER, 
        RichText::new(version).size(9.0).color(PRIMARY).strong(), FontId::default(), PRIMARY);
}