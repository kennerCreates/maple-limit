use iced::widget::canvas::{Frame, Path, Stroke};
use iced::{Color, Point, Renderer, Size};

#[derive(Debug, Clone)]
pub struct Style {
    pub stroke_color: Color,
    pub stroke_width: f32,
    pub fill_color: Option<Color>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            stroke_color: Color::BLACK,
            stroke_width: 2.0,
            fill_color: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SplineSegment {
    pub start: Point,
    pub control_a: Point,
    pub control_b: Point,
    pub end: Point,
}

#[derive(Debug, Clone)]
pub enum ShapeItem {
    Circle {
        center: Point,
        radius: f32,
        style: Style,
    },
    Rectangle {
        top_left: Point,
        size: Size,
        style: Style,
    },
    RegularPolygon {
        center: Point,
        radius: f32,
        sides: usize,
        rotation: f32,
        style: Style,
    },
    Line {
        start: Point,
        end: Point,
        style: Style,
    },
    Spline {
        segments: Vec<SplineSegment>,
        style: Style,
    },
}

impl ShapeItem {
    pub fn style(&self) -> &Style {
        match self {
            ShapeItem::Circle { style, .. } => style,
            ShapeItem::Rectangle { style, .. } => style,
            ShapeItem::RegularPolygon { style, .. } => style,
            ShapeItem::Line { style, .. } => style,
            ShapeItem::Spline { style, .. } => style,
        }
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        match self {
            ShapeItem::Circle { center, .. } => {
                center.x += dx;
                center.y += dy;
            }
            ShapeItem::Rectangle { top_left, .. } => {
                top_left.x += dx;
                top_left.y += dy;
            }
            ShapeItem::RegularPolygon { center, .. } => {
                center.x += dx;
                center.y += dy;
            }
            ShapeItem::Line { start, end, .. } => {
                start.x += dx;
                start.y += dy;
                end.x += dx;
                end.y += dy;
            }
            ShapeItem::Spline { segments, .. } => {
                for seg in segments {
                    seg.start.x += dx;
                    seg.start.y += dy;
                    seg.control_a.x += dx;
                    seg.control_a.y += dy;
                    seg.control_b.x += dx;
                    seg.control_b.y += dy;
                    seg.end.x += dx;
                    seg.end.y += dy;
                }
            }
        }
    }

    pub fn paint(&self, frame: &mut Frame<Renderer>) {
        let style = self.style();
        let stroke = Stroke::default()
            .with_color(style.stroke_color)
            .with_width(style.stroke_width);

        match self {
            ShapeItem::Circle { center, radius, .. } => {
                let path = Path::circle(*center, *radius);
                if let Some(fill) = style.fill_color {
                    frame.fill(&path, fill);
                }
                frame.stroke(&path, stroke);
            }
            ShapeItem::Rectangle { top_left, size, .. } => {
                let path = Path::rectangle(*top_left, *size);
                if let Some(fill) = style.fill_color {
                    frame.fill(&path, fill);
                }
                frame.stroke(&path, stroke);
            }
            ShapeItem::RegularPolygon {
                center,
                radius,
                sides,
                rotation,
                ..
            } => {
                let path = polygon_path(*center, *radius, *sides, *rotation);
                if let Some(fill) = style.fill_color {
                    frame.fill(&path, fill);
                }
                frame.stroke(&path, stroke);
            }
            ShapeItem::Line { start, end, .. } => {
                let path = Path::line(*start, *end);
                frame.stroke(&path, stroke);
            }
            ShapeItem::Spline { segments, .. } => {
                if segments.is_empty() {
                    return;
                }
                let path = Path::new(|builder| {
                    builder.move_to(segments[0].start);
                    for seg in segments {
                        builder.bezier_curve_to(seg.control_a, seg.control_b, seg.end);
                    }
                });
                if let Some(fill) = style.fill_color {
                    frame.fill(&path, fill);
                }
                frame.stroke(&path, stroke);
            }
        }
    }

    pub fn hit_test(&self, point: Point) -> bool {
        let threshold = 6.0;
        match self {
            ShapeItem::Circle { center, radius, .. } => {
                let dx = point.x - center.x;
                let dy = point.y - center.y;
                let dist = (dx * dx + dy * dy).sqrt();
                (dist - radius).abs() < threshold || dist < *radius
            }
            ShapeItem::Rectangle { top_left, size, .. } => {
                point.x >= top_left.x
                    && point.x <= top_left.x + size.width
                    && point.y >= top_left.y
                    && point.y <= top_left.y + size.height
            }
            ShapeItem::RegularPolygon {
                center,
                radius,
                sides,
                rotation,
                ..
            } => {
                let vertices = polygon_vertices(*center, *radius, *sides, *rotation);
                point_in_polygon(point, &vertices)
            }
            ShapeItem::Line { start, end, .. } => {
                point_to_line_dist(point, *start, *end) < threshold
            }
            ShapeItem::Spline { segments, .. } => {
                for seg in segments {
                    if point_near_bezier(point, seg, threshold) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

pub fn polygon_vertices(center: Point, radius: f32, sides: usize, rotation: f32) -> Vec<Point> {
    (0..sides)
        .map(|i| {
            let angle =
                rotation + (2.0 * std::f32::consts::PI * i as f32) / sides as f32 - std::f32::consts::FRAC_PI_2;
            Point::new(center.x + radius * angle.cos(), center.y + radius * angle.sin())
        })
        .collect()
}

pub fn polygon_path(center: Point, radius: f32, sides: usize, rotation: f32) -> Path {
    let verts = polygon_vertices(center, radius, sides, rotation);
    Path::new(|builder| {
        if let Some(first) = verts.first() {
            builder.move_to(*first);
            for v in &verts[1..] {
                builder.line_to(*v);
            }
            builder.close();
        }
    })
}

fn point_to_line_dist(p: Point, a: Point, b: Point) -> f32 {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let len_sq = dx * dx + dy * dy;
    if len_sq < 0.0001 {
        return ((p.x - a.x).powi(2) + (p.y - a.y).powi(2)).sqrt();
    }
    let t = ((p.x - a.x) * dx + (p.y - a.y) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);
    let proj_x = a.x + t * dx;
    let proj_y = a.y + t * dy;
    ((p.x - proj_x).powi(2) + (p.y - proj_y).powi(2)).sqrt()
}

fn point_in_polygon(p: Point, vertices: &[Point]) -> bool {
    let mut inside = false;
    let n = vertices.len();
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

fn point_near_bezier(p: Point, seg: &SplineSegment, threshold: f32) -> bool {
    // Sample the bezier curve and check distance to each sample
    let samples = 20;
    for i in 0..samples {
        let t = i as f32 / samples as f32;
        let bp = bezier_point(t, seg);
        let dist = ((p.x - bp.x).powi(2) + (p.y - bp.y).powi(2)).sqrt();
        if dist < threshold {
            return true;
        }
    }
    false
}

pub fn bezier_point(t: f32, seg: &SplineSegment) -> Point {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;
    Point::new(
        uuu * seg.start.x
            + 3.0 * uu * t * seg.control_a.x
            + 3.0 * u * tt * seg.control_b.x
            + ttt * seg.end.x,
        uuu * seg.start.y
            + 3.0 * uu * t * seg.control_a.y
            + 3.0 * u * tt * seg.control_b.y
            + ttt * seg.end.y,
    )
}
