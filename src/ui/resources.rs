use eframe::egui::{Color32, Ui, Vec2, Align2, Rounding, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextStyle, plot::{Plot, PlotPoints, Line, PlotUi, Bar, BarChart, AxisHints}};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};

#[derive(Debug, Clone)]
pub struct ResourceMetrics {
    pub cpu_percent: f32,
    pub memory_used_mb: u64,
    pub memory_total_mb: u64,
    pub disk_used_gb: f64,
    pub disk_total_gb: f64,
    pub network_rx_mbps: f32,
    pub network_tx_mbps: f32,
    pub timestamp: Instant,
}

pub struct ResourceMonitor {
    pub history: VecDeque<ResourceMetrics>,
    pub max_history: usize,
    pub last_update: Option<Instant>,
    pub update_interval: Duration,
}

impl Default for ResourceMonitor {
    fn default() -> Self {
        Self {
            history: VecDeque::with_capacity(60),
            max_history: 60,
            last_update: None,
            update_interval: Duration::from_secs(2),
        }
    }
}

impl ResourceMonitor {
    pub fn update(&mut self, metrics: ResourceMetrics) {
        self.history.push_back(metrics);
        if self.history.len() > self.max_history {
            self.history.pop_front();
        }
        self.last_update = Some(Instant::now());
    }
    
    pub fn should_update(&self) -> bool {
        self.last_update.map_or(true, |last| last.elapsed() >= self.update_interval)
    }
    
    pub fn cpu_history(&self) -> PlotPoints {
        self.history.iter()
            .enumerate()
            .map(|(i, m)| [i as f64, m.cpu_percent as f64])
            .collect()
    }
    
    pub fn memory_history(&self) -> PlotPoints {
        self.history.iter()
            .enumerate()
            .map(|(i, m)| [i as f64, m.memory_used_mb as f64])
            .collect()
    }
    
    pub fn network_rx_history(&self) -> PlotPoints {
        self.history.iter()
            .enumerate()
            .map(|(i, m)| [i as f64, m.network_rx_mbps as f64])
            .collect()
    }
    
    pub fn network_tx_history(&self) -> PlotPoints {
        self.history.iter()
            .enumerate()
            .map(|(i, m)| [i as f64, m.network_tx_mbps as f64])
            .collect()
    }
    
    pub fn current(&self) -> Option<&ResourceMetrics> {
        self.history.back()
    }
}

pub fn render_resources_tab(ui: &mut Ui, monitor: &ResourceMonitor, server_id: &str) {
    let current = monitor.current();
    
    // Header
    ui.horizontal(|ui| {
        ui.strong(RichText::new(format!("📊 Resources - {}", server_id)).size(20.0));
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            if outlined_button(ui, "Refresh").clicked() {
                // Trigger refresh
            }
        });
    });
    ui.add_space(16.0);
    
    if let Some(metrics) = current {
        // Summary cards
        ui.horizontal(|ui| {
            render_metric_card(ui, "CPU Usage", &format!("{:.1}%", metrics.cpu_percent), 
                format!("{:.0}%", metrics.cpu_percent), PRIMARY, get_cpu_color(metrics.cpu_percent));
            
            ui.add_space(12.0);
            
            let mem_percent = if metrics.memory_total_mb > 0 {
                (metrics.memory_used_mb as f64 / metrics.memory_total_mb as f64) * 100.0
            } else { 0.0 };
            
            render_metric_card(ui, "Memory", 
                &format!("{} / {} MB", metrics.memory_used_mb, metrics.memory_total_mb),
                &format!("{:.0}%", mem_percent),
                Color32::from_rgb(180, 200, 100), get_mem_color(mem_percent as f32));
            
            ui.add_space(12.0);
            
            let disk_percent = if metrics.disk_total_gb > 0.0 {
                (metrics.disk_used_gb / metrics.disk_total_gb) * 100.0
            } else { 0.0 };
            
            render_metric_card(ui, "Disk", 
                &format!("{:.1} / {:.1} GB", metrics.disk_used_gb, metrics.disk_total_gb),
                &format!("{:.0}%", disk_percent),
                Color32::from_rgb(255, 160, 100), get_disk_color(disk_percent as f32));
        });
        ui.add_space(16.0);
        
        // Network
        ui.horizontal(|ui| {
            render_metric_card(ui, "Network ↓", 
                &format!("{:.2} MB/s", metrics.network_rx_mbps),
                "Download", Color32::from_rgb(100, 180, 255), PRIMARY);
            
            ui.add_space(12.0);
            
            render_metric_card(ui, "Network ↑", 
                &format!("{:.2} MB/s", metrics.network_tx_mbps),
                "Upload", Color32::from_rgb(200, 100, 255), Color32::from_rgb(200, 100, 255));
        });
        ui.add_space(16.0);
        
        // Charts
        render_charts(ui, monitor);
    } else {
        // No data yet
        card_frame().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label(RichText::new("📊").size(48.0).color(ON_SURFACE_VARIANT));
                ui.add_space(16.0);
                ui.label(RichText::new("No resource data available").size(16.0).color(ON_SURFACE));
                ui.add_space(8.0);
                ui.label(RichText::new("Start the server to see resource usage").color(ON_SURFACE_VARIANT));
                ui.add_space(40.0);
            });
        });
    }
}

fn render_metric_card(ui: &mut Ui, title: &str, value: &str, subtitle: &str, icon_color: Color32, value_color: Color32) {
    let available_width = (ui.available_width() - 24.0) / 3.0;
    
    card_frame().show(ui, |ui| {
        ui.set_min_width(available_width);
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                let rect = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover()).rect;
                ui.painter().circle_filled(rect.center(), 16.0, icon_color.linear_multiply(0.15));
                ui.painter().circle_stroke(rect.center(), 16.0, Stroke::new(2.0, icon_color));
                ui.painter().text(rect.center(), Align2::CENTER_CENTER, 
                    match title { "CPU Usage" => "⚡", "Memory" => "💾", "Disk" => "💿", "Network ↓" => "⬇", _ => "⬆" },
                    FontId::new(16.0, FontFamily::Proportional), icon_color);
            });
            ui.add_space(8.0);
            ui.label(RichText::new(title).size(12.0).color(ON_SURFACE_VARIANT));
            ui.add_space(4.0);
            ui.label(RichText::new(value).size(20.0).color(value_color).strong());
            ui.add_space(2.0);
            ui.label(RichText::new(subtitle).size(11.0).color(ON_SURFACE_VARIANT));
        });
    });
}

fn get_cpu_color(percent: f32) -> Color32 {
    if percent > 90.0 { ERROR }
    else if percent > 70.0 { WARNING }
    else { SUCCESS }
}

fn get_mem_color(percent: f32) -> Color32 {
    if percent > 90.0 { ERROR }
    else if percent > 75.0 { WARNING }
    else { Color32::from_rgb(180, 200, 100) }
}

fn get_disk_color(percent: f32) -> Color32 {
    if percent > 95.0 { ERROR }
    else if percent > 85.0 { WARNING }
    else { Color32::from_rgb(255, 160, 100) }
}

fn render_charts(ui: &mut Ui, monitor: &ResourceMonitor) {
    let history_len = monitor.history.len();
    if history_len < 2 {
        return;
    }
    
    // CPU Chart
    card_frame().show(ui, |ui| {
        ui.strong(RichText::new("CPU Usage History").size(14.0));
        ui.add_space(8.0);
        
        let points: PlotPoints = monitor.cpu_history();
        
        Plot::new("cpu_chart")
            .height(150.0)
            .show_axes([true, true])
            .show_grid([true, true])
            .x_axis_hints(AxisHints::new().label("Time"))
            .y_axis_hints(AxisHints::new().label("%").min(0.0).max(100.0))
            .show(ui, |plot_ui: &mut PlotUi| {
                let line = Line::new(points)
                    .color(PRIMARY)
                    .width(2.0)
                    .fill(0.0);
                plot_ui.line(line);
            });
    });
    ui.add_space(12.0);
    
    // Memory Chart
    card_frame().show(ui, |ui| {
        ui.strong(RichText::new("Memory Usage History").size(14.0));
        ui.add_space(8.0);
        
        let points: PlotPoints = monitor.memory_history();
        let max_mem = monitor.current().map(|m| m.memory_total_mb as f64).unwrap_or(1024.0);
        
        Plot::new("memory_chart")
            .height(150.0)
            .show_axes([true, true])
            .show_grid([true, true])
            .x_axis_hints(AxisHints::new().label("Time"))
            .y_axis_hints(AxisHints::new().label("MB").min(0.0).max(max_mem))
            .show(ui, |plot_ui: &mut PlotUi| {
                let line = Line::new(points)
                    .color(Color32::from_rgb(180, 200, 100))
                    .width(2.0)
                    .fill(0.0);
                plot_ui.line(line);
            });
    });
    ui.add_space(12.0);
    
    // Network Chart
    card_frame().show(ui, |ui| {
        ui.strong(RichText::new("Network History").size(14.0));
        ui.add_space(8.0);
        
        let rx_points: PlotPoints = monitor.network_rx_history();
        let tx_points: PlotPoints = monitor.network_tx_history();
        
        Plot::new("network_chart")
            .height(150.0)
            .show_axes([true, true])
            .show_grid([true, true])
            .x_axis_hints(AxisHints::new().label("Time"))
            .y_axis_hints(AxisHints::new().label("MB/s").min(0.0))
            .show(ui, |plot_ui: &mut PlotUi| {
                let rx_line = Line::new(rx_points)
                    .color(PRIMARY)
                    .width(2.0)
                    .name("Download");
                let tx_line = Line::new(tx_points)
                    .color(Color32::from_rgb(200, 100, 255))
                    .width(2.0)
                    .name("Upload");
                plot_ui.line(rx_line);
                plot_ui.line(tx_line);
            });
    });
}

pub fn collect_system_metrics(server_path: &std::path::Path) -> ResourceMetrics {
    // Collect actual system metrics
    // This is a simplified version - in production you'd use sysinfo or similar
    
    let cpu_percent = get_cpu_usage();
    let (mem_used, mem_total) = get_memory_usage();
    let (disk_used, disk_total) = get_disk_usage(server_path);
    let (net_rx, net_tx) = get_network_usage();
    
    ResourceMetrics {
        cpu_percent,
        memory_used_mb: mem_used,
        memory_total_mb: mem_total,
        disk_used_gb: disk_used,
        disk_total_gb: disk_total,
        network_rx_mbps: net_rx,
        network_tx_mbps: net_tx,
        timestamp: Instant::now(),
    }
}

fn get_cpu_usage() -> f32 {
    // Simplified - in reality use sysinfo crate
    // For now return a simulated value
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as f32;
    (secs.sin() * 20.0 + 30.0).max(0.0).min(100.0)
}

fn get_memory_usage() -> (u64, u64) {
    // Simplified - in reality read from /proc/meminfo or use sysinfo
    (512, 2048)
}

fn get_disk_usage(path: &std::path::Path) -> (f64, f64) {
    // Simplified - in reality use std::fs::metadata or similar
    let used = 2.5;
    let total = 64.0;
    (used, total)
}

fn get_network_usage() -> (f32, f32) {
    // Simplified - in reality track network interface stats
    (0.1, 0.05)
}