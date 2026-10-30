use eframe::egui::{Color32, Style, Visuals, FontDefinitions, FontFamily, FontData, FontId, TextStyle, Rounding, Stroke, Widgets};

pub fn setup_material3_theme(ctx: &egui::Context) {
    let font_bytes = include_bytes!("../../assets/DejaVuSans.ttf");
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "dejavu_sans".to_owned(),
        std::sync::Arc::new(FontData::from_static(font_bytes)),
    );
    fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "dejavu_sans".to_owned());
    fonts.families.entry(FontFamily::Monospace).or_default().insert(0, "dejavu_sans".to_owned());
    ctx.set_fonts(fonts);

    let mut style = Style::default();
    
    style.visuals = Visuals {
        dark_mode: true,
        override_text_color: Some(Color32::from_rgb(232, 232, 232)),
        window_fill: Color32::from_rgb(24, 24, 27),
        panel_fill: Color32::from_rgb(24, 24, 27),
        faint_bg_color: Color32::from_rgb(39, 39, 42),
        extreme_bg_color: Color32::from_rgb(18, 18, 20),
        code_bg_color: Color32::from_rgb(28, 28, 30),
        warn_fg_color: Color32::from_rgb(255, 180, 0),
        error_fg_color: Color32::from_rgb(255, 80, 80),
        hyperlink_color: Color32::from_rgb(100, 180, 255),
        selection: egui::epaint::Selection {
            bg_fill: Color32::from_rgba_premultiplied(100, 180, 255, 80),
            stroke: Stroke::new(1.0, Color32::from_rgb(100, 180, 255)),
        },
        widgets: Widgets {
            noninteractive: egui::WidgetVisuals {
                bg_fill: Color32::from_rgb(39, 39, 42),
                weak_bg_fill: Color32::from_rgb(28, 28, 30),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(63, 63, 70)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(212, 212, 216)),
                rounding: Rounding::same(8.0),
                expansion: 0.0,
            },
            inactive: egui::WidgetVisuals {
                bg_fill: Color32::from_rgb(39, 39, 42),
                weak_bg_fill: Color32::from_rgb(28, 28, 30),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(63, 63, 70)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(212, 212, 216)),
                rounding: Rounding::same(12.0),
                expansion: 0.0,
            },
            hovered: egui::WidgetVisuals {
                bg_fill: Color32::from_rgb(51, 51, 55),
                weak_bg_fill: Color32::from_rgb(39, 39, 42),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(100, 180, 255)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
                rounding: Rounding::same(12.0),
                expansion: 1.0,
            },
            active: egui::WidgetVisuals {
                bg_fill: Color32::from_rgb(100, 180, 255),
                weak_bg_fill: Color32::from_rgb(80, 160, 240),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(100, 180, 255)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
                rounding: Rounding::same(12.0),
                expansion: 2.0,
            },
            open: egui::WidgetVisuals {
                bg_fill: Color32::from_rgb(39, 39, 42),
                weak_bg_fill: Color32::from_rgb(28, 28, 30),
                bg_stroke: Stroke::new(1.0, Color32::from_rgb(100, 180, 255)),
                fg_stroke: Stroke::new(1.0, Color32::from_rgb(212, 212, 216)),
                rounding: Rounding::same(8.0),
                expansion: 0.0,
            },
        },
        window_corner_radius: Rounding::same(16.0),
        window_shadow: egui::epaint::Shadow {
            extrusion: 16.0,
            color: Color32::from_rgba_premultiplied(0, 0, 0, 120),
        },
        popup_shadow: egui::epaint::Shadow {
            extrusion: 8.0,
            color: Color32::from_rgba_premultiplied(0, 0, 0, 100),
        },
        resize_corner_size: 16.0,
        text_cursor: Stroke::new(2.0, Color32::from_rgb(100, 180, 255)),
        clip_rect_margin: 4.0,
        button_frame: true,
        collapsing_header_frame: true,
        indent_has_left_vline: false,
        striped: true,
        slider_trailing_fill: true,
    };

    style.spacing.item_spacing = egui::Vec2::new(12.0, 10.0);
    style.spacing.window_margin = egui::Margin::same(16.0);
    style.spacing.button_padding = egui::Vec2::new(16.0, 10.0);
    style.spacing.menu_margin = egui::Margin::same(8.0);
    style.spacing.slider_width = 200.0;
    style.spacing.text_edit_width = 280.0;
    style.spacing.combo_width = 200.0;
    style.spacing.indent = 24.0;
    style.spacing.icon_width = 20.0;
    style.spacing.icon_width_inner = 16.0;
    style.spacing.interact_size = egui::Vec2::new(48.0, 40.0);
    style.spacing.scroll = egui::style::ScrollStyle::default();

    style.text_styles.insert(TextStyle::Heading, FontId::new(28.0, FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Body, FontId::new(15.0, FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Monospace, FontId::new(13.0, FontFamily::Monospace));
    style.text_styles.insert(TextStyle::Button, FontId::new(14.0, FontFamily::Proportional));
    style.text_styles.insert(TextStyle::Small, FontId::new(12.0, FontFamily::Proportional));

    ctx.set_style(style);
}

pub const PRIMARY: Color32 = Color32::from_rgb(100, 180, 255);
pub const PRIMARY_CONTAINER: Color32 = Color32::from_rgb(30, 60, 90);
pub const SECONDARY: Color32 = Color32::from_rgb(180, 200, 100);
pub const SECONDARY_CONTAINER: Color32 = Color32::from_rgb(50, 60, 30);
pub const TERTIARY: Color32 = Color32::from_rgb(255, 160, 100);
pub const TERTIARY_CONTAINER: Color32 = Color32::from_rgb(70, 40, 20);
pub const ERROR: Color32 = Color32::from_rgb(255, 80, 80);
pub const ERROR_CONTAINER: Color32 = Color32::from_rgb(70, 20, 20);
pub const SURFACE: Color32 = Color32::from_rgb(28, 28, 30);
pub const SURFACE_VARIANT: Color32 = Color32::from_rgb(39, 39, 42);
pub const SURFACE_CONTAINER: Color32 = Color32::from_rgb(32, 32, 35);
pub const OUTLINE: Color32 = Color32::from_rgb(63, 63, 70);
pub const OUTLINE_VARIANT: Color32 = Color32::from_rgb(50, 50, 55);
pub const ON_SURFACE: Color32 = Color32::from_rgb(232, 232, 232);
pub const ON_SURFACE_VARIANT: Color32 = Color32::from_rgb(180, 180, 185);
pub const INVERSE_SURFACE: Color32 = Color32::from_rgb(240, 240, 245);
pub const INVERSE_ON_SURFACE: Color32 = Color32::from_rgb(30, 30, 30);
pub const SUCCESS: Color32 = Color32::from_rgb(80, 220, 120);
pub const WARNING: Color32 = Color32::from_rgb(255, 200, 50);

pub fn card_frame() -> egui::Frame {
    egui::Frame::default()
        .fill(SURFACE_CONTAINER)
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
        .fill(SURFACE)
        .stroke(Stroke::new(1.0, OUTLINE_VARIANT))
        .rounding(Rounding::same(16.0))
        .inner_margin(egui::Margin::symmetric(16.0, 14.0))
        .shadow(egui::epaint::Shadow {
            extrusion: 8.0,
            color: Color32::from_rgba_premultiplied(0, 0, 0, 100),
        })
}

pub fn primary_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let btn = egui::Button::new(egui::RichText::new(text).color(ON_SURFACE).size(14.0))
        .fill(PRIMARY)
        .rounding(Rounding::same(12.0))
        .min_size(egui::Vec2::new(0.0, 44.0));
    ui.add(btn)
}

pub fn secondary_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let btn = egui::Button::new(egui::RichText::new(text).color(PRIMARY).size(14.0))
        .fill(PRIMARY_CONTAINER)
        .stroke(Stroke::new(1.0, PRIMARY))
        .rounding(Rounding::same(12.0))
        .min_size(egui::Vec2::new(0.0, 44.0));
    ui.add(btn)
}

pub fn outlined_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let btn = egui::Button::new(egui::RichText::new(text).color(ON_SURFACE_VARIANT).size(14.0))
        .fill(SURFACE_VARIANT)
        .stroke(Stroke::new(1.0, OUTLINE))
        .rounding(Rounding::same(12.0))
        .min_size(egui::Vec2::new(0.0, 44.0));
    ui.add(btn)
}

pub fn text_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let btn = egui::Button::new(egui::RichText::new(text).color(PRIMARY).size(14.0))
        .fill(Color32::TRANSPARENT)
        .rounding(Rounding::same(8.0))
        .min_size(egui::Vec2::new(0.0, 36.0));
    ui.add(btn)
}

pub fn danger_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let btn = egui::Button::new(egui::RichText::new(text).color(ON_SURFACE).size(14.0))
        .fill(ERROR)
        .rounding(Rounding::same(12.0))
        .min_size(egui::Vec2::new(0.0, 44.0));
    ui.add(btn)
}

pub fn icon_button(ui: &mut egui::Ui, text: &str, selected: bool) -> egui::Response {
    let (fill, text_color) = if selected {
        (PRIMARY_CONTAINER, PRIMARY)
    } else {
        (Color32::TRANSPARENT, ON_SURFACE_VARIANT)
    };
    let btn = egui::Button::new(egui::RichText::new(text).color(text_color).size(22.0))
        .fill(fill)
        .rounding(Rounding::same(12.0))
        .min_size(egui::Vec2::new(48.0, 48.0));
    ui.add(btn)
}

pub fn chip(ui: &mut egui::Ui, text: &str, selected: bool, color: Color32) -> egui::Response {
    let (fill, text_color) = if selected {
        (color, Color32::WHITE)
    } else {
        (SURFACE_VARIANT, ON_SURFACE_VARIANT)
    };
    let btn = egui::Button::new(egui::RichText::new(text).color(text_color).size(12.0))
        .fill(fill)
        .rounding(Rounding::same(16.0))
        .min_size(egui::Vec2::new(0.0, 32.0));
    ui.add(btn)
}

pub fn status_chip(ui: &mut egui::Ui, text: &str, status: ServerStatus) -> egui::Response {
    let (fill, text_color, dot_color) = match status {
        ServerStatus::Running => (SUCCESS.linear_multiply(0.15), SUCCESS, SUCCESS),
        ServerStatus::Stopped => (ERROR_CONTAINER, ERROR, ERROR),
        ServerStatus::Starting => (WARNING.linear_multiply(0.15), WARNING, WARNING),
        ServerStatus::Stopping => (WARNING.linear_multiply(0.15), WARNING, WARNING),
        ServerStatus::Error => (ERROR_CONTAINER, ERROR, ERROR),
        ServerStatus::Unknown => (OUTLINE_VARIANT.linear_multiply(0.15), ON_SURFACE_VARIANT, OUTLINE),
    };
    
    ui.horizontal(|ui| {
        ui.add_space(2.0);
        let rect = ui.allocate_exact_size(egui::Vec2::splat(8.0), egui::Sense::hover());
        ui.painter().circle_filled(rect.center(), 4.0, dot_color);
        ui.add_space(6.0);
        ui.label(egui::RichText::new(text).color(text_color).size(12.0));
        ui.add_space(2.0);
    }).response
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerStatus {
    Running,
    Stopped,
    Starting,
    Stopping,
    Error,
    Unknown,
}

impl ServerStatus {
    pub fn from_bool(running: bool) -> Self {
        if running { ServerStatus::Running } else { ServerStatus::Stopped }
    }
}