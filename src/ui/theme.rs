use eframe::egui::{Color32, Style, Visuals, FontDefinitions, FontFamily, FontData, FontId, TextStyle, CornerRadius, Stroke, Widgets};

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
    
    let mut visuals = Visuals::default();
    visuals.dark_mode = true;
    visuals.override_text_color = Some(Color32::from_rgb(232, 232, 232));
    visuals.window_fill = Color32::from_rgb(24, 24, 27);
    visuals.panel_fill = Color32::from_rgb(24, 24, 27);
    visuals.faint_bg_color = Color32::from_rgb(39, 39, 42);
    visuals.extreme_bg_color = Color32::from_rgb(18, 18, 20);
    visuals.code_bg_color = Color32::from_rgb(28, 28, 30);
    visuals.warn_fg_color = Color32::from_rgb(255, 180, 0);
    visuals.error_fg_color = Color32::from_rgb(255, 80, 80);
    visuals.hyperlink_color = Color32::from_rgb(100, 180, 255);
    visuals.selection = egui::epaint::Selection {
        bg_fill: Color32::from_rgba_premultiplied(100, 180, 255, 80),
        stroke: Stroke::new(1.0, Color32::from_rgb(100, 180, 255)),
    };
    visuals.widgets = Widgets {
        noninteractive: egui::WidgetVisuals {
            bg_fill: Color32::from_rgb(39, 39, 42),
            weak_bg_fill: Color32::from_rgb(28, 28, 30),
            bg_stroke: Stroke::new(1.0, Color32::from_rgb(63, 63, 70)),
            fg_stroke: Stroke::new(1.0, Color32::from_rgb(212, 212, 216)),
            rounding: CornerRadius::same(8),
            expansion: 0.0,
        },
        inactive: egui::WidgetVisuals {
            bg_fill: Color32::from_rgb(39, 39, 42),
            weak_bg_fill: Color32::from_rgb(28, 28, 30),
            bg_stroke: Stroke::new(1.0, Color32::from_rgb(63, 63, 70)),
            fg_stroke: Stroke::new(1.0, Color32::from_rgb(212, 212, 216)),
            rounding: CornerRadius::same(12),
            expansion: 0.0,
        },
        hovered: egui::WidgetVisuals {
            bg_fill: Color32::from_rgb(51, 51, 55),
            weak_bg_fill: Color32::from_rgb(39, 39, 42),
            bg_stroke: Stroke::new(1.0, Color32::from_rgb(100, 180, 255)),
            fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
            rounding: CornerRadius::same(12),
            expansion: 1.0,
        },
        active: egui::WidgetVisuals {
            bg_fill: Color32::from_rgb(100, 180, 255),
            weak_bg_fill: Color32::from_rgb(80, 160, 240),
            bg_stroke: Stroke::new(1.0, Color32::from_rgb(100, 180, 255)),
            fg_stroke: Stroke::new(1.0, Color32::from_rgb(255, 255, 255)),
            rounding: CornerRadius::same(12),
            expansion: 2.0,
        },
        open: egui::WidgetVisuals {
            bg_fill: Color32::from_rgb(39, 39, 42),
            weak_bg_fill: Color32::from_rgb(28, 28, 30),
            bg_stroke: Stroke::new(1.0, Color32::from_rgb(100, 180, 255)),
            fg_stroke: Stroke::new(1.0, Color32::from_rgb(212, 212, 216)),
            rounding: CornerRadius::same(8),
            expansion: 0.0,
        },
    };
    visuals.window_corner_radius = CornerRadius::same(16);
    visuals.window_shadow = egui::epaint::Shadow {
        offset: [0, 8],
        blur: 16,
        spread: 0,
        color: Color32::from_rgba_premultiplied(0, 0, 0, 120),
    };
    visuals.popup_shadow = egui::epaint::Shadow {
        offset: [0, 4],
        blur: 8,
        spread: 0,
        color: Color32::from_rgba_premultiplied(0, 0, 0, 100),
    };
    visuals.resize_corner_size = 16.0;
    visuals.button_frame = true;
    visuals.collapsing_header_frame = true;
    visuals.indent_has_left_vline = false;
    visuals.striped = true;
    visuals.slider_trailing_fill = true;

    style.visuals = visuals;

    style.spacing.item_spacing = egui::Vec2::new(12.0, 10.0);
    style.spacing.window_margin = egui::Margin::same(16);
    style.spacing.button_padding = egui::Vec2::new(16.0, 10.0);
    style.spacing.menu_margin = egui::Margin::same(8);
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

    ctx.set_style_of(egui::Theme::Dark, style);
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