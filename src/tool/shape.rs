use iced::{Point, Size};

use crate::shape::ShapeItem;
use super::{ShapeType, ToolEvent, ToolPreview, ToolResult, ToolState};

pub fn handle(state: &mut ToolState, event: ToolEvent) -> ToolResult {
    match event {
        ToolEvent::Press(pos, _) => {
            state.drag_start = Some(pos);
            state.drag_current = Some(pos);
            ToolResult::RequestRedraw
        }
        ToolEvent::Drag(pos) | ToolEvent::Move(pos) => {
            if state.drag_start.is_some() {
                state.drag_current = Some(pos);
                ToolResult::RequestRedraw
            } else {
                ToolResult::None
            }
        }
        ToolEvent::Release(pos) => {
            if let Some(start) = state.drag_start.take() {
                state.drag_current = None;
                if let Some(shape) = build_shape(start, pos, state) {
                    ToolResult::ShapeCompleted(shape)
                } else {
                    ToolResult::None
                }
            } else {
                ToolResult::None
            }
        }
        _ => ToolResult::None,
    }
}

fn build_shape(start: Point, end: Point, state: &ToolState) -> Option<ShapeItem> {
    let style = state.current_style.clone();

    match state.shape_type {
        ShapeType::Circle => {
            let dx = end.x - start.x;
            let dy = end.y - start.y;
            let radius = (dx * dx + dy * dy).sqrt();
            if radius > 1.0 {
                Some(ShapeItem::Circle { center: start, radius, style })
            } else {
                None
            }
        }
        ShapeType::Rectangle => {
            let (tl, size) = rect_from_points(start, end);
            if size.width > 1.0 && size.height > 1.0 {
                if state.skew_angle > 0.0 {
                    let angle_rad = state.skew_angle.to_radians();
                    let skew = size.height * angle_rad.tan();
                    let bl = tl;
                    let br = Point::new(tl.x + size.width, tl.y);
                    let tr = Point::new(tl.x + size.width + skew, tl.y + size.height);
                    let tl_skewed = Point::new(tl.x + skew, tl.y + size.height);
                    Some(ShapeItem::Polyline {
                        points: vec![bl, br, tr, tl_skewed, bl],
                        style,
                    })
                } else {
                    Some(ShapeItem::Rectangle { top_left: tl, size, corner_radius: 0.0, style })
                }
            } else {
                None
            }
        }
        polygon_type => {
            // Regular polygon (3-12 sides)
            let sides = polygon_type.sides().unwrap_or(6);
            let dx = end.x - start.x;
            let dy = end.y - start.y;
            let radius = (dx * dx + dy * dy).sqrt();
            if radius > 1.0 {
                let rotation = dy.atan2(dx) + std::f32::consts::FRAC_PI_2;
                Some(ShapeItem::RegularPolygon {
                    center: start,
                    radius,
                    sides,
                    rotation,
                    style,
                })
            } else {
                None
            }
        }
    }
}

pub fn preview(state: &ToolState) -> ToolPreview {
    if let (Some(start), Some(current)) = (state.drag_start, state.drag_current) {
        if let Some(shape) = build_shape(start, current, state) {
            ToolPreview::Shape(shape)
        } else {
            ToolPreview::None
        }
    } else {
        ToolPreview::None
    }
}

fn rect_from_points(a: Point, b: Point) -> (Point, Size) {
    let x = a.x.min(b.x);
    let y = a.y.min(b.y);
    let w = (a.x - b.x).abs();
    let h = (a.y - b.y).abs();
    (Point::new(x, y), Size::new(w, h))
}
