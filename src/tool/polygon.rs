use iced::Point;

use crate::shape::ShapeItem;
use super::{ToolEvent, ToolPreview, ToolResult, ToolState};

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
                let radius = distance(center, pos);
                if radius > 1.0 {
                    ToolResult::ShapeCompleted(ShapeItem::RegularPolygon {
                        center,
                        radius,
                        sides: state.polygon_sides,
                        rotation: 0.0,
                        style: state.current_style.clone(),
                    })
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

pub fn preview(state: &ToolState) -> ToolPreview {
    if let (Some(center), Some(current)) = (state.drag_start, state.drag_current) {
        let radius = distance(center, current);
        ToolPreview::Shape(ShapeItem::RegularPolygon {
            center,
            radius,
            sides: state.polygon_sides,
            rotation: 0.0,
            style: state.current_style.clone(),
        })
    } else {
        ToolPreview::None
    }
}

fn distance(a: Point, b: Point) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
