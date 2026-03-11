use iced::widget::{button, container, row, slider, svg, text, text_input, Column, Space};
use iced::{Background, Color, Element};

use crate::app::{Message, PaletteTarget, SidebarMode};
use crate::grid::{GridConfig, GridStyle};
use crate::palette::Palette;
use crate::shape::{LineCap, LineJoin, ShapeItem, Style};
use crate::theme::{EditorColors, ThemeMode};
use crate::tool::{ShapeType, Tool};

const ICON: f32 = 24.0;
const SHAPE_ICON: f32 = 30.0;
const SMALL_ICON: f32 = 22.0;

fn icon(name: &str, size: f32, color: Color) -> svg::Svg<'static> {
    svg::Svg::new(svg::Handle::from_path(format!("assets/icons/{}.svg", name)))
        .width(size)
        .height(size)
        .style(move |_theme, _status| svg::Style {
            color: Some(color),
        })
}

fn icon_toggle<'a>(
    icon_name: &str,
    active: bool,
    on_press: Message,
    colors: EditorColors,
) -> Element<'a, Message> {
    let active_bg = colors.panel_button_active;
    let hover_bg = colors.panel_button_hover;
    let icon_color = colors.icon_color;
    button(icon(icon_name, ICON, icon_color))
        .on_press(on_press)
        .padding(3)
        .style(move |_theme, status| {
            let bg = if active {
                Some(Background::Color(active_bg))
            } else {
                match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        Some(Background::Color(hover_bg))
                    }
                    _ => None,
                }
            };
            button::Style {
                background: bg,
                border: iced::Border { radius: 4.0.into(), ..Default::default() },
                ..Default::default()
            }
        })
        .into()
}

fn shape_icon_toggle<'a>(
    icon_name: &str,
    active: bool,
    on_press: Message,
    colors: EditorColors,
) -> Element<'a, Message> {
    let active_bg = colors.panel_button_active;
    let hover_bg = colors.panel_button_hover;
    let icon_color = colors.icon_color;
    button(icon(icon_name, SHAPE_ICON, icon_color))
        .on_press(on_press)
        .padding(3)
        .style(move |_theme, status| {
            let bg = if active {
                Some(Background::Color(active_bg))
            } else {
                match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        Some(Background::Color(hover_bg))
                    }
                    _ => None,
                }
            };
            button::Style {
                background: bg,
                border: iced::Border { radius: 4.0.into(), ..Default::default() },
                ..Default::default()
            }
        })
        .into()
}

fn small_shape_button<'a>(
    label: &'a str,
    shape: ShapeType,
    active: bool,
    colors: EditorColors,
) -> Element<'a, Message> {
    let active_bg = colors.panel_button_active;
    let hover_bg = colors.panel_button_hover;
    button(
        container(text(label).size(10))
            .center_x(SMALL_ICON)
            .center_y(SMALL_ICON),
    )
    .width(SMALL_ICON + 8.0)
    .height(SMALL_ICON + 8.0)
    .on_press(Message::SetShapeType(shape))
    .padding(0)
    .style(move |_theme, status| {
        let bg = if active {
            Some(Background::Color(active_bg))
        } else {
            match status {
                button::Status::Hovered | button::Status::Pressed => {
                    Some(Background::Color(hover_bg))
                }
                _ => None,
            }
        };
        button::Style {
            background: bg,
            border: iced::Border { radius: 4.0.into(), ..Default::default() },
            ..Default::default()
        }
    })
    .into()
}

fn text_button<'a>(
    label: &'a str,
    on_press: Message,
    colors: EditorColors,
) -> Element<'a, Message> {
    let hover_bg = colors.panel_button_hover;
    let icon_color = colors.icon_color;
    button(text(label).size(10).color(icon_color))
        .on_press(on_press)
        .padding(4)
        .style(move |_theme, status| {
            let bg = match status {
                button::Status::Hovered | button::Status::Pressed => {
                    Some(Background::Color(hover_bg))
                }
                _ => None,
            };
            button::Style {
                background: bg,
                border: iced::Border { radius: 4.0.into(), ..Default::default() },
                ..Default::default()
            }
        })
        .into()
}

#[allow(clippy::too_many_arguments)]
pub fn view<'a>(
    sidebar_mode: SidebarMode,
    active_tool: Tool,
    style: &Style,
    shape_type: ShapeType,
    skew_angle: f32,
    palette: &Palette,
    palette_slug: &str,
    palette_status: &'a str,
    grid: &GridConfig,
    selected_shape: Option<&ShapeItem>,
    palette_target: Option<PaletteTarget>,
    stroke_color_index: Option<usize>,
    fill_color_index: Option<usize>,
    reorder_mode: bool,
    reorder_src: Option<usize>,
    colors: EditorColors,
    polygon_submenu_open: bool,
    color_picker_target: Option<usize>,
    color_picker_r: f32,
    color_picker_g: f32,
    color_picker_b: f32,
    theme_mode: ThemeMode,
    base_text_size: f32,
    settings_color_field: Option<&str>,
    settings_picker_r: f32,
    settings_picker_g: f32,
    settings_picker_b: f32,
) -> Element<'a, Message> {
    let items = match sidebar_mode {
        SidebarMode::ToolConfig => build_tool_config(
            active_tool, style, shape_type, skew_angle, palette,
            selected_shape, palette_target, stroke_color_index, fill_color_index,
            reorder_mode, reorder_src, colors, polygon_submenu_open,
        ),
        SidebarMode::Palette => build_palette_panel(
            palette, palette_slug, palette_status,
            reorder_mode, reorder_src, colors,
            color_picker_target, color_picker_r, color_picker_g, color_picker_b,
        ),
        SidebarMode::Settings => build_settings_panel(
            theme_mode, grid, colors, base_text_size,
            settings_color_field, settings_picker_r, settings_picker_g, settings_picker_b,
        ),
    };

    let panel_bg = colors.panel_bg;
    let panel_border = colors.panel_border;
    container(
        Column::with_children(items)
            .spacing(6)
            .padding(8)
            .width(180),
    )
    .style(move |_theme| container::Style {
        background: Some(iced::Background::Color(panel_bg)),
        border: iced::Border {
            radius: 8.0.into(),
            width: 1.0,
            color: panel_border,
        },
        ..Default::default()
    })
    .into()
}

// ─── Tool Config (existing sidebar content) ───

fn build_tool_config<'a>(
    active_tool: Tool,
    style: &Style,
    shape_type: ShapeType,
    skew_angle: f32,
    palette: &Palette,
    selected_shape: Option<&ShapeItem>,
    palette_target: Option<PaletteTarget>,
    stroke_color_index: Option<usize>,
    fill_color_index: Option<usize>,
    reorder_mode: bool,
    reorder_src: Option<usize>,
    colors: EditorColors,
    polygon_submenu_open: bool,
) -> Vec<Element<'a, Message>> {
    let mut items: Vec<Element<'a, Message>> = Vec::new();

    // --- Selected shape editing ---
    if active_tool == Tool::Select {
        if let Some(shape) = selected_shape {
            let s = shape.style();

            items.push(
                row![
                    icon("style_stroke", ICON, colors.icon_color),
                    slider(0.0..=20.0, s.stroke_width, Message::SetSelectedStrokeWidth).step(0.5),
                    text_input("", &format!("{:.1}", s.stroke_width))
                        .on_input(Message::SelectedStrokeWidthInput)
                        .size(11)
                        .width(40),
                ]
                .spacing(4)
                .align_y(iced::Alignment::Center)
                .into(),
            );

            if let Some(cr) = shape.corner_radius() {
                items.push(
                    row![
                        icon("style_corner", ICON, colors.icon_color),
                        slider(0.0..=100.0, cr, Message::SetSelectedCornerRadius).step(1.0),
                        text(format!("{:.0}", cr)).size(11),
                    ]
                    .spacing(4)
                    .align_y(iced::Alignment::Center)
                    .into(),
                );
            }

            items.push(
                row![
                    icon_toggle("cap_butt", s.line_cap == LineCap::Butt, Message::SetSelectedLineCap(LineCap::Butt), colors),
                    icon_toggle("cap_round", s.line_cap == LineCap::Round, Message::SetSelectedLineCap(LineCap::Round), colors),
                    icon_toggle("cap_square", s.line_cap == LineCap::Square, Message::SetSelectedLineCap(LineCap::Square), colors),
                ]
                .spacing(2)
                .into(),
            );

            items.push(
                row![
                    icon_toggle("join_miter", s.line_join == LineJoin::Miter, Message::SetSelectedLineJoin(LineJoin::Miter), colors),
                    icon_toggle("join_round", s.line_join == LineJoin::Round, Message::SetSelectedLineJoin(LineJoin::Round), colors),
                    icon_toggle("join_bevel", s.line_join == LineJoin::Bevel, Message::SetSelectedLineJoin(LineJoin::Bevel), colors),
                ]
                .spacing(2)
                .into(),
            );
        }
    }

    // --- Tool style (for new shapes) ---
    if active_tool != Tool::Select {
        items.push(
            row![
                icon("style_stroke", ICON, colors.icon_color),
                slider(0.0..=20.0, style.stroke_width, Message::SetStrokeWidth).step(0.5),
                text_input("", &format!("{:.1}", style.stroke_width))
                    .on_input(Message::StrokeWidthInput)
                    .size(11)
                    .width(40),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center)
            .into(),
        );
    }

    // Shape tool config
    if active_tool == Tool::Shape {
        let is_polygon = matches!(
            shape_type,
            ShapeType::Heptagon | ShapeType::Octagon | ShapeType::Nonagon
                | ShapeType::Decagon | ShapeType::Hendecagon | ShapeType::Dodecagon
        );

        items.push(
            row![
                shape_icon_toggle("shape_triangle", shape_type == ShapeType::Triangle, Message::SetShapeType(ShapeType::Triangle), colors),
                shape_icon_toggle("shape_square", shape_type == ShapeType::Rectangle, Message::SetShapeType(ShapeType::Rectangle), colors),
                shape_icon_toggle("shape_pentagon", shape_type == ShapeType::Pentagon, Message::SetShapeType(ShapeType::Pentagon), colors),
            ]
            .spacing(2)
            .into(),
        );

        items.push(
            row![
                shape_icon_toggle("shape_hexagon", shape_type == ShapeType::Hexagon, Message::SetShapeType(ShapeType::Hexagon), colors),
                shape_icon_toggle("shape_circle", shape_type == ShapeType::Circle, Message::SetShapeType(ShapeType::Circle), colors),
                shape_icon_toggle("shape_polygon", is_polygon, Message::TogglePolygonSubmenu, colors),
            ]
            .spacing(2)
            .into(),
        );

        if polygon_submenu_open {
            items.push(
                row![
                    small_shape_button("7", ShapeType::Heptagon, shape_type == ShapeType::Heptagon, colors),
                    small_shape_button("8", ShapeType::Octagon, shape_type == ShapeType::Octagon, colors),
                    small_shape_button("9", ShapeType::Nonagon, shape_type == ShapeType::Nonagon, colors),
                    small_shape_button("10", ShapeType::Decagon, shape_type == ShapeType::Decagon, colors),
                    small_shape_button("11", ShapeType::Hendecagon, shape_type == ShapeType::Hendecagon, colors),
                    small_shape_button("12", ShapeType::Dodecagon, shape_type == ShapeType::Dodecagon, colors),
                ]
                .spacing(2)
                .into(),
            );
        }

        if shape_type == ShapeType::Rectangle {
            items.push(
                row![
                    text("Skew").size(11),
                    slider(0.0..=60.0, skew_angle, Message::SetSkewAngle).step(1.0),
                    text(format!("{:.0}\u{00b0}", skew_angle)).size(11),
                ]
                .spacing(4)
                .align_y(iced::Alignment::Center)
                .into(),
            );
        }
    }

    // --- Stroke/Fill color selection (stays in tool config) ---
    items.push(Space::new().height(4).into());

    // Stroke color row
    let stroke_expanded = palette_target == Some(PaletteTarget::Stroke);
    let stroke_preview: Element<'a, Message> = if let Some(c) = style.stroke_color {
        color_swatch_button(c, stroke_expanded, Message::SetPaletteTarget(PaletteTarget::Stroke), colors)
    } else {
        none_swatch_button(stroke_expanded, Message::SetPaletteTarget(PaletteTarget::Stroke), colors)
    };
    items.push(
        row![
            icon("style_stroke", SMALL_ICON, colors.icon_color),
            stroke_preview,
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center)
        .into(),
    );

    if stroke_expanded {
        build_color_pick_swatches(&mut items, palette, stroke_color_index, reorder_mode, reorder_src, colors);
    }

    // Fill color row
    let fill_expanded = palette_target == Some(PaletteTarget::Fill);
    let fill_preview: Element<'a, Message> = if let Some(fill) = style.fill_color {
        color_swatch_button(fill, fill_expanded, Message::SetPaletteTarget(PaletteTarget::Fill), colors)
    } else {
        none_swatch_button(fill_expanded, Message::SetPaletteTarget(PaletteTarget::Fill), colors)
    };
    items.push(
        row![
            icon("style_fill", SMALL_ICON, colors.icon_color),
            fill_preview,
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center)
        .into(),
    );

    if fill_expanded {
        build_color_pick_swatches(&mut items, palette, fill_color_index, reorder_mode, reorder_src, colors);
    }

    items
}

// ─── Palette Panel ───

fn build_palette_panel<'a>(
    palette: &Palette,
    palette_slug: &str,
    palette_status: &'a str,
    reorder_mode: bool,
    reorder_src: Option<usize>,
    colors: EditorColors,
    color_picker_target: Option<usize>,
    picker_r: f32,
    picker_g: f32,
    picker_b: f32,
) -> Vec<Element<'a, Message>> {
    let mut items: Vec<Element<'a, Message>> = Vec::new();

    // Palette header
    items.push(
        row![
            text(format!("{}", palette.name)).size(12),
            icon_toggle("palette_reorder", reorder_mode, Message::PaletteReorderToggle, colors),
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center)
        .into(),
    );

    if reorder_mode {
        items.push(
            text(if reorder_src.is_some() { "Click to place" } else { "Click to pick up" })
                .size(10)
                .into(),
        );
    }

    // Palette swatches — clicking opens color picker for editing
    build_palette_edit_swatches(&mut items, palette, color_picker_target, reorder_mode, reorder_src, colors);

    // Add color button
    items.push(
        row![
            text_button("+ Add Color", Message::AddPaletteColor, colors),
        ]
        .spacing(4)
        .into(),
    );

    // Color picker (when editing a color)
    if let Some(idx) = color_picker_target {
        items.push(Space::new().height(4).into());

        // Preview swatch
        let preview_color = Color::from_rgb(picker_r, picker_g, picker_b);
        let pc = preview_color;
        items.push(
            container(text("").size(1))
                .width(160)
                .height(30)
                .style(move |_theme| container::Style {
                    background: Some(iced::Background::Color(pc)),
                    border: iced::Border {
                        radius: 4.0.into(),
                        width: 1.0,
                        color: Color::from_rgb(0.5, 0.5, 0.5),
                    },
                    ..Default::default()
                })
                .into(),
        );

        // R slider
        items.push(
            row![
                text("R").size(10),
                slider(0.0..=1.0, picker_r, Message::ColorPickerR).step(0.005),
                text(format!("{:.0}", picker_r * 255.0)).size(10),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center)
            .into(),
        );

        // G slider
        items.push(
            row![
                text("G").size(10),
                slider(0.0..=1.0, picker_g, Message::ColorPickerG).step(0.005),
                text(format!("{:.0}", picker_g * 255.0)).size(10),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center)
            .into(),
        );

        // B slider
        items.push(
            row![
                text("B").size(10),
                slider(0.0..=1.0, picker_b, Message::ColorPickerB).step(0.005),
                text(format!("{:.0}", picker_b * 255.0)).size(10),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center)
            .into(),
        );

        // Hex display
        let hex = format!(
            "#{:02x}{:02x}{:02x}",
            (picker_r * 255.0) as u8,
            (picker_g * 255.0) as u8,
            (picker_b * 255.0) as u8,
        );
        items.push(text(hex).size(10).into());

        // Apply / Cancel / Delete
        items.push(
            row![
                text_button("Apply", Message::ColorPickerApply, colors),
                text_button("Cancel", Message::ColorPickerCancel, colors),
                text_button("Delete", Message::DeletePaletteColor(idx), colors),
            ]
            .spacing(4)
            .into(),
        );
    }

    // Lospec import
    items.push(Space::new().height(4).into());
    items.push(
        row![
            text_input("slug...", palette_slug)
                .on_input(Message::PaletteSlugChanged)
                .size(11),
            icon_toggle("palette_import", false, Message::ImportPalette, colors),
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center)
        .into(),
    );

    if !palette_status.is_empty() {
        items.push(text(palette_status).size(10).into());
    }

    // Default palette controls
    items.push(Space::new().height(4).into());
    items.push(
        row![
            text_button("Reset Default", Message::ResetPalette, colors),
            text_button("Set Default", Message::SetAsDefaultPalette, colors),
        ]
        .spacing(4)
        .into(),
    );

    items
}

// ─── Settings Panel ───

fn build_settings_panel<'a>(
    theme_mode: ThemeMode,
    grid: &GridConfig,
    colors: EditorColors,
    base_text_size: f32,
    settings_color_field: Option<&str>,
    picker_r: f32,
    picker_g: f32,
    picker_b: f32,
) -> Vec<Element<'a, Message>> {
    let mut items: Vec<Element<'a, Message>> = Vec::new();

    // Theme mode toggle
    items.push(
        row![
            icon_toggle("action_dark_mode", theme_mode == ThemeMode::Dark, Message::SetThemeMode(ThemeMode::Dark), colors),
            icon_toggle("action_light_mode", theme_mode == ThemeMode::Light, Message::SetThemeMode(ThemeMode::Light), colors),
        ]
        .spacing(2)
        .into(),
    );

    // Custom theme colors
    items.push(Space::new().height(4).into());
    items.push(text("Theme Colors").size(11).into());

    for &(field_name, label) in EditorColors::editable_fields() {
        let c = colors.get_field(field_name);
        let is_editing = settings_color_field == Some(field_name);
        let bc = if is_editing { colors.swatch_border_selected } else { colors.swatch_border };
        let bw = if is_editing { 2.0 } else { 1.0 };
        let field_owned = field_name.to_string();

        items.push(
            row![
                button(text("").size(1))
                    .width(22)
                    .height(22)
                    .style(move |_theme, _status| button::Style {
                        background: Some(iced::Background::Color(c)),
                        border: iced::Border {
                            width: bw,
                            color: bc,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(Message::EditThemeColor(field_owned)),
                text(label).size(10),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center)
            .into(),
        );
    }

    // Settings color picker
    if let Some(_field) = settings_color_field {
        let preview_color = Color::from_rgb(picker_r, picker_g, picker_b);
        let pc = preview_color;
        items.push(
            container(text("").size(1))
                .width(160)
                .height(20)
                .style(move |_theme| container::Style {
                    background: Some(iced::Background::Color(pc)),
                    border: iced::Border {
                        radius: 4.0.into(),
                        width: 1.0,
                        color: Color::from_rgb(0.5, 0.5, 0.5),
                    },
                    ..Default::default()
                })
                .into(),
        );

        items.push(
            row![
                text("R").size(10),
                slider(0.0..=1.0, picker_r, Message::SettingsPickerR).step(0.005),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center)
            .into(),
        );
        items.push(
            row![
                text("G").size(10),
                slider(0.0..=1.0, picker_g, Message::SettingsPickerG).step(0.005),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center)
            .into(),
        );
        items.push(
            row![
                text("B").size(10),
                slider(0.0..=1.0, picker_b, Message::SettingsPickerB).step(0.005),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center)
            .into(),
        );

        items.push(
            row![
                text_button("Apply", Message::SettingsPickerApply, colors),
                text_button("Cancel", Message::SettingsPickerCancel, colors),
            ]
            .spacing(4)
            .into(),
        );
    }

    items.push(
        text_button("Reset Colors", Message::ResetThemeColors, colors),
    );

    // Base text size
    items.push(Space::new().height(4).into());
    items.push(
        row![
            text("Text Size").size(11),
            slider(8.0..=20.0, base_text_size, Message::SetBaseTextSize).step(1.0),
            text(format!("{:.0}", base_text_size)).size(11),
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center)
        .into(),
    );

    // Grid section
    items.push(Space::new().height(4).into());
    items.push(text("Grid").size(11).into());

    let vis_icon = if grid.visible { "grid_visible" } else { "grid_off" };
    let snap_icon = if grid.snap { "grid_snap" } else { "grid_snap_off" };
    items.push(
        row![
            icon_toggle(vis_icon, grid.visible, Message::ToggleGridVisible(!grid.visible), colors),
            icon_toggle(snap_icon, grid.snap, Message::ToggleGridSnap(!grid.snap), colors),
        ]
        .spacing(2)
        .into(),
    );

    items.push(
        row![
            icon_toggle("grid_lines", grid.style == GridStyle::Lines, Message::SetGridStyle(GridStyle::Lines), colors),
            icon_toggle("grid_dots", grid.style == GridStyle::Dots, Message::SetGridStyle(GridStyle::Dots), colors),
            icon_toggle("grid_iso", grid.style == GridStyle::Isometric, Message::SetGridStyle(GridStyle::Isometric), colors),
        ]
        .spacing(2)
        .into(),
    );

    // Grid size: slider over power-of-2 exponents (1,2,4,8,...,128)
    // Exponent 0..7 maps to 2^0=1 .. 2^7=128
    let grid_exp = (grid.size as f64).log2().round() as u32;
    let grid_exp_f32 = grid_exp as f32;
    items.push(
        row![
            text("Size").size(11),
            slider(0.0..=7.0, grid_exp_f32, |v| {
                Message::SetGridSize(2.0_f32.powi(v.round() as i32))
            }).step(1.0),
            text(format!("{}", grid.size as u32)).size(11),
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center)
        .into(),
    );

    items
}

// ─── Shared swatch helpers ───

/// Swatches for picking stroke/fill colors (used in ToolConfig mode).
/// Clicking selects the color for stroke or fill.
fn build_color_pick_swatches<'a>(
    items: &mut Vec<Element<'a, Message>>,
    palette: &Palette,
    selected_index: Option<usize>,
    reorder_mode: bool,
    reorder_src: Option<usize>,
    colors: EditorColors,
) {
    let total = palette.colors.len() + 1;
    let mut swatch_rows: Vec<Element<'a, Message>> = Vec::new();
    let mut swatch_elements: Vec<Element<'a, Message>> = Vec::new();

    for i in 0..total {
        let is_selected = if i == 0 {
            selected_index.is_none()
        } else {
            selected_index == Some(i)
        };
        let is_picked_up = reorder_src == Some(i);

        let border_color = if is_picked_up {
            colors.swatch_border_picked_up
        } else if reorder_mode && reorder_src.is_some() && i > 0 && reorder_src != Some(i) {
            colors.swatch_border_drop_target
        } else if is_selected {
            colors.swatch_border_selected
        } else {
            colors.swatch_border
        };
        let border_width = if is_picked_up || (reorder_mode && reorder_src.is_some() && i > 0) || is_selected {
            2.0
        } else {
            1.0
        };

        let on_press = if reorder_mode {
            if i == 0 {
                Message::PaletteColorClicked(0)
            } else if reorder_src.is_some() {
                Message::PaletteReorderDrop(i)
            } else {
                Message::PaletteReorderPickUp(i)
            }
        } else {
            Message::PaletteColorClicked(i)
        };

        if i == 0 {
            let bc = border_color;
            let none_bg = colors.swatch_none_bg;
            let icon_color = colors.swatch_none_text;
            swatch_elements.push(
                button(icon("none", 18.0, icon_color))
                .width(22)
                .height(22)
                .padding(2)
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(none_bg)),
                    border: iced::Border {
                        width: border_width,
                        color: bc,
                        radius: 2.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(on_press)
                .into(),
            );
        } else {
            let c = palette.colors[i - 1];
            let bc = border_color;
            let display_color = c;
            swatch_elements.push(
                button(text("").size(1))
                    .width(22)
                    .height(22)
                    .style(move |_theme, _status| button::Style {
                        background: Some(iced::Background::Color(display_color)),
                        border: iced::Border {
                            width: border_width,
                            color: bc,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(on_press)
                    .into(),
            );
        }

        if (i + 1) % 6 == 0 || i == total - 1 {
            let row_items: Vec<Element<'a, Message>> = swatch_elements.drain(..).collect();
            swatch_rows.push(row(row_items).spacing(2).into());
        }
    }

    if reorder_mode && reorder_src.is_some() {
        let end_idx = total;
        let end_bg = colors.end_drop_bg;
        let end_text = colors.end_drop_text;
        let end_border = colors.end_drop_border;
        swatch_rows.push(
            button(
                container(text("+").size(10))
                    .center_x(22)
                    .center_y(22),
            )
            .width(22)
            .height(22)
            .style(move |_theme, _status| button::Style {
                background: Some(iced::Background::Color(end_bg)),
                text_color: end_text,
                border: iced::Border {
                    width: 2.0,
                    color: end_border,
                    radius: 2.0.into(),
                },
                ..Default::default()
            })
            .on_press(Message::PaletteReorderDrop(end_idx))
            .into(),
        );
    }

    for swatch_row in swatch_rows {
        items.push(swatch_row);
    }
}

/// Swatches for the palette management panel.
/// Clicking opens the color picker for editing that color.
fn build_palette_edit_swatches<'a>(
    items: &mut Vec<Element<'a, Message>>,
    palette: &Palette,
    editing_index: Option<usize>,
    reorder_mode: bool,
    reorder_src: Option<usize>,
    colors: EditorColors,
) {
    let total = palette.colors.len();
    let mut swatch_rows: Vec<Element<'a, Message>> = Vec::new();
    let mut swatch_elements: Vec<Element<'a, Message>> = Vec::new();

    for i in 0..total {
        let ui_idx = i + 1; // 1-based
        let is_editing = editing_index == Some(ui_idx);
        let is_picked_up = reorder_src == Some(ui_idx);

        let border_color = if is_picked_up {
            colors.swatch_border_picked_up
        } else if reorder_mode && reorder_src.is_some() && reorder_src != Some(ui_idx) {
            colors.swatch_border_drop_target
        } else if is_editing {
            colors.swatch_border_selected
        } else {
            colors.swatch_border
        };
        let border_width = if is_picked_up || (reorder_mode && reorder_src.is_some()) || is_editing {
            2.0
        } else {
            1.0
        };

        let on_press = if reorder_mode {
            if reorder_src.is_some() {
                Message::PaletteReorderDrop(ui_idx)
            } else {
                Message::PaletteReorderPickUp(ui_idx)
            }
        } else {
            Message::EditPaletteColor(ui_idx)
        };

        let c = palette.colors[i];
        let bc = border_color;
        let display_color = c;
        swatch_elements.push(
            button(text("").size(1))
                .width(22)
                .height(22)
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(display_color)),
                    border: iced::Border {
                        width: border_width,
                        color: bc,
                        radius: 2.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(on_press)
                .into(),
        );

        if (i + 1) % 6 == 0 || i == total - 1 {
            let row_items: Vec<Element<'a, Message>> = swatch_elements.drain(..).collect();
            swatch_rows.push(row(row_items).spacing(2).into());
        }
    }

    if reorder_mode && reorder_src.is_some() {
        let end_idx = total + 1;
        let end_bg = colors.end_drop_bg;
        let end_text = colors.end_drop_text;
        let end_border = colors.end_drop_border;
        swatch_rows.push(
            button(
                container(text("+").size(10))
                    .center_x(22)
                    .center_y(22),
            )
            .width(22)
            .height(22)
            .style(move |_theme, _status| button::Style {
                background: Some(iced::Background::Color(end_bg)),
                text_color: end_text,
                border: iced::Border {
                    width: 2.0,
                    color: end_border,
                    radius: 2.0.into(),
                },
                ..Default::default()
            })
            .on_press(Message::PaletteReorderDrop(end_idx))
            .into(),
        );
    }

    for swatch_row in swatch_rows {
        items.push(swatch_row);
    }
}

fn color_swatch_button<'a>(color: Color, expanded: bool, on_press: Message, colors: EditorColors) -> Element<'a, Message> {
    let c = color;
    let border_color = if expanded { colors.swatch_border_selected } else { colors.swatch_border };
    let border_width = if expanded { 2.0 } else { 1.0 };
    button(text("").size(1))
        .width(22)
        .height(22)
        .style(move |_theme, _status| button::Style {
            background: Some(iced::Background::Color(c)),
            border: iced::Border {
                width: border_width,
                color: border_color,
                radius: 2.0.into(),
            },
            ..Default::default()
        })
        .on_press(on_press)
        .into()
}

fn none_swatch_button<'a>(expanded: bool, on_press: Message, colors: EditorColors) -> Element<'a, Message> {
    let border_color = if expanded { colors.swatch_border_selected } else { colors.swatch_border };
    let border_width = if expanded { 2.0 } else { 1.0 };
    let none_bg = colors.swatch_none_bg;
    let icon_color = colors.swatch_none_text;
    button(icon("none", 18.0, icon_color))
    .width(22)
    .height(22)
    .padding(2)
    .style(move |_theme, _status| button::Style {
        background: Some(iced::Background::Color(none_bg)),
        border: iced::Border {
            width: border_width,
            color: border_color,
            radius: 2.0.into(),
        },
        ..Default::default()
    })
    .on_press(on_press)
    .into()
}
