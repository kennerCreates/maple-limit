use crate::shape::ShapeItem;
use super::{ToolEvent, ToolPreview, ToolResult, ToolState};

pub fn handle(state: &mut ToolState, event: ToolEvent) -> ToolResult {
    match event {
        ToolEvent::Press(pos, _) => {
            state.line_points.push(pos);
            ToolResult::RequestRedraw
        }
        ToolEvent::Move(pos) => {
            if !state.line_points.is_empty() {
                state.drag_current = Some(pos);
                ToolResult::RequestRedraw
            } else {
                ToolResult::None
            }
        }
        ToolEvent::KeyEnter | ToolEvent::RightClick(_) => {
            finalize(state)
        }
        _ => ToolResult::None,
    }
}

fn finalize(state: &mut ToolState) -> ToolResult {
    if state.line_points.len() < 2 {
        state.reset_line();
        return ToolResult::None;
    }

    let points = std::mem::take(&mut state.line_points);
    let style = state.current_style.clone();
    state.drag_current = None;

    ToolResult::ShapeCompleted(ShapeItem::Polyline { points, style })
}

pub fn preview(state: &ToolState) -> ToolPreview {
    if state.line_points.is_empty() {
        return ToolPreview::None;
    }
    ToolPreview::PolylineInProgress {
        points: state.line_points.clone(),
    }
}
