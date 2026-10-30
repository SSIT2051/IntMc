use eframe::egui::{Color32, Ui, Vec2, Align2, Rounding, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextStyle, TextEdit};
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};

pub struct LogViewer {
    pub logs: Vec<LogEntry>,
    pub filter: String,
    pub follow: bool,
    pub max_lines: usize,
    pub level_filter: LogLevel,
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: std::time::SystemTime,
    pub level: LogLevel,
    pub message: String,
    pub raw: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn color(&self) -> Color32 {
        match self {
            LogLevel::Trace => Color32::from_rgb(150, 150, 150),
            LogLevel::Debug => Color32::from_rgb(100, 180, 255),
            LogLevel::Info => Color32::from_rgb(80, 220, 120),
            LogLevel::Warn => Color32::from_rgb(255, 200, 50),
            LogLevel::Error => Color32::from_rgb(255, 80, 80),
        }
    }
    
    pub fn icon(&self) -> &'static str {
        match self {
            LogLevel::Trace => "🔍",
            LogLevel::Debug => "🐛",
            LogLevel::Info => "ℹ",
            LogLevel::Warn => "⚠",
            LogLevel::Error => "✗",
        }
    }
    
    pub fn from_str(s: &str) -> Option<Self> {
        let s = s.to_uppercase();
        if s.contains("ERROR") || s.contains("ERR") { Some(LogLevel::Error) }
        else if s.contains("WARN") { Some(LogLevel::Warn) }
        else if s.contains("INFO") { Some(LogLevel::Info) }
        else if s.contains("DEBUG") { Some(LogLevel::Debug) }
        else if s.contains("TRACE") { Some(LogLevel::Trace) }
        else { Some(LogLevel::Info) }
    }
}

impl Default for LogViewer {
    fn default() -> Self {
        Self {
            logs: Vec::new(),
            filter: String::new(),
            follow: true,
            max_lines: 1000,
            level_filter: LogLevel::Info,
        }
    }
}

impl LogViewer {
    pub fn add_log(&mut self, raw: String) {
        let level = LogLevel::from_str(&raw).unwrap_or(LogLevel::Info);
        let entry = LogEntry {
            timestamp: std::time::SystemTime::now(),
            level,
            message: raw.clone(),
            raw,
        };
        
        self.logs.push(entry);
        
        // Trim to max lines
        if self.logs.len() > self.max_lines {
            self.logs.drain(0..self.logs.len() - self.max_lines);
        }
    }
    
    pub fn filtered_logs(&self) -> Vec<&LogEntry> {
        self.logs.iter()
            .filter(|e| {
                let level_match = match self.level_filter {
                    LogLevel::Trace => true,
                    LogLevel::Debug => matches!(e.level, LogLevel::Debug | LogLevel::Info | LogLevel::Warn | LogLevel::Error),
                    LogLevel::Info => matches!(e.level, LogLevel::Info | LogLevel::Warn | LogLevel::Error),
                    LogLevel::Warn => matches!(e.level, LogLevel::Warn | LogLevel::Error),
                    LogLevel::Error => matches!(e.level, LogLevel::Error),
                };
                let filter_match = self.filter.is_empty() || 
                    e.message.to_lowercase().contains(&self.filter.to_lowercase());
                level_match && filter_match
            })
            .collect()
    }
}

pub fn render_log_viewer(ui: &mut Ui, viewer: &mut LogViewer) {
    // Toolbar
    card_frame().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Console").size(16.0).strong());
            
            ui.add_space(16.0);
            
            // Filter input
            ui.label(RichText::new("🔍").size(14.0).color(ON_SURFACE_VARIANT));
            ui.add(TextEdit::singleline(&mut viewer.filter).hint_text("Filter...").desired_width(200.0));
            
            ui.add_space(16.0);
            
            // Level filter
            ComboBox::from_label("")
                .selected_text(format!("{:?}", viewer.level_filter))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut viewer.level_filter, LogLevel::Trace, "Trace");
                    ui.selectable_value(&mut viewer.level_filter, LogLevel::Debug, "Debug");
                    ui.selectable_value(&mut viewer.level_filter, LogLevel::Info, "Info");
                    ui.selectable_value(&mut viewer.level_filter, LogLevel::Warn, "Warn");
                    ui.selectable_value(&mut viewer.level_filter, LogLevel::Error, "Error");
                });
            
            ui.add_space(16.0);
            
            // Follow toggle
            let mut follow = viewer.follow;
            ui.checkbox(&mut follow, "Follow");
            viewer.follow = follow;
            
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if outlined_button(ui, "Clear").clicked() {
                    viewer.logs.clear();
                }
                ui.add_space(8.0);
                if outlined_button(ui, "Copy").clicked() {
                    let text: String = viewer.filtered_logs().iter()
                        .map(|e| format!("[{:?}] {}", e.level, e.message))
                        .collect::<Vec<_>>()
                        .join("\n");
                    ui.ctx().copy_text(text);
                }
            });
        });
    });
    ui.add_space(8.0);
    
    // Log content
    elevated_card_frame().show(ui, |ui| {
        let available_height = ui.available_height();
        
        ScrollArea::vertical()
            .auto_shrink([false, false])
            .max_height(available_height)
            .show(ui, |ui| {
                let filtered = viewer.filtered_logs();
                
                for entry in filtered {
                    render_log_entry(ui, entry);
                }
                
                // Auto-scroll to bottom if following
                if viewer.follow && !filtered.is_empty() {
                    ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                }
            });
    });
}

fn render_log_entry(ui: &mut Ui, entry: &LogEntry) {
    let time_str = format_time(entry.timestamp);
    
    ui.horizontal(|ui| {
        // Timestamp
        ui.label(RichText::new(time_str).size(11.0).color(ON_SURFACE_VARIANT).monospace());
        ui.add_space(8.0);
        
        // Level icon
        ui.label(RichText::new(entry.level.icon()).size(12.0).color(entry.level.color()));
        ui.add_space(4.0);
        
        // Message
        let message = highlight_ansi(&entry.message);
        ui.label(message.size(12.0).color(ON_SURFACE).monospace());
    });
}

fn format_time(time: std::time::SystemTime) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();
    
    // Convert to local time (simplified)
    let hours = (secs / 3600) % 24;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    
    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

fn highlight_ansi(text: &str) -> RichText {
    // Simple ANSI color code stripping and highlighting
    let mut rich = RichText::new(text).monospace();
    
    // Highlight common patterns
    if text.contains("ERROR") || text.contains("Error") || text.contains("error") {
        rich = rich.color(ERROR);
    } else if text.contains("WARN") || text.contains("Warn") || text.contains("warn") {
        rich = rich.color(WARNING);
    } else if text.contains("INFO") || text.contains("Info") || text.contains("info") {
        rich = rich.color(SUCCESS);
    }
    
    rich
}

pub fn parse_server_log_line(line: &str) -> Option<LogEntry> {
    // Parse common Minecraft server log formats
    // [HH:MM:SS] [LEVEL] message
    // [HH:MM:SS] [Server thread/LEVEL]: message
    
    if line.len() < 10 {
        return None;
    }
    
    // Try to extract timestamp
    if line.starts_with('[') {
        if let Some(end) = line.find(']') {
            let time_part = &line[1..end];
            let rest = &line[end + 1..].trim_start();
            
            // Try to extract level
            let level = if rest.starts_with('[') {
                if let Some(level_end) = rest.find(']') {
                    let level_str = &rest[1..level_end];
                    LogLevel::from_str(level_str).unwrap_or(LogLevel::Info)
                } else {
                    LogLevel::Info
                }
            } else {
                LogLevel::Info
            };
            
            return Some(LogEntry {
                timestamp: std::time::SystemTime::now(),
                level,
                message: line.to_string(),
                raw: line.to_string(),
            });
        }
    }
    
    None
}