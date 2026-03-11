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
    palette_target: PaletteTarget,
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

            // Stroke color - show swatches inline for quick picking
            items.push(text("Stroke Color").size(13).into());
            items.push(color_preview(s.stroke_color));
            // Quick stroke color buttons
            {
                let stroke_colors = [
                    ("Blk", Color::BLACK),
                    ("Wht", Color::WHITE),
                    ("Red", Color::from_rgb(1.0, 0.0, 0.0)),
                    ("Blu", Color::from_rgb(0.0, 0.0, 1.0)),
                ];
                let mut stroke_btns: Vec<Element<'a, Message>> = Vec::new();
                for (label, color) in stroke_colors {
                    let c = color;
                    stroke_btns.push(
                        button(text(label).size(9))
                            .width(32)
                            .height(18)
                            .style(move |_theme, _status| button::Style {
                                background: Some(iced::Background::Color(c)),
                                text_color: if c.r + c.g + c.b > 1.5 { Color::BLACK } else { Color::WHITE },
                                border: iced::Border {
                                    width: 1.0,
                                    color: Color::from_rgb(0.3, 0.3, 0.3),
                                    radius: 2.0.into(),
                                },
                                ..Default::default()
                            })
                            .on_press(Message::SetSelectedStrokeColor(color))
                            .into(),
                    );
                }
                items.push(row(stroke_btns).spacing(2).into());
            }

            // Fill color
            items.push(text("Fill Color").size(13).into());
            if let Some(fill) = s.fill_color {
                items.push(
                    row![
                        color_preview(fill),
                        button(text("Clear").size(11))
                            .on_press(Message::SetSelectedFill(None))
                            .style(button::secondary),
                    ]
                    .spacing(4)
                    .into(),
                );
            } else {
                items.push(text("None (click palette to set)").size(12).into());
            }

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

        // Stroke color preview
        items.push(text("Stroke Color").size(13).into());
        items.push(color_preview(style.stroke_color));

        // Fill color preview
        items.push(text("Fill Color").size(13).into());
        if let Some(fill) = style.fill_color {
            items.push(
                row![
                    color_preview(fill),
                    button(text("Clear").size(11))
                        .on_press(Message::ClearFillColor)
                        .style(button::secondary),
                ]
                .spacing(4)
                .into(),
            );
        } else {
            items.push(text("None").size(12).into());
        }
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
    items.push(text(format!("Palette: {}", palette.name)).size(14).into());

    // Palette target toggle (Fill vs Stroke)
    items.push(
        row![
            button(text("Fill").size(12))
                .on_press(Message::SetPaletteTarget(PaletteTarget::Fill))
                .style(if palette_target == PaletteTarget::Fill { button::primary } else { button::secondary }),
            button(text("Stroke").size(12))
                .on_press(Message::SetPaletteTarget(PaletteTarget::Stroke))
                .style(if palette_target == PaletteTarget::Stroke { button::primary } else { button::secondary }),
        ]
        .spacing(4)
        .into(),
    );

    // Palette swatches in rows of 4
    let swatch_count = palette.colors.len();
    let mut swatch_rows: Vec<Element<'a, Message>> = Vec::new();
    let mut swatch_elements: Vec<Element<'a, Message>> = Vec::new();
    for (i, color) in palette.colors.iter().enumerate() {
        let c = *color;
        swatch_elements.push(
            button(text("").size(1))
                .width(22)
                .height(22)
                .style(move |_theme, _status| button::Style {
                    background: Some(iced::Background::Color(c)),
                    border: iced::Border {
                        width: 1.0,
                        color: Color::from_rgb(0.3, 0.3, 0.3),
                        radius: 2.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(Message::PaletteColorClicked(c))
                .into(),
        );

        if (i + 1) % 4 == 0 || i == swatch_count - 1 {
            let row_items: Vec<Element<'a, Message>> = swatch_elements.drain(..).collect();
            swatch_rows.push(row(row_items).spacing(2).into());
        }
    }

    for swatch_row in swatch_rows {
        items.push(swatch_row);
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

fn color_preview<'a>(color: Color) -> Element<'a, Message> {
    let c = color;
    container(text("").size(1))
        .width(40)
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
