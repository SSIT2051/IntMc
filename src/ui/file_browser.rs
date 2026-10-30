use eframe::egui::{Color32, Ui, Vec2, Align2, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextEdit, ComboBox, DragValue, TextStyle};
use crate::storage::{PluginInfo, PluginFormat};
use crate::app::AppMessage;
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
}

pub fn render_file_browser(
    ui: &mut Ui,
    current_path: &mut PathBuf,
    selected_file: &mut Option<PathBuf>,
    on_view: impl FnMut(&PathBuf),
    on_navigate: impl FnMut(&PathBuf),
) {
    card_frame().show(ui, |ui| {
        // Path bar
        ui.horizontal(|ui| {
            ui.label(RichText::new("Path:").size(12.0).color(ON_SURFACE_VARIANT));
            ui.add_space(8.0);
            ui.label(RichText::new(current_path.to_string_lossy().to_string()).size(12.0).color(ON_SURFACE).monospace());
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("⬆ Up").clicked() {
                    if let Some(parent) = current_path.parent() {
                        *current_path = parent.to_path_buf();
                    }
                }
            });
        });
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);
        
        // File list
        ScrollArea::vertical().auto_shrink([false, false]).max_height(300.0).show(ui, |ui| {
            let entries = read_dir(current_path).unwrap_or_default();
            for entry in entries {
                let is_selected = selected_file.as_ref() == Some(&entry.path);
                let response = ui.selectable_label(is_selected, format!("{} {}", 
                    if entry.is_dir { "📁" } else { "📄" }, 
                    entry.name
                ));
                if response.clicked() {
                    if entry.is_dir {
                        on_navigate(&entry.path);
                    } else {
                        *selected_file = Some(entry.path.clone());
                        on_view(&entry.path);
                    }
                }
            }
        });
    });
}

fn read_dir(path: &PathBuf) -> std::io::Result<Vec<FileEntry>> {
    let mut entries = Vec::new();
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        entries.push(FileEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path(),
            is_dir: metadata.is_dir(),
            size: metadata.len(),
        });
    }
    entries.sort_by(|a, b| {
        // Directories first, then alphabetical
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });
    Ok(entries)
}

pub fn render_file_viewer(ui: &mut Ui, path: &PathBuf, content: &mut String) {
    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.strong(format!("Editing: {}", path.file_name().unwrap_or_default().to_string_lossy()));
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if primary_button(ui, "Save").clicked() {
                    let content_to_write = content.clone();
                    let _ = std::fs::write(path, content_to_write);
                }
            });
        });
        ui.add_space(8.0);
        ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            ui.add(
                TextEdit::multiline(content)
                    .font(TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .desired_rows(20)
            );
        });
    });
}

pub fn render_file_browser_tab(ui: &mut Ui, _storage: &std::sync::Arc<tokio::sync::Mutex<crate::storage::Storage>>, _messages: &mut Vec<AppMessage>) {
    ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        ui.strong(RichText::new("📁").size(24.0));
        ui.add_space(8.0);
        
        ui.horizontal(|ui| {
            ui.label(RichText::new("Directory").size(16.0).color(ON_SURFACE));
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if outlined_button(ui, "Browse...").clicked() {
                }
            });
        });
        ui.add_space(16.0);
        
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            ui.label(RichText::new("No files found").size(16.0).color(ON_SURFACE_VARIANT));
            ui.add_space(16.0);
        });
        
        ui.add_space(16.0);
        
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            ui.label(RichText::new("Server Files").size(16.0).color(ON_SURFACE));
            ui.add_space(16.0);
        });
        
        ui.add_space(24.0);
        
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            let mut new_server_name = String::new();
            ui.add(
                egui::TextEdit::singleline(&mut new_server_name)
                    .hint_text("New server name...")
                    .desired_width(f32::INFINITY)
            );
            ui.add_space(8.0);
            if primary_button(ui, "Create Server").clicked() {
            }
            ui.add_space(16.0);
        });
        
        ui.add_space(32.0);
        
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Server directory: /data/data/rust.intmc/servers/").size(14.0).color(ON_SURFACE_VARIANT).monospace());
                ui.add_space(8.0);
                ui.label(RichText::new("No servers configured yet. Create a new server to get started.").size(14.0).color(ON_SURFACE_VARIANT));
            });
        });
    });
}