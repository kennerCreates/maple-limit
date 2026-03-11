use iced::widget::canvas::{self, Frame, Path, Stroke};
use iced::{Color, Point, Renderer, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

impl LineCap {
    pub fn to_canvas(self) -> canvas::LineCap {
        match self {
            LineCap::Butt => canvas::LineCap::Butt,
            LineCap::Round => canvas::LineCap::Round,
            LineCap::Square => canvas::LineCap::Square,
        }
    }
}

impl std::fmt::Display for LineCap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineCap::Butt => write!(f, "Butt"),
            LineCap::Round => write!(f, "Round"),
            LineCap::Square => write!(f, "Square"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

impl LineJoin {
    pub fn to_canvas(self) -> canvas::LineJoin {
        match self {
            LineJoin::Miter => canvas::LineJoin::Miter,
            LineJoin::Round => canvas::LineJoin::Round,
            LineJoin::Bevel => canvas::LineJoin::Bevel,
        }
    }
}

impl std::fmt::Display for LineJoin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LineJoin::Miter => write!(f, "Miter"),
            LineJoin::Round => write!(f, "Round"),
            LineJoin::Bevel => write!(f, "Bevel"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Style {
    pub stroke_color: Option<Color>,
    pub stroke_width: f32,
    pub fill_color: Option<Color>,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            stroke_color: Some(Color::BLACK),
            stroke_width: 2.0,
            fill_color: None,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
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
#[allow(dead_code)]
pub enum ShapeItem {
    Circle {
        center: Point,
        radius: f32,
        style: Style,
    },
    Rectangle {
        top_left: Point,
        size: Size,
        corner_radius: f32,
        style: Style,
    },
    RegularPolygon {
        center: Point,
        radius: f32,
        sides: usize,
        rotation: f32,
        style: Style,
    },
    RightTriangle {
        origin: Point,
        width: f32,
        height: f32,
        style: Style,
    },
    Line {
        start: Point,
        end: Point,
        style: Style,
    },
    Polyline {
        points: Vec<Point>,
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
            ShapeItem::RightTriangle { style, .. } => style,
            ShapeItem::Line { style, .. } => style,
            ShapeItem::Polyline { style, .. } => style,
            ShapeItem::Spline { style, .. } => style,
        }
    }

    pub fn style_mut(&mut self) -> &mut Style {
        match self {
            ShapeItem::Circle { style, .. } => style,
            ShapeItem::Rectangle { style, .. } => style,
            ShapeItem::RegularPolygon { style, .. } => style,
            ShapeItem::RightTriangle { style, .. } => style,
            ShapeItem::Line { style, .. } => style,
            ShapeItem::Polyline { style, .. } => style,
            ShapeItem::Spline { style, .. } => style,
        }
    }

    pub fn corner_radius(&self) -> Option<f32> {
        match self {
            ShapeItem::Rectangle { corner_radius, .. } => Some(*corner_radius),
            _ => None,
        }
    }

    pub fn set_corner_radius(&mut self, r: f32) {
        if let ShapeItem::Rectangle { corner_radius, .. } = self {
            *corner_radius = r;
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
            ShapeItem::RightTriangle { origin, .. } => {
                origin.x += dx;
                origin.y += dy;
            }
            ShapeItem::Line { start, end, .. } => {
                start.x += dx;
                start.y += dy;
                end.x += dx;
                end.y += dy;
            }
            ShapeItem::Polyline { points, .. } => {
                for p in points {
                    p.x += dx;
                    p.y += dy;
                }
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
        let stroke = style.stroke_color.map(|color| {
            Stroke::default()
                .with_color(color)
                .with_width(style.stroke_width)
                .with_line_cap(style.line_cap.to_canvas())
                .with_line_join(style.line_join.to_canvas())
        });

        match self {
            ShapeItem::Circle { center, radius, .. } => {
                let path = Path::circle(*center, *radius);
                if let Some(fill) = style.fill_color {
                    frame.fill(&path, fill);
                }
                if let Some(ref s) = stroke { frame.stroke(&path, s.clone()); }
            }
            ShapeItem::Rectangle { top_left, size, corner_radius, .. } => {
                if *corner_radius > 0.0 {
                    let path = rounded_rect_path(*top_left, *size, *corner_radius);
                    if let Some(fill) = style.fill_color {
                        frame.fill(&path, fill);
                    }
                    if let Some(ref s) = stroke { frame.stroke(&path, s.clone()); }
                } else {
                    let path = Path::rectangle(*top_left, *size);
                    if let Some(fill) = style.fill_color {
                        frame.fill(&path, fill);
                    }
                    if let Some(ref s) = stroke { frame.stroke(&path, s.clone()); }
                }
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
                if let Some(ref s) = stroke { frame.stroke(&path, s.clone()); }
            }
            ShapeItem::RightTriangle { origin, width, height, .. } => {
                let verts = [
                    *origin,
                    Point::new(origin.x + width, origin.y),
                    Point::new(origin.x, origin.y + height),
                ];
                let path = Path::new(|builder| {
                    builder.move_to(verts[0]);
                    builder.line_to(verts[1]);
                    builder.line_to(verts[2]);
                    builder.close();
                });
                if let Some(fill) = style.fill_color {
                    frame.fill(&path, fill);
                }
                if let Some(ref s) = stroke { frame.stroke(&path, s.clone()); }
            }
            ShapeItem::Line { start, end, .. } => {
                let path = Path::line(*start, *end);
                if let Some(ref s) = stroke { frame.stroke(&path, s.clone()); }
            }
            ShapeItem::Polyline { points, .. } => {
                if points.len() >= 2 {
                    let path = Path::new(|builder| {
                        builder.move_to(points[0]);
                        for p in &points[1..] {
                            builder.line_to(*p);
                        }
                    });
                    if let Some(fill) = style.fill_color {
                        frame.fill(&path, fill);
                    }
                    if let Some(ref s) = stroke { frame.stroke(&path, s.clone()); }
                }
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
                if let Some(ref s) = stroke { frame.stroke(&path, s.clone()); }
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
            ShapeItem::RightTriangle { origin, width, height, .. } => {
                let vertices = [
                    *origin,
                    Point::new(origin.x + width, origin.y),
                    Point::new(origin.x, origin.y + height),
                ];
                point_in_polygon(point, &vertices)
            }
            ShapeItem::Line { start, end, .. } => {
                point_to_line_dist(point, *start, *end) < threshold
            }
            ShapeItem::Polyline { points, .. } => {
                for i in 0..points.len().saturating_sub(1) {
                    if point_to_line_dist(point, points[i], points[i + 1]) < threshold {
                        return true;
                    }
                }
                false
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

fn rounded_rect_path(top_left: Point, size: Size, radius: f32) -> Path {
    // Clamp radius so it doesn't exceed half the smallest dimension
    let r = radius.min(size.width / 2.0).min(size.height / 2.0);
    let x = top_left.x;
    let y = top_left.y;
    let w = size.width;
    let h = size.height;

    // Approximate quarter circle with cubic bezier
    // Magic number for circle approximation: 0.5522847498
    let k = 0.5522847498 * r;

    Path::new(|builder| {
        // Start at top-left + radius
        builder.move_to(Point::new(x + r, y));
        // Top edge
        builder.line_to(Point::new(x + w - r, y));
        // Top-right corner
        builder.bezier_curve_to(
            Point::new(x + w - r + k, y),
            Point::new(x + w, y + r - k),
            Point::new(x + w, y + r),
        );
        // Right edge
        builder.line_to(Point::new(x + w, y + h - r));
        // Bottom-right corner
        builder.bezier_curve_to(
            Point::new(x + w, y + h - r + k),
            Point::new(x + w - r + k, y + h),
            Point::new(x + w - r, y + h),
        );
        // Bottom edge
        builder.line_to(Point::new(x + r, y + h));
        // Bottom-left corner
        builder.bezier_curve_to(
            Point::new(x + r - k, y + h),
            Point::new(x, y + h - r + k),
            Point::new(x, y + h - r),
        );
        // Left edge
        builder.line_to(Point::new(x, y + r));
        // Top-left corner
        builder.bezier_curve_to(
            Point::new(x, y + r - k),
            Point::new(x + r - k, y),
            Point::new(x + r, y),
        );
        builder.close();
    })
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
