use svg::node::element::{self, path::Data};
use svg::Document;

use crate::document::Document as EditorDocument;
use crate::shape::{polygon_vertices, LineCap, LineJoin, ShapeItem};

macro_rules! apply_style {
    ($node:expr, $style:expr) => {{
        let mut n = $node;
        if let Some(stroke) = $style.stroke_color {
            n = n.set("stroke", color_to_svg(stroke))
                .set("stroke-width", $style.stroke_width);
        } else {
            n = n.set("stroke", "none");
        }
        if let Some(fill) = $style.fill_color {
            n = n.set("fill", color_to_svg(fill));
        } else {
            n = n.set("fill", "none");
        }
        match $style.line_cap {
            LineCap::Butt => {}
            LineCap::Round => { n = n.set("stroke-linecap", "round"); }
            LineCap::Square => { n = n.set("stroke-linecap", "square"); }
        }
        match $style.line_join {
            LineJoin::Miter => {}
            LineJoin::Round => { n = n.set("stroke-linejoin", "round"); }
            LineJoin::Bevel => { n = n.set("stroke-linejoin", "bevel"); }
        }
        n
    }};
}

pub fn export_svg(doc: &EditorDocument, width: f32, height: f32) -> svg::Document {
    let mut svg_doc = Document::new().set("viewBox", (0, 0, width as i32, height as i32));

    for shape in &doc.shapes {
        let style = shape.style();
        match shape {
            ShapeItem::Circle {
                center, radius, ..
            } => {
                let circle = element::Circle::new()
                    .set("cx", center.x)
                    .set("cy", center.y)
                    .set("r", *radius);
                let circle = apply_style!(circle, style);
                svg_doc = svg_doc.add(circle);
            }
            ShapeItem::Rectangle {
                top_left,
                size,
                corner_radius,
                ..
            } => {
                if *corner_radius > 0.0 && style.line_join == LineJoin::Bevel {
                    // Export chamfered rectangle as a polygon path
                    let r = corner_radius.min(size.width / 2.0).min(size.height / 2.0);
                    let x = top_left.x;
                    let y = top_left.y;
                    let w = size.width;
                    let h = size.height;
                    let data = Data::new()
                        .move_to((x + r, y))
                        .line_to((x + w - r, y))
                        .line_to((x + w, y + r))
                        .line_to((x + w, y + h - r))
                        .line_to((x + w - r, y + h))
                        .line_to((x + r, y + h))
                        .line_to((x, y + h - r))
                        .line_to((x, y + r))
                        .close();
                    let path = element::Path::new().set("d", data);
                    let path = apply_style!(path, style);
                    svg_doc = svg_doc.add(path);
                } else {
                    let mut rect = element::Rectangle::new()
                        .set("x", top_left.x)
                        .set("y", top_left.y)
                        .set("width", size.width)
                        .set("height", size.height);
                    if *corner_radius > 0.0 {
                        rect = rect.set("rx", *corner_radius);
                    }
                    let rect = apply_style!(rect, style);
                    svg_doc = svg_doc.add(rect);
                }
            }
            ShapeItem::RegularPolygon {
                center,
                radius,
                sides,
                rotation,
                ..
            } => {
                let verts = polygon_vertices(*center, *radius, *sides, *rotation);
                let points: String = verts
                    .iter()
                    .map(|v| format!("{:.2},{:.2}", v.x, v.y))
                    .collect::<Vec<_>>()
                    .join(" ");
                let polygon = element::Polygon::new()
                    .set("points", points);
                let polygon = apply_style!(polygon, style);
                svg_doc = svg_doc.add(polygon);
            }
            ShapeItem::RightTriangle {
                origin,
                width,
                height,
                ..
            } => {
                let points = format!(
                    "{:.2},{:.2} {:.2},{:.2} {:.2},{:.2}",
                    origin.x, origin.y,
                    origin.x + width, origin.y,
                    origin.x, origin.y + height,
                );
                let polygon = element::Polygon::new()
                    .set("points", points);
                let polygon = apply_style!(polygon, style);
                svg_doc = svg_doc.add(polygon);
            }
            ShapeItem::Line { start, end, .. } => {
                let line = element::Line::new()
                    .set("x1", start.x)
                    .set("y1", start.y)
                    .set("x2", end.x)
                    .set("y2", end.y);
                let line = apply_style!(line, style);
                svg_doc = svg_doc.add(line);
            }
            ShapeItem::Polyline { points, .. } => {
                if points.len() < 2 {
                    continue;
                }
                let pts: String = points
                    .iter()
                    .map(|p| format!("{:.2},{:.2}", p.x, p.y))
                    .collect::<Vec<_>>()
                    .join(" ");
                let polyline = element::Polyline::new()
                    .set("points", pts);
                let polyline = apply_style!(polyline, style);
                svg_doc = svg_doc.add(polyline);
            }
            ShapeItem::Spline { segments, .. } => {
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
                let path = element::Path::new()
                    .set("d", data);
                let path = apply_style!(path, style);
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
