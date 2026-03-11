use iced::Point;

use crate::shape::ShapeItem;
use super::{ToolEvent, ToolPreview, ToolResult, ToolState};

const CIRCLE_SIDES: usize = 64;

pub fn handle(state: &mut ToolState, event: ToolEvent) -> ToolResult {
    match event {
        ToolEvent::Press(pos) => {
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
            if let Some(center) = state.drag_start.take() {
                state.drag_current = None;
                let dx = pos.x - center.x;
                let dy = pos.y - center.y;
                let radius = (dx * dx + dy * dy).sqrt();
                if radius > 1.0 {
                    let shape = build_shape(center, pos, radius, state);
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

fn build_shape(center: Point, drag_end: Point, radius: f32, state: &ToolState) -> ShapeItem {
    let sides = state.shape_sides;
    let style = state.current_style.clone();

    // Right triangle mode
    if sides == 3 && state.right_triangle {
        let width = drag_end.x - center.x;
        let height = drag_end.y - center.y;
        return ShapeItem::RightTriangle {
            origin: center,
            width,
            height,
            style,
        };
    }

    // Circle at max sides
    if sides >= CIRCLE_SIDES {
        return ShapeItem::Circle {
            center,
            radius,
            style,
        };
    }

    // Regular polygon
    let rotation = (drag_end.y - center.y).atan2(drag_end.x - center.x)
        + std::f32::consts::FRAC_PI_2;
    ShapeItem::RegularPolygon {
        center,
        radius,
        sides,
        rotation,
        style,
    }
}

pub fn preview(state: &ToolState) -> ToolPreview {
    if let (Some(center), Some(current)) = (state.drag_start, state.drag_current) {
        let dx = current.x - center.x;
        let dy = current.y - center.y;
        let radius = (dx * dx + dy * dy).sqrt();
        if radius > 0.5 {
            ToolPreview::Shape(build_shape(center, current, radius, state))
        } else {
            ToolPreview::None
        }
    } else {
        ToolPreview::None
    }
}
