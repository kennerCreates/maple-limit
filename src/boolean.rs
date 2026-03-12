use iced::widget::canvas::{Frame, Path, Stroke};
use iced::{Point, Renderer};

use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::float::single::SingleFloatOverlay;

use crate::shape::{polygon_vertices, ShapeItem, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoolOp {
    Union,
    Intersection,
    Difference,
    Xor,
}

impl std::fmt::Display for BoolOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoolOp::Union => write!(f, "Union"),
            BoolOp::Intersection => write!(f, "Intersection"),
            BoolOp::Difference => write!(f, "Difference"),
            BoolOp::Xor => write!(f, "Xor"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BooleanGroup {
    pub op: BoolOp,
    pub shape_a: usize,
    pub shape_b: usize,
    pub style: Style,
    pub cached_result: Vec<Vec<[f32; 2]>>,
}

/// Convert a closed ShapeItem to a polygon (list of [x, y] vertices).
/// Returns None for open paths (Line, Polyline, Spline).
pub fn shape_to_polygon(shape: &ShapeItem) -> Option<Vec<[f32; 2]>> {
    match shape {
        ShapeItem::Circle { center, radius, .. } => {
            let n = 64;
            let verts: Vec<[f32; 2]> = (0..n)
                .map(|i| {
                    let angle = 2.0 * std::f32::consts::PI * i as f32 / n as f32;
                    [center.x + radius * angle.cos(), center.y + radius * angle.sin()]
                })
                .collect();
            Some(verts)
        }
        ShapeItem::Rectangle { top_left, size, corner_radius, style } => {
            if *corner_radius > 0.0 {
                let r = corner_radius.min(size.width / 2.0).min(size.height / 2.0);
                let x = top_left.x;
                let y = top_left.y;
                let w = size.width;
                let h = size.height;

                if style.line_join == crate::shape::LineJoin::Bevel {
                    // Chamfered rectangle - 8 vertices
                    Some(vec![
                        [x + r, y],
                        [x + w - r, y],
                        [x + w, y + r],
                        [x + w, y + h - r],
                        [x + w - r, y + h],
                        [x + r, y + h],
                        [x, y + h - r],
                        [x, y + r],
                    ])
                } else {
                    // Rounded rectangle - approximate curves with segments
                    let segments_per_corner = 8;
                    let mut verts = Vec::new();
                    let k = 0.5522847498_f32 * r;

                    // Sample each corner's bezier curve
                    let corners: [(Point, Point, Point); 4] = [
                        // Top-right
                        (Point::new(x + w - r + k, y), Point::new(x + w, y + r - k), Point::new(x + w, y + r)),
                        // Bottom-right
                        (Point::new(x + w, y + h - r + k), Point::new(x + w - r + k, y + h), Point::new(x + w - r, y + h)),
                        // Bottom-left
                        (Point::new(x + r - k, y + h), Point::new(x, y + h - r + k), Point::new(x, y + h - r)),
                        // Top-left
                        (Point::new(x, y + r - k), Point::new(x + r - k, y), Point::new(x + r, y)),
                    ];
                    let starts = [
                        Point::new(x + w - r, y),
                        Point::new(x + w, y + h - r),
                        Point::new(x + r, y + h),
                        Point::new(x, y + r),
                    ];

                    for (i, (c1, c2, end)) in corners.iter().enumerate() {
                        let start = starts[i];
                        verts.push([start.x, start.y]);
                        for j in 1..=segments_per_corner {
                            let t = j as f32 / segments_per_corner as f32;
                            let u = 1.0 - t;
                            let px = u*u*u*start.x + 3.0*u*u*t*c1.x + 3.0*u*t*t*c2.x + t*t*t*end.x;
                            let py = u*u*u*start.y + 3.0*u*u*t*c1.y + 3.0*u*t*t*c2.y + t*t*t*end.y;
                            verts.push([px, py]);
                        }
                    }
                    Some(verts)
                }
            } else {
                // Simple rectangle - 4 corners
                let x = top_left.x;
                let y = top_left.y;
                Some(vec![
                    [x, y],
                    [x + size.width, y],
                    [x + size.width, y + size.height],
                    [x, y + size.height],
                ])
            }
        }
        ShapeItem::RegularPolygon { center, radius, sides, rotation, .. } => {
            let pts = polygon_vertices(*center, *radius, *sides, *rotation);
            Some(pts.iter().map(|p| [p.x, p.y]).collect())
        }
        ShapeItem::RightTriangle { origin, width, height, .. } => {
            Some(vec![
                [origin.x, origin.y],
                [origin.x + width, origin.y],
                [origin.x, origin.y + height],
            ])
        }
        ShapeItem::Line { .. } | ShapeItem::Polyline { .. } | ShapeItem::Spline { .. } => None,
    }
}

/// Returns true if the shape is a closed shape that can participate in boolean ops.
pub fn is_closed_shape(shape: &ShapeItem) -> bool {
    matches!(
        shape,
        ShapeItem::Circle { .. }
            | ShapeItem::Rectangle { .. }
            | ShapeItem::RegularPolygon { .. }
            | ShapeItem::RightTriangle { .. }
    )
}

fn overlay_rule(op: BoolOp) -> OverlayRule {
    match op {
        BoolOp::Union => OverlayRule::Union,
        BoolOp::Intersection => OverlayRule::Intersect,
        BoolOp::Difference => OverlayRule::Difference,
        BoolOp::Xor => OverlayRule::Xor,
    }
}

/// Compute the boolean operation between two polygons.
pub fn compute_boolean(a: &[[f32; 2]], b: &[[f32; 2]], op: BoolOp) -> Vec<Vec<[f32; 2]>> {
    let subject = vec![a.to_vec()];
    let clip = vec![b.to_vec()];
    let result = subject.overlay(&clip, overlay_rule(op), FillRule::EvenOdd);
    // i_overlay returns Vec<Vec<Vec<[f32; 2]>>> (shapes with contours)
    // Flatten to Vec<Vec<[f32; 2]>> (list of contours)
    result.into_iter().flatten().collect()
}

/// Paint boolean result contours onto the canvas.
pub fn paint_boolean_result(frame: &mut Frame<Renderer>, contours: &[Vec<[f32; 2]>], style: &Style, opacity: f32) {
    if contours.is_empty() {
        return;
    }

    let path = Path::new(|builder| {
        for contour in contours {
            if let Some(first) = contour.first() {
                builder.move_to(Point::new(first[0], first[1]));
                for pt in &contour[1..] {
                    builder.line_to(Point::new(pt[0], pt[1]));
                }
                builder.close();
            }
        }
    });

    if let Some(fill) = style.fill_color {
        let fill = if opacity < 1.0 {
            iced::Color { a: fill.a * opacity, ..fill }
        } else {
            fill
        };
        frame.fill(&path, fill);
    }
    if let Some(stroke_color) = style.stroke_color {
        let color = if opacity < 1.0 {
            iced::Color { a: stroke_color.a * opacity, ..stroke_color }
        } else {
            stroke_color
        };
        let stroke = Stroke::default()
            .with_color(color)
            .with_width(style.stroke_width)
            .with_line_cap(style.line_cap.to_canvas())
            .with_line_join(style.line_join.to_canvas());
        frame.stroke(&path, stroke);
    }
}

/// Hit test a point against boolean result contours.
pub fn hit_test_contours(point: Point, contours: &[Vec<[f32; 2]>]) -> bool {
    for contour in contours {
        let vertices: Vec<Point> = contour.iter().map(|p| Point::new(p[0], p[1])).collect();
        if point_in_polygon_f(point, &vertices) {
            return true;
        }
    }
    false
}

fn point_in_polygon_f(p: Point, vertices: &[Point]) -> bool {
    let mut inside = false;
    let n = vertices.len();
    if n < 3 {
        return false;
    }
    let mut j = n - 1;
    for i in 0..n {
        let vi = vertices[i];
        let vj = vertices[j];
        if ((vi.y > p.y) != (vj.y > p.y))
            && (p.x < (vj.x - vi.x) * (p.y - vi.y) / (vj.y - vi.y) + vi.x)
        {
            inside = !inside;
        }
        j = i;
    }
    inside
}
