use eframe::egui::{Color32, Ui, Vec2, Align2, Rounding, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextStyle, TextEdit, ComboBox, TextEdit};
use crate::storage::{PluginInfo, PluginFormat};
use crate::plugins::PluginEntry;
use crate::app::AppMessage;
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};

pub fn render_marketplace_tab(ui: &mut Ui, _storage: &std::sync::Arc<tokio::sync::Mutex<crate::storage::Storage>>, _playit: &crate::playit::PlayitManager, _messages: &mut Vec<AppMessage>) {
    // Search and filter bar
    ui.horizontal(|ui| {
        ui.add_space(4.0);
        let search_response = ui.add(
            TextEdit::singleline(&mut String::new())
                .hint_text("Search plugins...")
                .desired_width(300.0)
        );
        if search_response.changed() {
            // Trigger search
        }
        
        ui.add_space(8.0);
        
        ComboBox::from_label("")
            .selected_text("All Formats")
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut None, None, "All Formats");
                ui.selectable_value(&mut Some(PluginFormat::Wasm), Some(PluginFormat::Wasm), "WASM (Pumpkin)");
                ui.selectable_value(&mut Some(PluginFormat::Jar), Some(PluginFormat::Jar), "JAR (Steel/Spigot)");
            });
    });
    ui.add_space(8.0);

    // Categories
    ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal(|ui| {
            let categories = ["All", "Essentials", "World", "Economy", "Chat", "Admin", "Fun", "Security"];
            for (i, cat) in categories.iter().enumerate() {
                let selected = i == 0; // placeholder
                if chip(ui, cat, selected, PRIMARY).clicked() {
                    // Filter by category
                }
                ui.add_space(8.0);
            }
        });
    });
    ui.add_space(16.0);

    // Plugin grid/list
    ui.label(RichText::new("Featured Plugins").size(18.0).strong());
    ui.add_space(8.0);

    // Sample plugin cards
    let plugins = vec![
        PluginEntry {
            name: "EssentialsX".to_string(),
            version: "2.195.0".to_string(),
            description: "Essential commands for Minecraft servers (/home, /spawn, /tpa, kits, warps, and more)".to_string(),
            author: "EssentialsX Team".to_string(),
            download_url: "https://github.com/EssentialsX/Essentials/releases/download/2.195.0/EssentialsX.jar".to_string(),
            format: PluginFormat::Wasm,
            sha256: None,
        },
        PluginEntry {
            name: "WorldGuard".to_string(),
            version: "7.0.107".to_string(),
            description: "Region protection plugin - protect your builds, prevent griefing, manage regions".to_string(),
            author: "Sk89q".to_string(),
            download_url: "https://github.com/Sk89q/WorldGuard/releases/download/7.0.107/WorldGuard-7.0.107.jar".to_string(),
            format: PluginFormat::Wasm,
            sha256: None,
        },
        PluginEntry {
            name: "LuckPerms".to_string(),
            version: "5.4.105".to_string(),
            description: "Advanced permissions plugin with web editor, verbose debugging, and flexible inheritance".to_string(),
            author: "Luck".to_string(),
            download_url: "https://github.com/LuckPerms/LuckPerms/releases/download/5.4.105/LuckPerms-Bukkit-5.4.105.jar".to_string(),
            format: PluginFormat::Wasm,
            sha256: None,
        },
        PluginEntry {
            name: "Vault".to_string(),
            version: "1.7.3".to_string(),
            description: "Economy and permissions API for other plugins to hook into".to_string(),
            author: "MikeTheMoose".to_string(),
            download_url: "https://github.com/MikeTheMoose/Vault/releases/download/1.7.3/Vault.jar".to_string(),
            format: PluginFormat::Jar,
            sha256: None,
        },
        PluginEntry {
            name: "PlaceholderAPI".to_string(),
            version: "2.3.0".to_string(),
            description: "Placeholder expansion API - use placeholders in any plugin that supports it".to_string(),
            author: "PlaceholderAPI".to_string(),
            download_url: "https://github.com/PlaceholderAPI/PlaceholderAPI/releases/download/2.3.0/PlaceholderAPI.jar".to_string(),
            format: PluginFormat::Jar,
            sha256: None,
        },
        PluginEntry {
            name: "CoreProtect".to_string(),
            version: "21.1".to_string(),
            description: "Block logging and rollback plugin - investigate griefing, rollback changes".to_string(),
            author: "Intellivoid".to_string(),
            download_url: "https://github.com/Intellivoid/CoreProtect/releases/download/v21.1/CoreProtect.jar".to_string(),
            format: PluginFormat::Jar,
            sha256: None,
        },
    ];

    ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        for plugin in &plugins {
            render_plugin_card(ui, plugin);
        }
    });
}

pub fn render_plugin_card(ui: &mut Ui, plugin: &PluginEntry) {
    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            // Plugin icon
            let format_color = match plugin.format {
                PluginFormat::Wasm => Color32::from_rgb(100, 180, 255),
                PluginFormat::Jar => Color32::from_rgb(180, 200, 100),
            };
            
            let rect = ui.allocate_exact_size(Vec2::splat(56.0), Sense::hover()).rect;
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
            
            // Plugin info
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
                if primary_button(ui, "Install").clicked() {
                    // Handle install
                }
                ui.add_space(8.0);
                if outlined_button(ui, "Details").clicked() {
                    // Show details
                }
            });
        });
    });
    ui.add_space(8.0);
}