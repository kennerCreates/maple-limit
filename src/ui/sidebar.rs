use iced::widget::{button, container, image, row, slider, text, text_input, Column, Space};
use iced::{Background, Color, Element};

use crate::app::{Message, PaletteTarget};
use crate::grid::{GridConfig, GridStyle};
use crate::palette::Palette;
use crate::shape::{LineCap, LineJoin, ShapeItem, Style};
use crate::theme::EditorColors;
use crate::tool::{ShapeType, Tool};

const ICON: f32 = 20.0;
const SMALL_ICON: f32 = 18.0;

fn icon(name: &str, size: f32) -> image::Image<image::Handle> {
    image::Image::new(image::Handle::from_path(format!("assets/icons/{}.png", name)))
        .width(size)
        .height(size)
}

fn icon_toggle<'a>(
    icon_name: &str,
    active: bool,
    on_press: Message,
    colors: EditorColors,
) -> Element<'a, Message> {
    let active_bg = colors.panel_button_active;
    let hover_bg = colors.panel_button_hover;
    button(icon(icon_name, ICON))
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

pub fn view<'a>(
    active_tool: Tool,
    style: &Style,
    shape_type: ShapeType,
    skew_angle: f32,
    palette: &Palette,
    palette_slug: &str,
    grid: &GridConfig,
    selected_shape: Option<&ShapeItem>,
    palette_target: Option<PaletteTarget>,
    stroke_color_index: Option<usize>,
    fill_color_index: Option<usize>,
    reorder_mode: bool,
    reorder_src: Option<usize>,
    colors: EditorColors,
) -> Element<'a, Message> {
    let mut items: Vec<Element<'a, Message>> = Vec::new();

    // --- Selected shape editing ---
    if active_tool == Tool::Select {
        if let Some(shape) = selected_shape {
            let s = shape.style();

            // Stroke width: icon + slider + value
            items.push(
                row![
                    icon("style_stroke", ICON),
                    slider(0.0..=20.0, s.stroke_width, Message::SetSelectedStrokeWidth).step(0.5),
                    text(format!("{:.1}", s.stroke_width)).size(11),
                ]
                .spacing(4)
                .align_y(iced::Alignment::Center)
                .into(),
            );

            // Corner radius (rectangles only)
            if let Some(cr) = shape.corner_radius() {
                items.push(
                    row![
                        icon("style_corner", ICON),
                        slider(0.0..=100.0, cr, Message::SetSelectedCornerRadius).step(1.0),
                        text(format!("{:.0}", cr)).size(11),
                    ]
                    .spacing(4)
                    .align_y(iced::Alignment::Center)
                    .into(),
                );
            }

            // Line cap: icon buttons
            items.push(
                row![
                    icon_toggle("cap_butt", s.line_cap == LineCap::Butt, Message::SetSelectedLineCap(LineCap::Butt), colors),
                    icon_toggle("cap_round", s.line_cap == LineCap::Round, Message::SetSelectedLineCap(LineCap::Round), colors),
                    icon_toggle("cap_square", s.line_cap == LineCap::Square, Message::SetSelectedLineCap(LineCap::Square), colors),
                ]
                .spacing(2)
                .into(),
            );

            // Line join: icon buttons
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
        // Stroke width: icon + slider + value
        items.push(
            row![
                icon("style_stroke", ICON),
                slider(0.0..=20.0, style.stroke_width, Message::SetStrokeWidth).step(0.5),
                text(format!("{:.1}", style.stroke_width)).size(11),
            ]
            .spacing(4)
            .align_y(iced::Alignment::Center)
            .into(),
        );
    }

    // Shape tool config
    if active_tool == Tool::Shape {
        // Primary shapes: Triangle, Rectangle, Pentagon, Hexagon, Circle
        items.push(
            row![
                icon_toggle("shape_triangle", shape_type == ShapeType::Triangle, Message::SetShapeType(ShapeType::Triangle), colors),
                icon_toggle("shape_square", shape_type == ShapeType::Rectangle, Message::SetShapeType(ShapeType::Rectangle), colors),
                icon_toggle("shape_pentagon", shape_type == ShapeType::Pentagon, Message::SetShapeType(ShapeType::Pentagon), colors),
                icon_toggle("shape_hexagon", shape_type == ShapeType::Hexagon, Message::SetShapeType(ShapeType::Hexagon), colors),
                icon_toggle("shape_circle", shape_type == ShapeType::Circle, Message::SetShapeType(ShapeType::Circle), colors),
            ]
            .spacing(2)
            .into(),
        );

        // Secondary shapes: 7-12 sided polygons
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

        // Skew angle slider for Rectangle
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

    // --- Palette section ---
    items.push(Space::new().height(4).into());

    // Palette header with reorder icon
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

    // Stroke color row
    let stroke_expanded = palette_target == Some(PaletteTarget::Stroke);
    let stroke_preview: Element<'a, Message> = if let Some(c) = style.stroke_color {
        color_swatch_button(c, stroke_expanded, Message::SetPaletteTarget(PaletteTarget::Stroke), colors)
    } else {
        none_swatch_button(stroke_expanded, Message::SetPaletteTarget(PaletteTarget::Stroke), colors)
    };
    items.push(
        row![
            icon("style_stroke", SMALL_ICON),
            stroke_preview,
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center)
        .into(),
    );

    if stroke_expanded {
        build_palette_swatches(&mut items, palette, stroke_color_index, reorder_mode, reorder_src, colors);
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
            icon("style_fill", SMALL_ICON),
            fill_preview,
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center)
        .into(),
    );

    if fill_expanded {
        build_palette_swatches(&mut items, palette, fill_color_index, reorder_mode, reorder_src, colors);
    }

    // Import palette
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

    // --- Grid section ---
    items.push(Space::new().height(4).into());

    // Grid toggles: visible + snap
    let vis_icon = if grid.visible { "grid_visible" } else { "grid_off" };
    let snap_icon = if grid.snap { "grid_snap_on" } else { "grid_snap_off" };
    items.push(
        row![
            icon_toggle(vis_icon, grid.visible, Message::ToggleGridVisible(!grid.visible), colors),
            icon_toggle(snap_icon, grid.snap, Message::ToggleGridSnap(!grid.snap), colors),
        ]
        .spacing(2)
        .into(),
    );

    // Grid style: icon buttons
    items.push(
        row![
            icon_toggle("grid_lines", grid.style == GridStyle::Lines, Message::SetGridStyle(GridStyle::Lines), colors),
            icon_toggle("grid_dots", grid.style == GridStyle::Dots, Message::SetGridStyle(GridStyle::Dots), colors),
            icon_toggle("grid_iso", grid.style == GridStyle::Isometric, Message::SetGridStyle(GridStyle::Isometric), colors),
        ]
        .spacing(2)
        .into(),
    );

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

fn build_palette_swatches<'a>(
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
            let none_text = colors.swatch_none_text;
            swatch_elements.push(
                button(
                    container(text("X").size(10))
                        .center_x(22)
                        .center_y(22),
                )
                .width(22)
                .height(22)
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(none_bg)),
                    text_color: none_text,
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
            let opacity = if is_picked_up { 0.5 } else { 1.0 };
            let display_color = Color::from_rgba(c.r, c.g, c.b, opacity);
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
    let none_text = colors.swatch_none_text;
    button(
        container(text("X").size(10))
            .center_x(22)
            .center_y(22),
    )
    .width(22)
    .height(22)
    .style(move |_theme, _status| button::Style {
        background: Some(iced::Background::Color(none_bg)),
        text_color: none_text,
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
