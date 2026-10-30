use eframe::egui::{Color32, Ui, Vec2, Align2, Rounding, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextStyle, TextEdit};
use std::path::{Path, PathBuf};
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<std::time::SystemTime>,
}

pub fn render_file_browser(
    ui: &mut Ui, 
    current_path: &mut PathBuf, 
    selected_file: &mut Option<PathBuf>,
    on_file_click: impl Fn(&PathBuf),
    on_folder_navigate: impl Fn(&PathBuf),
) {
    // Path breadcrumb
    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("📁").size(16.0).color(PRIMARY));
            ui.add_space(8.0);
            
            let components: Vec<_> = current_path.components().collect();
            for (i, comp) in components.iter().enumerate() {
                if i > 0 {
                    ui.label(RichText::new("/").size(13.0).color(ON_SURFACE_VARIANT));
                }
                let part_path: PathBuf = components[..=i].iter().collect();
                let name = comp.as_os_str().to_string_lossy();
                if text_button(ui, &name).clicked() {
                    on_folder_navigate(&part_path);
                }
            }
            
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if outlined_button(ui, "↑ Up").clicked() {
                    if let Some(parent) = current_path.parent() {
                        on_folder_navigate(parent.to_path_buf());
                    }
                }
            });
        });
    });
    ui.add_space(8.0);

    // File list
    let entries = read_directory(current_path);
    
    ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        if entries.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(40.0);
                    ui.label(RichText::new("📂").size(48.0).color(ON_SURFACE_VARIANT));
                    ui.add_space(16.0);
                    ui.label(RichText::new("This folder is empty").size(16.0).color(ON_SURFACE));
                });
            });
        } else {
            for entry in &entries {
                render_file_entry(ui, entry, selected_file, on_file_click, on_folder_navigate);
            }
        }
    });
}

fn read_directory(path: &Path) -> Vec<FileEntry> {
    let mut entries = Vec::new();
    
    if let Ok(read_dir) = std::fs::read_dir(path) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            let is_dir = path.is_dir();
            let (size, modified) = if let Ok(metadata) = entry.metadata() {
                (metadata.len(), metadata.modified().ok())
            } else {
                (0, None)
            };
            
            entries.push(FileEntry {
                path,
                name,
                is_dir,
                size,
                modified,
            });
        }
    }
    
    // Sort: directories first, then alphabetically
    entries.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });
    
    entries
}

fn render_file_entry(
    ui: &mut Ui, 
    entry: &FileEntry, 
    selected_file: &mut Option<PathBuf>,
    on_file_click: impl Fn(&PathBuf),
    on_folder_navigate: impl Fn(&PathBuf),
) {
    let selected = selected_file.as_ref() == Some(&entry.path);
    
    let frame = if selected {
        elevated_card_frame()
    } else {
        card_frame()
    };
    
    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            // Icon
            let (icon, color) = if entry.is_dir {
                ("📁", PRIMARY)
            } else if entry.name.ends_with(".jar") {
                ("☕", Color32::from_rgb(180, 200, 100))
            } else if entry.name.ends_with(".wasm") {
                ("⚡", Color32::from_rgb(100, 180, 255))
            } else if entry.name.ends_with(".yml") || entry.name.ends_with(".yaml") {
                ("📄", Color32::from_rgb(255, 160, 100))
            } else if entry.name.ends_with(".log") || entry.name.ends_with(".txt") {
                ("📝", Color32::from_rgb(150, 150, 150))
            } else if entry.name.ends_with(".json") {
                ("📋", Color32::from_rgb(200, 100, 255))
            } else if entry.name.ends_with(".properties") {
                ("⚙", Color32::from_rgb(200, 200, 100))
            } else {
                ("📄", ON_SURFACE_VARIANT)
            };
            
            let rect = ui.allocate_exact_size(Vec2::splat(40.0), Sense::hover()).rect;
            ui.painter().text(
                rect.center(),
                Align2::CENTER_CENTER,
                icon,
                FontId::new(20.0, FontFamily::Proportional),
                color,
            );
            
            ui.add_space(8.0);
            
            // Name and info
            ui.vertical(|ui| {
                let name_text = if entry.is_dir {
                    RichText::new(&entry.name).size(14.0).color(ON_SURFACE).strong()
                } else {
                    RichText::new(&entry.name).size(14.0).color(ON_SURFACE)
                };
                ui.label(name_text);
                
                if !entry.is_dir {
                    ui.label(RichText::new(format_size(entry.size)).size(11.0).color(ON_SURFACE_VARIANT));
                }
            });
            
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if entry.is_dir {
                    if icon_button(ui, "→", false).on_hover_text("Open folder").clicked() {
                        on_folder_navigate(&entry.path);
                    }
                } else {
                    if icon_button(ui, "👁", false).on_hover_text("View file").clicked() {
                        on_file_click(&entry.path);
                    }
                    ui.add_space(4.0);
                    if icon_button(ui, "📋", false).on_hover_text("Copy path").clicked() {
                        ui.ctx().copy_text(entry.path.to_string_lossy().to_string());
                    }
                }
            });
        });
    });
    
    // Handle click on the whole row
    // This would need a response from the frame
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    
    if unit_idx == 0 {
        format!("{} {}", bytes, UNITS[unit_idx])
    } else {
        format!("{:.1} {}", size, UNITS[unit_idx])
    }
}

pub fn render_file_viewer(ui: &mut Ui, path: &Path, content: &str) {
    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new(format!("📄 {}", path.file_name().unwrap_or_default().to_string_lossy())).size(16.0).strong());
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if outlined_button(ui, "Copy All").clicked() {
                    ui.ctx().copy_text(content.to_string());
                }
                ui.add_space(8.0);
                if outlined_button(ui, "Close").clicked() {
                    // Close viewer
                }
            });
        });
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);
        
        ScrollArea::both().auto_shrink([false, false]).show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut content.to_string())
                    .font(FontId::new(12.0, FontFamily::Monospace))
                    .desired_rows(30)
                    .desired_width(f32::INFINITY)
                    .interactive(false)
            );
        });
    });
}