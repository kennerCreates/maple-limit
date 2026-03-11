use iced::widget::{button, checkbox, container, pick_list, row, slider, text, text_input, Column};
use iced::{Color, Element};

use crate::app::{Message, PaletteTarget};
use crate::grid::{GridConfig, GridStyle};
use crate::palette::Palette;
use crate::shape::{LineCap, LineJoin, ShapeItem, Style};
use crate::tool::Tool;

pub fn view<'a>(
    active_tool: Tool,
    style: &Style,
    shape_sides: usize,
    right_triangle: bool,
    palette: &Palette,
    palette_slug: &str,
    grid: &GridConfig,
    selected_shape: Option<&ShapeItem>,
    palette_target: Option<PaletteTarget>,
    stroke_color_index: Option<usize>,
    fill_color_index: Option<usize>,
    reorder_mode: bool,
    reorder_src: Option<usize>,
) -> Element<'a, Message> {
    let mut items: Vec<Element<'a, Message>> = Vec::new();

    // --- Selected shape editing ---
    if active_tool == Tool::Select {
        if let Some(shape) = selected_shape {
            items.push(text("Edit Shape").size(16).into());

            let s = shape.style();

            // Stroke width
            items.push(text(format!("Stroke: {:.1}", s.stroke_width)).size(13).into());
            items.push(
                slider(0.5..=20.0, s.stroke_width, Message::SetSelectedStrokeWidth)
                    .step(0.5)
                    .into(),
            );

            // Corner radius (only for rectangles)
            if let Some(cr) = shape.corner_radius() {
                items.push(text(format!("Corner Radius: {:.0}", cr)).size(13).into());
                items.push(
                    slider(0.0..=100.0, cr, Message::SetSelectedCornerRadius)
                        .step(1.0)
                        .into(),
                );
            }

            // Line cap
            items.push(text("Line Cap").size(13).into());
            items.push(
                pick_list(
                    LineCap::ALL,
                    Some(s.line_cap),
                    Message::SetSelectedLineCap,
                )
                .text_size(13)
                .into(),
            );

            // Line join
            items.push(text("Line Join").size(13).into());
            items.push(
                pick_list(
                    LineJoin::ALL,
                    Some(s.line_join),
                    Message::SetSelectedLineJoin,
                )
                .text_size(13)
                .into(),
            );

            items.push(text("").size(8).into()); // spacer
        } else {
            items.push(text("No shape selected").size(13).into());
            items.push(text("").size(4).into());
        }
    }

    // --- Tool style (for new shapes) ---
    if active_tool != Tool::Select {
        items.push(text("Style").size(16).into());

        // Stroke width
        items.push(text(format!("Stroke: {:.1}", style.stroke_width)).size(13).into());
        items.push(
            slider(0.5..=20.0, style.stroke_width, Message::SetStrokeWidth)
                .step(0.5)
                .into(),
        );
    }

    // Shape tool config
    if active_tool == Tool::Shape {
        let sides_label = match shape_sides {
            3 => "Triangle (3)".to_string(),
            4 => "Square (4)".to_string(),
            5 => "Pentagon (5)".to_string(),
            6 => "Hexagon (6)".to_string(),
            s if s >= 64 => "Circle".to_string(),
            s => format!("{}-gon", s),
        };
        items.push(text(format!("Sides: {}", sides_label)).size(13).into());
        items.push(
            slider(3.0..=64.0, shape_sides as f32, |v| {
                Message::SetShapeSides(v as usize)
            })
            .step(1.0)
            .into(),
        );

        // Right triangle toggle
        if shape_sides == 3 {
            items.push(
                checkbox(right_triangle)
                    .label("Right Triangle")
                    .on_toggle(Message::SetRightTriangle)
                    .size(16)
                    .into(),
            );
        }
    }

    // --- Palette section ---
    items.push(text("").size(8).into()); // spacer

    // Palette header with reorder toggle
    items.push(
        row![
            text(format!("Palette: {}", palette.name)).size(14),
            button(text(if reorder_mode { "Done" } else { "Reorder" }).size(10))
                .on_press(Message::PaletteReorderToggle)
                .style(if reorder_mode { button::primary } else { button::secondary }),
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center)
        .into(),
    );

    if reorder_mode {
        items.push(
            text(if reorder_src.is_some() {
                "Click a position to place"
            } else {
                "Click a color to pick up"
            })
            .size(11)
            .into(),
        );
    }

    // Stroke color button with preview - expands/collapses palette
    let stroke_expanded = palette_target == Some(PaletteTarget::Stroke);
    let stroke_btn_style = if stroke_expanded { button::primary } else { button::secondary };
    items.push(
        row![
            button(text("Stroke").size(12))
                .on_press(Message::SetPaletteTarget(PaletteTarget::Stroke))
                .style(stroke_btn_style),
            color_preview(style.stroke_color),
            text(format!("#{}", stroke_color_index.map_or("—".to_string(), |i| i.to_string()))).size(11),
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center)
        .into(),
    );

    // Show palette grid when stroke is expanded
    if stroke_expanded {
        build_palette_swatches(&mut items, palette, stroke_color_index, reorder_mode, reorder_src);
    }

    // Fill color button with preview - expands/collapses palette
    let fill_expanded = palette_target == Some(PaletteTarget::Fill);
    let fill_btn_style = if fill_expanded { button::primary } else { button::secondary };
    let fill_preview: Element<'a, Message> = if let Some(fill) = style.fill_color {
        color_preview(fill)
    } else {
        none_preview()
    };
    items.push(
        row![
            button(text("Fill").size(12))
                .on_press(Message::SetPaletteTarget(PaletteTarget::Fill))
                .style(fill_btn_style),
            fill_preview,
            text(fill_color_index.map_or("None".to_string(), |i| format!("#{}", i))).size(11),
        ]
        .spacing(4)
        .align_y(iced::Alignment::Center)
        .into(),
    );

    // Show palette grid when fill is expanded
    if fill_expanded {
        build_palette_swatches(&mut items, palette, fill_color_index, reorder_mode, reorder_src);
    }

    // Import palette
    items.push(text("").size(4).into());
    items.push(text("Lospec Import").size(13).into());
    items.push(
        text_input("palette slug...", palette_slug)
            .on_input(Message::PaletteSlugChanged)
            .size(13)
            .into(),
    );
    items.push(
        button(text("Import").size(13))
            .on_press(Message::ImportPalette)
            .style(button::secondary)
            .into(),
    );

    // --- Grid section ---
    items.push(text("").size(8).into());
    items.push(text("Grid").size(16).into());

    items.push(
        checkbox(grid.visible)
            .label("Visible")
            .on_toggle(Message::ToggleGridVisible)
            .size(16)
            .into(),
    );

    items.push(
        checkbox(grid.snap)
            .label("Snap to Grid")
            .on_toggle(Message::ToggleGridSnap)
            .size(16)
            .into(),
    );

    items.push(text("Style").size(13).into());
    items.push(
        pick_list(
            GridStyle::ALL,
            Some(grid.style),
            Message::SetGridStyle,
        )
        .text_size(13)
        .into(),
    );

    items.push(text(format!("Size: {:.0}", grid.size)).size(13).into());
    items.push(
        slider(5.0..=100.0, grid.size, Message::SetGridSize)
            .step(5.0)
            .into(),
    );

    container(
        Column::with_children(items)
            .spacing(6)
            .padding(10)
            .width(180),
    )
    .into()
}

fn build_palette_swatches<'a>(
    items: &mut Vec<Element<'a, Message>>,
    palette: &Palette,
    selected_index: Option<usize>,
    reorder_mode: bool,
    reorder_src: Option<usize>,
) {
    let total = palette.colors.len() + 1; // +1 for "None" at index 0
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
            Color::from_rgb(1.0, 0.6, 0.0) // orange for picked-up
        } else if reorder_mode && reorder_src.is_some() && i > 0 && reorder_src != Some(i) {
            Color::from_rgb(0.0, 0.8, 0.0) // green for drop targets
        } else if is_selected {
            Color::from_rgb(0.0, 0.5, 1.0)
        } else {
            Color::from_rgb(0.3, 0.3, 0.3)
        };
        let border_width = if is_picked_up || (reorder_mode && reorder_src.is_some() && i > 0) {
            2.0
        } else if is_selected {
            2.0
        } else {
            1.0
        };

        // Determine what clicking this swatch does
        let on_press = if reorder_mode {
            if i == 0 {
                // Can't pick up or drop on None
                Message::PaletteColorClicked(0) // just select None
            } else if reorder_src.is_some() {
                // We have a color picked up - drop it here
                Message::PaletteReorderDrop(i)
            } else {
                // Pick up this color
                Message::PaletteReorderPickUp(i)
            }
        } else {
            Message::PaletteColorClicked(i)
        };

        if i == 0 {
            // "None" swatch
            let bc = border_color;
            swatch_elements.push(
                button(
                    container(text("X").size(10))
                        .center_x(22)
                        .center_y(22),
                )
                .width(22)
                .height(22)
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(Color::WHITE)),
                    text_color: Color::from_rgb(0.7, 0.0, 0.0),
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

        if (i + 1) % 4 == 0 || i == total - 1 {
            let row_items: Vec<Element<'a, Message>> = swatch_elements.drain(..).collect();
            swatch_rows.push(row(row_items).spacing(2).into());
        }
    }

    // If in reorder mode with a picked-up color, add an "End" drop target
    if reorder_mode && reorder_src.is_some() {
        let end_idx = total; // one past the last
        swatch_rows.push(
            button(
                container(text("+").size(10))
                    .center_x(22)
                    .center_y(22),
            )
            .width(22)
            .height(22)
            .style(move |_theme, _status| button::Style {
                background: Some(iced::Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
                text_color: Color::from_rgb(0.0, 0.6, 0.0),
                border: iced::Border {
                    width: 2.0,
                    color: Color::from_rgb(0.0, 0.8, 0.0),
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

fn color_preview<'a>(color: Color) -> Element<'a, Message> {
    let c = color;
    container(text("").size(1))
        .width(20)
        .height(20)
        .style(move |_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(c)),
            border: iced::Border {
                width: 1.0,
                color: Color::from_rgb(0.3, 0.3, 0.3),
                radius: 2.0.into(),
            },
            ..Default::default()
        })
        .into()
}

fn none_preview<'a>() -> Element<'a, Message> {
    container(text("X").size(9))
        .width(20)
        .height(20)
        .center_x(20)
        .center_y(20)
        .style(|_theme: &iced::Theme| container::Style {
            background: Some(iced::Background::Color(Color::WHITE)),
            border: iced::Border {
                width: 1.0,
                color: Color::from_rgb(0.3, 0.3, 0.3),
                radius: 2.0.into(),
            },
            ..Default::default()
        })
        .into()
}
