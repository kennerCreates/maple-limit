use svg::node::element::{self, path::Data};
use svg::Document;

use crate::document::Document as EditorDocument;
use crate::shape::{polygon_vertices, ShapeItem};

pub fn export_svg(doc: &EditorDocument, width: f32, height: f32) -> svg::Document {
    let mut svg_doc = Document::new().set("viewBox", (0, 0, width as i32, height as i32));

    for shape in &doc.shapes {
        match shape {
            ShapeItem::Circle {
                center,
                radius,
                style,
            } => {
                let mut circle = element::Circle::new()
                    .set("cx", center.x)
                    .set("cy", center.y)
                    .set("r", *radius)
                    .set("stroke", color_to_svg(style.stroke_color))
                    .set("stroke-width", style.stroke_width);
                if let Some(fill) = style.fill_color {
                    circle = circle.set("fill", color_to_svg(fill));
                } else {
                    circle = circle.set("fill", "none");
                }
                svg_doc = svg_doc.add(circle);
            }
            ShapeItem::Rectangle {
                top_left,
                size,
                style,
            } => {
                let mut rect = element::Rectangle::new()
                    .set("x", top_left.x)
                    .set("y", top_left.y)
                    .set("width", size.width)
                    .set("height", size.height)
                    .set("stroke", color_to_svg(style.stroke_color))
                    .set("stroke-width", style.stroke_width);
                if let Some(fill) = style.fill_color {
                    rect = rect.set("fill", color_to_svg(fill));
                } else {
                    rect = rect.set("fill", "none");
                }
                svg_doc = svg_doc.add(rect);
            }
            ShapeItem::RegularPolygon {
                center,
                radius,
                sides,
                rotation,
                style,
            } => {
                let verts = polygon_vertices(*center, *radius, *sides, *rotation);
                let points: String = verts
                    .iter()
                    .map(|v| format!("{:.2},{:.2}", v.x, v.y))
                    .collect::<Vec<_>>()
                    .join(" ");
                let mut polygon = element::Polygon::new()
                    .set("points", points)
                    .set("stroke", color_to_svg(style.stroke_color))
                    .set("stroke-width", style.stroke_width);
                if let Some(fill) = style.fill_color {
                    polygon = polygon.set("fill", color_to_svg(fill));
                } else {
                    polygon = polygon.set("fill", "none");
                }
                svg_doc = svg_doc.add(polygon);
            }
            ShapeItem::Line { start, end, style } => {
                let line = element::Line::new()
                    .set("x1", start.x)
                    .set("y1", start.y)
                    .set("x2", end.x)
                    .set("y2", end.y)
                    .set("stroke", color_to_svg(style.stroke_color))
                    .set("stroke-width", style.stroke_width);
                svg_doc = svg_doc.add(line);
            }
            ShapeItem::Spline { segments, style } => {
                if segments.is_empty() {
                    continue;
                }
                let mut data = Data::new().move_to((
                    segments[0].start.x,
                    segments[0].start.y,
                ));
                for seg in segments {
                    data = data.cubic_curve_to((
                        seg.control_a.x,
                        seg.control_a.y,
                        seg.control_b.x,
                        seg.control_b.y,
                        seg.end.x,
                        seg.end.y,
                    ));
                }
                let mut path = element::Path::new()
                    .set("d", data)
                    .set("stroke", color_to_svg(style.stroke_color))
                    .set("stroke-width", style.stroke_width);
                if let Some(fill) = style.fill_color {
                    path = path.set("fill", color_to_svg(fill));
                } else {
                    path = path.set("fill", "none");
                }
                svg_doc = svg_doc.add(path);
            }
        }
    }

    svg_doc
}

fn color_to_svg(color: iced::Color) -> String {
    let [r, g, b, _a] = color.into_rgba8();
    format!("rgb({},{},{})", r, g, b)
}
