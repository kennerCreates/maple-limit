use crate::document::Document;
use super::{ToolEvent, ToolResult, ToolState};

pub fn handle(state: &mut ToolState, event: ToolEvent, doc: &Document) -> ToolResult {
    match event {
        ToolEvent::Press(pos) => {
            let hit = doc.hit_test(pos);
            state.selected_index = hit;
            if hit.is_some() {
                state.select_drag_start = Some(pos);
            }
            ToolResult::SelectShape(hit)
        }
        ToolEvent::Drag(pos) => {
            if let (Some(_selected), Some(drag_start)) =
                (state.selected_index, state.select_drag_start)
            {
                let dx = pos.x - drag_start.x;
                let dy = pos.y - drag_start.y;
                state.select_drag_start = Some(pos);
                ToolResult::MoveSelected(dx, dy)
            } else {
                ToolResult::None
            }
        }
        ToolEvent::Release(_) => {
            state.select_drag_start = None;
            ToolResult::None
        }
        _ => ToolResult::None,
    }
}
