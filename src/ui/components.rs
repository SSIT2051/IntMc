use eframe::egui::{Color32, Ui, Vec2, Align2, Rounding, Stroke, RichText, FontId, FontFamily, Response, Sense, Layout, Widget, TextStyle};
use crate::ui::theme::{PRIMARY, PRIMARY_CONTAINER, ON_SURFACE, ON_SURFACE_VARIANT, SURFACE_VARIANT, OUTLINE, SUCCESS, ERROR, WARNING};

pub fn render_empty_state(ui: &mut Ui, icon: &str, title: &str, subtitle: &str, action_text: Option<&str>, on_action: Option<impl FnOnce()>) {
    ui.vertical_centered(|ui| {
        ui.add_space(40.0);
        ui.label(RichText::new(icon).size(64.0).color(ON_SURFACE_VARIANT));
        ui.add_space(16.0);
        ui.label(RichText::new(title).size(20.0).color(ON_SURFACE).strong());
        ui.add_space(8.0);
        ui.label(RichText::new(subtitle).size(14.0).color(ON_SURFACE_VARIANT));
        
        if let (Some(text), Some(action)) = (action_text, on_action) {
            ui.add_space(24.0);
            if primary_button(ui, text).clicked() {
                action();
            }
        }
        
        ui.add_space(40.0);
    });
}

pub fn render_loading_state(ui: &mut Ui, message: &str) {
    ui.vertical_centered(|ui| {
        ui.add_space(40.0);
        ui.spinner();
        ui.add_space(16.0);
        ui.label(RichText::new(message).size(14.0).color(ON_SURFACE_VARIANT));
        ui.add_space(40.0);
    });
}

pub fn render_error_state(ui: &mut Ui, message: &str, on_retry: Option<impl FnOnce()>) {
    ui.vertical_centered(|ui| {
        ui.add_space(40.0);
        ui.label(RichText::new("⚠").size(48.0).color(ERROR));
        ui.add_space(16.0);
        ui.label(RichText::new("Something went wrong").size(18.0).color(ON_SURFACE).strong());
        ui.add_space(8.0);
        ui.label(RichText::new(message).size(14.0).color(ON_SURFACE_VARIANT));
        
        if let Some(retry) = on_retry {
            ui.add_space(24.0);
            if outlined_button(ui, "Try Again").clicked() {
                retry();
            }
        }
        
        ui.add_space(40.0);
    });
}

pub fn render_section_header(ui: &mut Ui, title: &str, subtitle: Option<&str>, action: Option<(&str, impl FnOnce())>) {
    ui.horizontal(|ui| {
        ui.strong(RichText::new(title).size(18.0));
        if let Some(sub) = subtitle {
            ui.add_space(8.0);
            ui.label(RichText::new(sub).size(12.0).color(ON_SURFACE_VARIANT));
        }
        
        if let Some((text, action_fn)) = action {
            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                if text_button(ui, text).clicked() {
                    action_fn();
                }
            });
        }
    });
    ui.add_space(8.0);
}

pub fn primary_button(ui: &mut Ui, text: &str) -> Response {
    let btn = egui::Button::new(RichText::new(text).color(Color32::WHITE).size(14.0).strong())
        .fill(PRIMARY)
        .rounding(Rounding::same(12.0))
        .min_size(Vec2::new(0.0, 44.0));
    ui.add(btn)
}

pub fn secondary_button(ui: &mut Ui, text: &str) -> Response {
    let btn = egui::Button::new(RichText::new(text).color(PRIMARY).size(14.0))
        .fill(PRIMARY_CONTAINER)
        .stroke(Stroke::new(1.0, PRIMARY))
        .rounding(Rounding::same(12.0))
        .min_size(Vec2::new(0.0, 44.0));
    ui.add(btn)
}

pub fn outlined_button(ui: &mut Ui, text: &str) -> Response {
    let btn = egui::Button::new(RichText::new(text).color(ON_SURFACE_VARIANT).size(14.0))
        .fill(SURFACE_VARIANT)
        .stroke(Stroke::new(1.0, OUTLINE))
        .rounding(Rounding::same(12.0))
        .min_size(Vec2::new(0.0, 44.0));
    ui.add(btn)
}

pub fn text_button(ui: &mut Ui, text: &str) -> Response {
    let btn = egui::Button::new(RichText::new(text).color(PRIMARY).size(14.0))
        .fill(Color32::TRANSPARENT)
        .rounding(Rounding::same(8.0))
        .min_size(Vec2::new(0.0, 36.0));
    ui.add(btn)
}

pub fn danger_button(ui: &mut Ui, text: &str) -> Response {
    let btn = egui::Button::new(RichText::new(text).color(Color32::WHITE).size(14.0).strong())
        .fill(ERROR)
        .rounding(Rounding::same(12.0))
        .min_size(Vec2::new(0.0, 44.0));
    ui.add(btn)
}

pub fn icon_button(ui: &mut Ui, text: &str, selected: bool) -> Response {
    let (fill, text_color) = if selected {
        (PRIMARY_CONTAINER, PRIMARY)
    } else {
        (Color32::TRANSPARENT, ON_SURFACE_VARIANT)
    };
    let btn = egui::Button::new(RichText::new(text).color(text_color).size(22.0))
        .fill(fill)
        .rounding(Rounding::same(12.0))
        .min_size(Vec2::new(48.0, 48.0));
    ui.add(btn)
}

pub fn chip(ui: &mut Ui, text: &str, selected: bool, color: Color32) -> Response {
    let (fill, text_color) = if selected {
        (color, Color32::WHITE)
    } else {
        (SURFACE_VARIANT, ON_SURFACE_VARIANT)
    };
    let btn = egui::Button::new(RichText::new(text).color(text_color).size(12.0).strong())
        .fill(fill)
        .rounding(Rounding::same(16.0))
        .min_size(Vec2::new(0.0, 32.0));
    ui.add(btn)
}

pub fn card_frame() -> egui::Frame {
    egui::Frame::default()
        .fill(Color32::from_rgb(32, 32, 35))
        .stroke(Stroke::new(1.0, OUTLINE))
        .rounding(Rounding::same(16.0))
        .inner_margin(egui::Margin::symmetric(16.0, 14.0))
        .shadow(egui::epaint::Shadow {
            extrusion: 4.0,
            color: Color32::from_rgba_premultiplied(0, 0, 0, 80),
        })
}

pub fn elevated_card_frame() -> egui::Frame {
    egui::Frame::default()
        .fill(Color32::from_rgb(28, 28, 30))
        .stroke(Stroke::new(1.0, OUTLINE))
        .rounding(Rounding::same(16.0))
        .inner_margin(egui::Margin::symmetric(16.0, 14.0))
        .shadow(egui::epaint::Shadow {
            extrusion: 8.0,
            color: Color32::from_rgba_premultiplied(0, 0, 0, 100),
        })
}

pub fn status_indicator(ui: &mut Ui, running: bool) -> Response {
    let (color, text) = if running {
        (SUCCESS, "Running")
    } else {
        (ERROR, "Stopped")
    };
    
    ui.horizontal(|ui| {
        let rect = ui.allocate_exact_size(Vec2::splat(8.0), Sense::hover()).rect;
        ui.painter().circle_filled(rect.center(), 4.0, color);
        ui.add_space(6.0);
        ui.label(RichText::new(text).size(12.0).color(color).strong());
    }).response
}

pub fn backend_badge(ui: &mut Ui, backend: &str) {
    let (color, icon) = match backend {
        "Pumpkin" => (Color32::from_rgb(100, 180, 255), "🎃"),
        "Steel" => (Color32::from_rgb(180, 200, 100), "⚙"),
        "Rosewater" => (Color32::from_rgb(255, 160, 100), "🌹"),
        "Glowstone" => (Color32::from_rgb(200, 100, 255), "✨"),
        _ => (Color32::from_rgb(150, 150, 150), "📦"),
    };
    
    ui.horizontal(|ui| {
        ui.label(RichText::new(icon).size(12.0));
        ui.add_space(4.0);
        ui.label(RichText::new(backend).size(11.0).color(color).strong());
    });
}

pub fn format_badge(ui: &mut Ui, format: &crate::storage::PluginFormat) {
    let (color, text) = match format {
        crate::storage::PluginFormat::Wasm => (Color32::from_rgb(100, 180, 255), "WASM"),
        crate::storage::PluginFormat::Jar => (Color32::from_rgb(180, 200, 100), "JAR"),
    };
    
    ui.horizontal(|ui| {
        let rect = ui.allocate_exact_size(Vec2::new(44.0, 20.0), Sense::hover()).rect;
        ui.painter().rect_filled(rect, Rounding::same(4.0), color.linear_multiply(0.15));
        ui.painter().rect_stroke(rect, Rounding::same(4.0), Stroke::new(1.0, color));
        ui.painter().text(rect.center(), Align2::CENTER_CENTER, 
            RichText::new(text).size(9.0).color(color).strong());
    });
}

pub fn version_badge(ui: &mut Ui, version: &str) {
    ui.horizontal(|ui| {
        let rect = ui.allocate_exact_size(Vec2::new(60.0, 20.0), Sense::hover()).rect;
        ui.painter().rect_filled(rect, Rounding::same(4.0), PRIMARY_CONTAINER);
        ui.painter().rect_stroke(rect, Rounding::same(4.0), Stroke::new(1.0, PRIMARY));
        ui.painter().text(rect.center(), Align2::CENTER_CENTER, 
            RichText::new(version).size(9.0).color(PRIMARY).strong());
    });
}

pub fn port_badge(ui: &mut Ui, port: u16) {
    ui.horizontal(|ui| {
        ui.label(RichText::new("🔌").size(11.0));
        ui.add_space(2.0);
        ui.label(RichText::new(port.to_string()).size(11.0).color(ON_SURFACE_VARIANT).monospace());
    });
}

pub fn player_count_badge(ui: &mut Ui, current: u32, max: u32) {
    let color = if current == 0 { ON_SURFACE_VARIANT } 
        else if current >= max { ERROR } 
        else { SUCCESS };
    
    ui.horizontal(|ui| {
        ui.label(RichText::new("👥").size(11.0));
        ui.add_space(2.0);
        ui.label(RichText::new(format!("{}/{}", current, max)).size(11.0).color(color).strong());
    });
}

pub fn confirm_dialog(ui: &mut Ui, title: &str, message: &str, confirm_text: &str, on_confirm: impl FnOnce(), on_cancel: impl FnOnce()) {
    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ui.ctx(), |ui| {
            ui.set_min_width(360.0);
            ui.vertical_centered(|ui| {
                ui.add_space(8.0);
                ui.label(RichText::new(message).size(14.0).color(ON_SURFACE));
                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    if secondary_button(ui, "Cancel").clicked() {
                        on_cancel();
                    }
                    ui.add_space(12.0);
                    if danger_button(ui, confirm_text).clicked() {
                        on_confirm();
                    }
                });
            });
        });
}

pub fn input_dialog(ui: &mut Ui, title: &str, label: &str, value: &mut String, placeholder: &str, on_confirm: impl FnOnce(), on_cancel: impl FnOnce()) {
    egui::Window::new(title)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .show(ui.ctx(), |ui| {
            ui.set_min_width(360.0);
            ui.vertical(|ui| {
                ui.add_space(8.0);
                ui.label(RichText::new(label).size(14.0).color(ON_SURFACE));
                ui.add_space(8.0);
                ui.add(TextEdit::singleline(value).hint_text(placeholder).desired_width(f32::INFINITY));
                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    if secondary_button(ui, "Cancel").clicked() {
                        on_cancel();
                    }
                    ui.add_space(12.0);
                    if primary_button(ui, "OK").clicked() {
                        on_confirm();
                    }
                });
            });
        });
}