use eframe::egui::{Color32, Ui, Vec2, Align2, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextEdit, ComboBox};
use crate::storage::{PluginFormat};
use crate::plugins::PluginEntry;
use crate::app::AppMessage;
use crate::ui::MarketplaceState;
use super::{card_frame, primary_button, outlined_button, chip, PRIMARY, ON_SURFACE, ON_SURFACE_VARIANT, render_empty_state, render_loading_state, render_error_state};

pub fn render_marketplace_tab(ui: &mut Ui, marketplace: &mut MarketplaceState, messages: &mut Vec<AppMessage>) {
    ui.horizontal(|ui| {
        ui.add_space(4.0);
        let search_response = ui.add(
            TextEdit::singleline(&mut marketplace.search_query)
                .hint_text("Search plugins...")
                .desired_width(300.0)
        );
        if search_response.changed() {
        }
        
        ui.add_space(8.0);
        
        let mut selected_format = marketplace.selected_format.clone();
        ComboBox::from_label("")
            .selected_text(match selected_format {
                None => "All Formats",
                Some(PluginFormat::Wasm) => "WASM (Pumpkin)",
                Some(PluginFormat::Jar) => "JAR (Steel/Spigot)",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut selected_format, None, "All Formats");
                ui.selectable_value(&mut selected_format, Some(PluginFormat::Wasm), "WASM (Pumpkin)");
                ui.selectable_value(&mut selected_format, Some(PluginFormat::Jar), "JAR (Steel/Spigot)");
            });
        marketplace.selected_format = selected_format;
    });
    ui.add_space(8.0);

    ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal(|ui| {
            let categories = ["All", "Essentials", "World", "Economy", "Chat", "Admin", "Fun", "Security"];
            for (i, _cat) in categories.iter().enumerate() {
                let cat = categories[i];
                let selected = marketplace.selected_category.as_ref() == Some(&cat.to_string());
                if chip(ui, cat, selected, PRIMARY).clicked() {
                    marketplace.selected_category = Some(cat.to_string());
                }
                ui.add_space(8.0);
            }
        });
    });
    ui.add_space(16.0);

    ui.horizontal(|ui| {
        ui.label(RichText::new("Available Plugins").size(18.0).strong());
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            if marketplace.loading {
                ui.spinner();
            } else if outlined_button(ui, "🔄 Refresh").clicked() {
                messages.push(AppMessage::RefreshMarketplace);
            }
        });
    });
    ui.add_space(8.0);

    if marketplace.loading {
        render_loading_state(ui, "Loading plugins...");
    } else if let Some(err) = &marketplace.error {
        render_error_state(ui, err, Some(|| {}));
    } else if marketplace.filtered_plugins.is_empty() {
        render_empty_state(ui, "📦", "No plugins found", "Try adjusting your search or filters", None, None::<fn()>);
    } else {
        ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            for plugin in &marketplace.filtered_plugins {
                let installing = marketplace.installing.get(&plugin.name).copied().unwrap_or(false);
                render_plugin_card(ui, plugin, marketplace.server_id.as_deref(), installing, messages);
            }
        });
    }
}

pub fn render_plugin_card(ui: &mut Ui, plugin: &PluginEntry, server_id: Option<&str>, installing: bool, messages: &mut Vec<AppMessage>) {
    let plugin_name = plugin.name.clone();
    let server_id = server_id.map(|s| s.to_string());
    
    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            let format_color = match plugin.format {
                PluginFormat::Wasm => Color32::from_rgb(100, 180, 255),
                PluginFormat::Jar => Color32::from_rgb(180, 200, 100),
            };
            
            let (rect, _response) = ui.allocate_exact_size(Vec2::splat(56.0), Sense::hover());
            ui.painter().circle_filled(rect.center(), 28.0, format_color.linear_multiply(0.15));
            ui.painter().circle_stroke(rect.center(), 28.0, Stroke::new(2.0, format_color));
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                match plugin.format {
                    PluginFormat::Wasm => "⚡",
                    PluginFormat::Jar => "☕",
                },
                FontId::new(24.0, FontFamily::Proportional),
                format_color,
            );
            
            ui.add_space(12.0);
            
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&plugin.name).size(16.0).color(ON_SURFACE).strong());
                    ui.add_space(8.0);
                    ui.label(RichText::new(format!("v{}", plugin.version)).size(12.0).color(ON_SURFACE_VARIANT));
                });
                
                ui.add_space(2.0);
                ui.label(RichText::new(&plugin.description).size(13.0).color(ON_SURFACE_VARIANT));
                
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("by {}", plugin.author)).size(11.0).color(ON_SURFACE_VARIANT));
                    ui.add_space(12.0);
                    let format_label = match plugin.format {
                        PluginFormat::Wasm => "WASM",
                        PluginFormat::Jar => "JAR",
                    };
                    ui.label(RichText::new(format_label).size(10.0).color(format_color).strong());
                });
            });
            
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                let install_text = if installing { "Installing..." } else { "Install" };
                if primary_button(ui, install_text).clicked() {
                    if let Some(sid) = server_id {
                        messages.push(AppMessage::InstallPlugin { 
                            server_id: sid, 
                            plugin_name: plugin_name.clone(), 
                            plugin_url: plugin.download_url.clone() 
                        });
                    }
                }
                ui.add_space(8.0);
                if outlined_button(ui, "Details").clicked() {
                }
            });
        });
    });
    ui.add_space(8.0);
}