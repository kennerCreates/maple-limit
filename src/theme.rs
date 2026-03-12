use iced::Color;
use iced::Theme;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemePalette {
    pub name: String,
    #[serde(with = "color_array")]
    pub colors: [Color; 5],
}

mod color_array {
    use iced::Color;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(colors: &[Color; 5], s: S) -> Result<S::Ok, S::Error> {
        let proxies: Vec<ColorProxy> = colors.iter().map(|c| ColorProxy { r: c.r, g: c.g, b: c.b, a: c.a }).collect();
        proxies.serialize(s)
    }

    #[derive(Serialize, Deserialize)]
    struct ColorProxy { r: f32, g: f32, b: f32, a: f32 }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[Color; 5], D::Error> {
        let proxies: Vec<ColorProxy> = Vec::deserialize(d)?;
        if proxies.len() != 5 {
            return Err(serde::de::Error::custom("expected 5 colors"));
        }
        let mut arr = [Color::BLACK; 5];
        for (i, p) in proxies.into_iter().enumerate() {
            arr[i] = Color { r: p.r, g: p.g, b: p.b, a: p.a };
        }
        Ok(arr)
    }
}

impl ThemePalette {
    pub fn default_dark() -> Self {
        Self {
            name: "Twilight 5".to_string(),
            colors: [
                Color::from_rgb(0xfb as f32 / 255.0, 0xbb as f32 / 255.0, 0xad as f32 / 255.0), // #fbbbad peachy-pink
                Color::from_rgb(0xee as f32 / 255.0, 0x86 as f32 / 255.0, 0x95 as f32 / 255.0), // #ee8695 muted rose
                Color::from_rgb(0x4a as f32 / 255.0, 0x7a as f32 / 255.0, 0x96 as f32 / 255.0), // #4a7a96 blue-gray
                Color::from_rgb(0x33 as f32 / 255.0, 0x3f as f32 / 255.0, 0x58 as f32 / 255.0), // #333f58 deep navy
                Color::from_rgb(0x29 as f32 / 255.0, 0x28 as f32 / 255.0, 0x31 as f32 / 255.0), // #292831 dark charcoal
            ],
        }
    }

    pub fn default_light() -> Self {
        // Golden Sunset palette (https://lospec.com/palette-list/golden-sunset)
        Self {
            name: "Golden Sunset".to_string(),
            colors: [
                Color::from_rgb(0xff as f32 / 255.0, 0xec as f32 / 255.0, 0xd6 as f32 / 255.0), // #ffecd6 light peach
                Color::from_rgb(0xff as f32 / 255.0, 0xb8 as f32 / 255.0, 0x73 as f32 / 255.0), // #ffb873 warm orange
                Color::from_rgb(0xcb as f32 / 255.0, 0x76 as f32 / 255.0, 0x5c as f32 / 255.0), // #cb765c dusty rose
                Color::from_rgb(0x7a as f32 / 255.0, 0x4a as f32 / 255.0, 0x5a as f32 / 255.0), // #7a4a5a muted plum
                Color::from_rgb(0x25 as f32 / 255.0, 0x21 as f32 / 255.0, 0x3e as f32 / 255.0), // #25213e deep indigo
            ],
        }
    }

    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self::default_dark(),
            ThemeMode::Light => Self::default_light(),
        }
    }
}

impl Default for ThemePalette {
    fn default() -> Self {
        Self::default_dark()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {}

/// Indices into ThemePalette::colors for the 7 editable UI elements.
/// Order matches EDITABLE_FIELDS: icon, text, panel_bg, panel_border, canvas_bg,
/// grid, selection_highlight.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ThemeMapping {
    pub indices: [usize; 7],
}

impl ThemeMapping {
    pub fn default_dark() -> Self {
        // T1=0, T2=1, T3=2, T4=3, T5=4
        Self {
            indices: [0, 0, 4, 3, 3, 2, 2],
        }
    }

    pub fn default_light() -> Self {
        Self {
            indices: [4, 4, 0, 1, 0, 1, 3],
        }
    }

    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self::default_dark(),
            ThemeMode::Light => Self::default_light(),
        }
    }
}

pub const EDITABLE_FIELDS: &[(&str, &str)] = &[
    ("icon_color", "Icon"),
    ("text", "Text"),
    ("panel_bg", "Panel BG"),
    ("panel_border", "Panel Border"),
    ("canvas_bg", "Canvas BG"),
    ("grid", "Grid"),
    ("selection_highlight", "Selection"),
];

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
    // Icons & text
    pub icon_color: Color,
    pub text: Color,
    // Floating panels
    pub panel_bg: Color,
    pub panel_border: Color,
    pub panel_button_active: Color,
    pub panel_button_hover: Color,
}

impl EditorColors {
    pub fn from_palette(palette: &ThemePalette, mode: ThemeMode, mapping: &ThemeMapping) -> Self {
        let p = &palette.colors;
        let m = &mapping.indices;

        // Editable fields from mapping
        let icon_color = p[m[0]];
        let text = p[m[1]];
        let panel_bg = p[m[2]];
        let panel_border = p[m[3]];
        let canvas_bg = p[m[4]];
        let grid = p[m[5]];
        let selection_highlight = p[m[6]];

        // Non-editable fields derived from palette + mode
        match mode {
            ThemeMode::Dark => Self {
                canvas_bg,
                grid_line: grid,
                grid_dot: grid,
                selection_highlight,
                pen_anchor: p[2],
                pen_handle_stroke: p[2],
                pen_handle_fill: p[4],
                pen_curve: p[0],
                pen_preview: p[1],
                polyline_dot: p[2],
                swatch_border: Color::from_rgb(p[2].r * 0.5, p[2].g * 0.5, p[2].b * 0.5),
                swatch_border_selected: p[2],
                swatch_border_picked_up: p[0],
                swatch_border_drop_target: p[1],
                swatch_none_bg: p[3],
                swatch_none_text: p[1],
                end_drop_bg: p[3],
                end_drop_text: p[2],
                end_drop_border: p[2],
                icon_color,
                text,
                panel_bg,
                panel_border,
                panel_button_active: p[2],
                panel_button_hover: p[3],
            },
            ThemeMode::Light => Self {
                canvas_bg,
                grid_line: grid,
                grid_dot: grid,
                selection_highlight,
                pen_anchor: p[3],
                pen_handle_stroke: p[2],
                pen_handle_fill: p[0],
                pen_curve: p[4],
                pen_preview: p[2],
                polyline_dot: p[3],
                swatch_border: p[2],
                swatch_border_selected: p[3],
                swatch_border_picked_up: p[4],
                swatch_border_drop_target: p[3],
                swatch_none_bg: p[0],
                swatch_none_text: p[3],
                end_drop_bg: p[0],
                end_drop_text: p[3],
                end_drop_border: p[3],
                icon_color,
                text,
                panel_bg,
                panel_border,
                panel_button_active: p[2],
                panel_button_hover: p[0],
            },
        }
    }
}

pub fn iced_theme(palette: &ThemePalette, mode: ThemeMode) -> Theme {
    let p = &palette.colors;
    match mode {
        ThemeMode::Dark => {
            let pal = iced::theme::Palette {
                background: p[4],
                text: p[0],
                primary: p[2],
                success: p[2],
                danger: p[1],
                warning: p[0],
            };
            Theme::custom("Custom Dark".to_string(), pal)
        }
        ThemeMode::Light => {
            let pal = iced::theme::Palette {
                background: p[0],
                text: p[4],
                primary: p[3],
                success: p[3],
                danger: p[1],
                warning: p[2],
            };
            Theme::custom("Custom Light".to_string(), pal)
        }
    }
}
