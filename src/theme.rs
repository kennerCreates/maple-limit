use iced::Color;
use iced::Theme;

// Twilight 5 palette
const T1: Color = Color::from_rgb(0xfb as f32 / 255.0, 0xbb as f32 / 255.0, 0xad as f32 / 255.0); // #fbbbad peachy-pink
const T2: Color = Color::from_rgb(0xee as f32 / 255.0, 0x86 as f32 / 255.0, 0x95 as f32 / 255.0); // #ee8695 muted rose
const T3: Color = Color::from_rgb(0x4a as f32 / 255.0, 0x7a as f32 / 255.0, 0x96 as f32 / 255.0); // #4a7a96 blue-gray
const T4: Color = Color::from_rgb(0x33 as f32 / 255.0, 0x3f as f32 / 255.0, 0x58 as f32 / 255.0); // #333f58 deep navy
const T5: Color = Color::from_rgb(0x29 as f32 / 255.0, 0x28 as f32 / 255.0, 0x31 as f32 / 255.0); // #292831 dark charcoal

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {
    pub fn toggle(self) -> Self {
        match self {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
        }
    }

}

#[derive(Debug, Clone, Copy)]
pub struct EditorColors {
    // Canvas
    pub canvas_bg: Color,
    pub grid_line: Color,
    pub grid_dot: Color,
    // Tool overlays
    pub selection_highlight: Color,
    pub pen_anchor: Color,
    pub pen_handle_stroke: Color,
    pub pen_handle_fill: Color,
    pub pen_curve: Color,
    pub pen_preview: Color,
    pub polyline_dot: Color,
    // Sidebar swatches
    pub swatch_border: Color,
    pub swatch_border_selected: Color,
    pub swatch_border_picked_up: Color,
    pub swatch_border_drop_target: Color,
    pub swatch_none_bg: Color,
    pub swatch_none_text: Color,
    pub end_drop_bg: Color,
    pub end_drop_text: Color,
    pub end_drop_border: Color,
    // Floating panels
    pub panel_bg: Color,
    pub panel_border: Color,
    pub panel_button_active: Color,
    pub panel_button_hover: Color,
}

impl EditorColors {
    pub fn dark() -> Self {
        Self {
            // Canvas
            canvas_bg: T4,
            grid_line: Color::from_rgba(0.20, 0.24, 0.38, 0.5),
            grid_dot: Color::from_rgba(T3.r, T3.g, T3.b, 0.4),
            // Tool overlays
            selection_highlight: T3,
            pen_anchor: T3,
            pen_handle_stroke: Color::from_rgb(T1.r * 0.6, T1.g * 0.6, T1.b * 0.6),
            pen_handle_fill: T5,
            pen_curve: T1,
            pen_preview: Color::from_rgba(T1.r, T1.g, T1.b, 0.4),
            polyline_dot: T3,
            // Sidebar swatches
            swatch_border: Color::from_rgb(T3.r * 0.5, T3.g * 0.5, T3.b * 0.5),
            swatch_border_selected: T3,
            swatch_border_picked_up: T1,
            swatch_border_drop_target: T2,
            swatch_none_bg: T4,
            swatch_none_text: T2,
            end_drop_bg: T4,
            end_drop_text: T3,
            end_drop_border: T3,
            // Floating panels
            panel_bg: T5,
            panel_border: Color::from_rgba(T3.r, T3.g, T3.b, 0.3),
            panel_button_active: T3,
            panel_button_hover: Color::from_rgb(T5.r + 0.06, T5.g + 0.06, T5.b + 0.06),
        }
    }

    pub fn light() -> Self {
        Self {
            // Canvas - T1 tinted, lighter than sidebar bg
            canvas_bg: Color::from_rgb(T1.r * 0.97 + 0.03, T1.g * 0.97 + 0.03, T1.b * 0.97 + 0.03),
            grid_line: Color::from_rgba(T2.r, T2.g, T2.b, 0.25),
            grid_dot: Color::from_rgba(T3.r, T3.g, T3.b, 0.35),
            // Tool overlays - darker accents
            selection_highlight: T4,
            pen_anchor: T4,
            pen_handle_stroke: T3,
            pen_handle_fill: T1,
            pen_curve: T5,
            pen_preview: Color::from_rgba(T5.r, T5.g, T5.b, 0.4),
            polyline_dot: T4,
            // Sidebar swatches - darker accents for borders
            swatch_border: Color::from_rgba(T3.r, T3.g, T3.b, 0.5),
            swatch_border_selected: T4,
            swatch_border_picked_up: T5,
            swatch_border_drop_target: T4,
            swatch_none_bg: T1,
            swatch_none_text: T4,
            end_drop_bg: T1,
            end_drop_text: T4,
            end_drop_border: T4,
            // Floating panels
            panel_bg: Color::from_rgb(0.98, 0.95, 0.93),
            panel_border: Color::from_rgba(T3.r, T3.g, T3.b, 0.3),
            panel_button_active: T3,
            panel_button_hover: Color::from_rgba(T4.r, T4.g, T4.b, 0.12),
        }
    }

    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Light => Self::light(),
        }
    }
}

pub fn iced_theme(mode: ThemeMode) -> Theme {
    match mode {
        ThemeMode::Dark => {
            let palette = iced::theme::Palette {
                background: T5,
                text: T1,
                primary: T3,
                success: T3,
                danger: T2,
                warning: T1,
            };
            Theme::custom("Twilight Dark".to_string(), palette)
        }
        ThemeMode::Light => {
            let palette = iced::theme::Palette {
                background: T1,
                text: T5,
                primary: T4,
                success: T4,
                danger: T2,
                warning: T3,
            };
            Theme::custom("Twilight Light".to_string(), palette)
        }
    }
}
