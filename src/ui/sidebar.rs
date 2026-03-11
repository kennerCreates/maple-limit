use iced::widget::{button, container, row, slider, text, text_input, Column};
use iced::{Color, Element};

use crate::app::Message;
use crate::palette::Palette;
use crate::shape::Style;
use crate::tool::Tool;

pub fn view<'a>(
    active_tool: Tool,
    style: &Style,
    polygon_sides: usize,
    palette: &Palette,
    palette_slug: &str,
) -> Element<'a, Message> {
    let mut items: Vec<Element<'a, Message>> = Vec::new();

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

    // Polygon sides
    if active_tool == Tool::RegularPolygon {
        items.push(text(format!("Sides: {}", polygon_sides)).size(13).into());
        items.push(
            slider(3.0..=12.0, polygon_sides as f32, |v| {
                Message::SetPolygonSides(v as usize)
            })
            .step(1.0)
            .into(),
        );
    }

    // Palette section
    items.push(text("").size(8).into()); // spacer
    items.push(text(format!("Palette: {}", palette.name)).size(14).into());

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
