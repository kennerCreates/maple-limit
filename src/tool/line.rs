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
            if let Some(start) = state.drag_start.take() {
                state.drag_current = None;
                let dist = ((start.x - pos.x).powi(2) + (start.y - pos.y).powi(2)).sqrt();
                if dist > 1.0 {
                    ToolResult::ShapeCompleted(ShapeItem::Line {
                        start,
                        end: pos,
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
    if let (Some(start), Some(current)) = (state.drag_start, state.drag_current) {
        ToolPreview::Shape(ShapeItem::Line {
            start,
            end: current,
            style: state.current_style.clone(),
        })
    } else {
        ToolPreview::None
    }
}
