use eframe::egui::{Color32, Ui, Vec2, Align2, Rounding, Stroke, RichText, FontId, FontFamily, ScrollArea, Layout, Sense, TextStyle, TextEdit, ComboBox, DragValue};
use crate::server::{ServerBackendType, get_backend};
use super::{card_frame, elevated_card_frame, primary_button, secondary_button, outlined_button, text_button, danger_button, icon_button, chip, PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};

#[derive(Debug, Clone)]
pub struct CreateServerData {
    pub name: String,
    pub backend: ServerBackendType,
    pub version: String,
    pub port: u16,
    pub memory_mb: usize,
    pub auto_tunnel: bool,
    pub selected_version_index: usize,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateServerStep {
    Basic = 0,
    Version = 1,
    Advanced = 2,
    Confirm = 3,
}

impl CreateServerStep {
    pub fn next(&self) -> Option<Self> {
        match self {
            CreateServerStep::Basic => Some(CreateServerStep::Version),
            CreateServerStep::Version => Some(CreateServerStep::Advanced),
            CreateServerStep::Advanced => Some(CreateServerStep::Confirm),
            CreateServerStep::Confirm => None,
        }
    }
    
    pub fn prev(&self) -> Option<Self> {
        match self {
            CreateServerStep::Basic => None,
            CreateServerStep::Version => Some(CreateServerStep::Basic),
            CreateServerStep::Advanced => Some(CreateServerStep::Version),
            CreateServerStep::Confirm => Some(CreateServerStep::Advanced),
        }
    }
    
    pub fn title(&self) -> &'static str {
        match self {
            CreateServerStep::Basic => "Basic Info",
            CreateServerStep::Version => "Version",
            CreateServerStep::Advanced => "Advanced",
            CreateServerStep::Confirm => "Confirm",
        }
    }
    
    pub fn index(&self) -> usize {
        *self as usize
    }
}

pub fn render_create_server_dialog(
    ctx: &egui::Context,
    state: &mut CreateServerData,
    should_close: &mut bool,
    messages: &mut Vec<crate::app::AppMessage>,
) {
    let mut step = CreateServerStep::Basic; // This would come from parent state
    
    egui::Window::new("Create New Server")
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ctx, |ui| {
            ui.set_min_width(480.0);
            
            // Progress indicator
            render_step_indicator(ui, step);
            ui.add_space(16.0);
            
            match step {
                CreateServerStep::Basic => render_basic_step(ui, state),
                CreateServerStep::Version => render_version_step(ui, state),
                CreateServerStep::Advanced => render_advanced_step(ui, state),
                CreateServerStep::Confirm => render_confirm_step(ui, state, should_close, messages),
            }
        });
}

fn render_step_indicator(ui: &mut Ui, current: CreateServerStep) {
    let steps = [
        CreateServerStep::Basic,
        CreateServerStep::Version,
        CreateServerStep::Advanced,
        CreateServerStep::Confirm,
    ];
    
    ui.horizontal(|ui| {
        for (i, step) in steps.iter().enumerate() {
            let is_active = *step == current;
            let is_completed = step.index() < current.index();
            
            if is_completed {
                // Completed step
                let rect = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover()).rect;
                ui.painter().circle_filled(rect.center(), 16.0, SUCCESS);
                ui.painter().text(rect.center(), Align2::CENTER_CENTER, "✓", 
                    FontId::new(14.0, FontFamily::Proportional), Color32::WHITE);
            } else if is_active {
                // Active step
                let rect = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover()).rect;
                ui.painter().circle_filled(rect.center(), 16.0, PRIMARY);
                ui.painter().text(rect.center(), Align2::CENTER_CENTER, 
                    format!("{}", step.index() + 1),
                    FontId::new(14.0, FontFamily::Proportional), Color32::WHITE);
            } else {
                // Inactive step
                let rect = ui.allocate_exact_size(Vec2::splat(32.0), Sense::hover()).rect;
                ui.painter().circle_stroke(rect.center(), 16.0, Stroke::new(2.0, OUTLINE));
                ui.painter().text(rect.center(), Align2::CENTER_CENTER, 
                    format!("{}", step.index() + 1),
                    FontId::new(14.0, FontFamily::Proportional), ON_SURFACE_VARIANT);
            }
            
            if i < steps.len() - 1 {
                // Connection line
                let line_rect = ui.allocate_exact_size(Vec2::new(40.0, 2.0), Sense::hover()).rect;
                let color = if step.index() < current.index() { SUCCESS } else { OUTLINE };
                ui.painter().rect_filled(line_rect, Rounding::same(1.0), color);
            }
        }
    });
    
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        for (i, step) in steps.iter().enumerate() {
            let is_active = *step == current;
            let width = 72.0;
            ui.allocate_ui(Vec2::new(width, 20.0), |ui| {
                ui.label(RichText::new(step.title())
                    .size(10.0)
                    .color(if is_active { PRIMARY } else { ON_SURFACE_VARIANT })
                    .strong());
            });
            if i < steps.len() - 1 {
                ui.add_space(40.0);
            }
        }
    });
}

fn render_basic_step(ui: &mut Ui, state: &mut CreateServerData) {
    ui.vertical(|ui| {
        ui.label(RichText::new("Server Name").size(14.0).color(ON_SURFACE).strong());
        ui.add_space(4.0);
        ui.add(TextEdit::singleline(&mut state.name)
            .hint_text("My Awesome Server")
            .desired_width(f32::INFINITY));
        ui.add_space(16.0);
        
        ui.label(RichText::new("Backend").size(14.0).color(ON_SURFACE).strong());
        ui.add_space(4.0);
        
        let backends = [
            (ServerBackendType::Pumpkin, "PumpkinMC", "🎃", "Modern Rust-based server, best for 1.21+"),
            (ServerBackendType::Steel, "SteelMC", "⚙", "High-performance Java-compatible, supports older versions"),
            (ServerBackendType::Rosewater, "Rosewater", "🌹", "Lightweight proxy/server hybrid"),
            (ServerBackendType::Glowstone, "Glowstone-rs", "✨", "Bukkit-compatible Rust implementation"),
        ];
        
        for (backend, name, icon, desc) in backends {
            let selected = state.backend == backend;
            
            let response = ui.allocate_response(Vec2::new(ui.available_width(), 64.0), Sense::click());
            let rect = response.rect;
            
            if selected {
                ui.painter().rect_filled(rect, Rounding::same(12.0), PRIMARY_CONTAINER);
                ui.painter().rect_stroke(rect, Rounding::same(12.0), Stroke::new(2.0, PRIMARY));
            } else {
                ui.painter().rect_filled(rect, Rounding::same(12.0), SURFACE_VARIANT);
                ui.painter().rect_stroke(rect, Rounding::same(12.0), Stroke::new(1.0, OUTLINE));
            }
            
            if response.clicked() {
                state.backend = backend;
                state.selected_version_index = 0;
            }
            
            ui.allocate_ui_at_rect(rect, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.label(RichText::new(icon).size(28.0));
                    ui.add_space(12.0);
                    ui.vertical(|ui| {
                        ui.add_space(6.0);
                        ui.label(RichText::new(name).size(16.0).color(ON_SURFACE).strong());
                        ui.label(RichText::new(desc).size(12.0).color(ON_SURFACE_VARIANT));
                    });
                });
            });
            
            ui.add_space(8.0);
        }
        
        ui.add_space(16.0);
        
        ui.label(RichText::new("Port").size(14.0).color(ON_SURFACE).strong());
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.add(DragValue::new(&mut state.port).speed(1.0).range(1024..=65535).suffix(" TCP"));
            ui.add_space(8.0);
            if outlined_button(ui, "Random").clicked() {
                state.port = 25565 + (fastrand::u16(..) % 10000);
            }
        });
    });
}

fn render_version_step(ui: &mut Ui, state: &mut CreateServerData) {
    let versions = get_versions_for_backend(&state.backend);
    
    ui.label(RichText::new("Select Minecraft Version").size(14.0).color(ON_SURFACE).strong());
    ui.add_space(4.0);
    ui.label(RichText::new("Choose the version compatible with your client").size(12.0).color(ON_SURFACE_VARIANT));
    ui.add_space(16.0);
    
    ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
        for (i, version) in versions.iter().enumerate() {
            let selected = i == state.selected_version_index;
            
            let response = ui.allocate_response(Vec2::new(ui.available_width(), 56.0), Sense::click());
            let rect = response.rect;
            
            if selected {
                ui.painter().rect_filled(rect, Rounding::same(12.0), PRIMARY_CONTAINER);
                ui.painter().rect_stroke(rect, Rounding::same(12.0), Stroke::new(2.0, PRIMARY));
            } else {
                ui.painter().rect_filled(rect, Rounding::same(12.0), SURFACE_VARIANT);
                ui.painter().rect_stroke(rect, Rounding::same(12.0), Stroke::new(1.0, OUTLINE));
            }
            
            if response.clicked() {
                state.selected_version_index = i;
                state.version = version.to_string();
            }
            
            ui.allocate_ui_at_rect(rect, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    ui.label(RichText::new("📦").size(24.0));
                    ui.add_space(12.0);
                    ui.vertical(|ui| {
                        ui.add_space(8.0);
                        ui.label(RichText::new(format!("Minecraft {}", version)).size(16.0).color(ON_SURFACE).strong());
                        ui.label(RichText::new(format!("Compatible with {} clients", version)).size(12.0).color(ON_SURFACE_VARIANT));
                    });
                    
                    ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(16.0);
                        if selected {
                            ui.label(RichText::new("✓ Selected").size(12.0).color(PRIMARY).strong());
                        }
                    });
                });
            });
            
            ui.add_space(8.0);
        }
    });
}

fn render_advanced_step(ui: &mut Ui, state: &mut CreateServerData) {
    ui.label(RichText::new("Advanced Settings").size(14.0).color(ON_SURFACE).strong());
    ui.add_space(4.0);
    ui.label(RichText::new("Optional configuration for your server").size(12.0).color(ON_SURFACE_VARIANT));
    ui.add_space(16.0);
    
    ui.label(RichText::new("Memory Allocation").size(14.0).color(ON_SURFACE).strong());
    ui.add_space(4.0);
    ui.add(DragValue::new(&mut state.memory_mb).speed(64.0).range(256..=8192).suffix(" MB"));
    ui.add_space(8.0);
    ui.label(RichText::new("Recommended: 1024 MB for small servers, 2048+ for larger worlds").size(11.0).color(ON_SURFACE_VARIANT));
    ui.add_space(16.0);
    
    ui.label(RichText::new("Playit.gg Tunnel").size(14.0).color(ON_SURFACE).strong());
    ui.add_space(4.0);
    ui.checkbox(&mut state.auto_tunnel, "Automatically create Playit.gg tunnel on server start");
    ui.add_space(8.0);
    ui.label(RichText::new("Allows players to join from anywhere without port forwarding").size(11.0).color(ON_SURFACE_VARIANT));
    ui.add_space(16.0);
    
    if let Some(err) = &state.error {
        ui.colored_label(ERROR, err);
        ui.add_space(8.0);
    }
}

fn render_confirm_step(
    ui: &mut Ui, 
    state: &mut CreateServerData, 
    should_close: &mut bool,
    messages: &mut Vec<crate::app::AppMessage>,
) {
    ui.label(RichText::new("Review Your Settings").size(14.0).color(ON_SURFACE).strong());
    ui.add_space(4.0);
    ui.label(RichText::new("Confirm the configuration before creating the server").size(12.0).color(ON_SURFACE_VARIANT));
    ui.add_space(16.0);
    
    card_frame().show(ui, |ui| {
        ui.vertical(|ui| {
            render_summary_row(ui, "Name", &state.name);
            render_summary_row(ui, "Backend", &state.backend.to_string());
            render_summary_row(ui, "Version", &state.version);
            render_summary_row(ui, "Port", &state.port.to_string());
            render_summary_row(ui, "Memory", &format!("{} MB", state.memory_mb));
            render_summary_row(ui, "Auto Tunnel", if state.auto_tunnel { "Enabled" } else { "Disabled" });
        });
    });
    ui.add_space(24.0);
    
    ui.horizontal(|ui| {
        if secondary_button(ui, "Back").clicked() {
            // Go back
        }
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            if primary_button(ui, "Create Server").clicked() {
                if state.name.trim().is_empty() {
                    state.error = Some("Server name cannot be empty".to_string());
                } else {
                    messages.push(crate::app::AppMessage::CreateServer {
                        name: state.name.clone(),
                        backend: state.backend,
                        port: state.port,
                    });
                    *should_close = true;
                }
            }
        });
    });
}

fn render_summary_row(ui: &mut Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(label).size(13.0).color(ON_SURFACE_VARIANT));
        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(RichText::new(value).size(13.0).color(ON_SURFACE).strong());
        });
    });
    ui.add_space(8.0);
}

fn get_versions_for_backend(backend: &ServerBackendType) -> Vec<&'static str> {
    let backend_impl = get_backend(backend);
    backend_impl.supported_versions()
        .into_iter()
        .map(|v| *v)
        .collect()
}