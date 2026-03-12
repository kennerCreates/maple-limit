use crate::document::{Document, HitTarget};
use super::{ToolEvent, ToolResult, ToolState};

pub fn handle(state: &mut ToolState, event: ToolEvent, doc: &Document) -> ToolResult {
    match event {
        ToolEvent::Press(pos) => {
            // If we're editing a boolean group, hit test within that group
            if let Some(group_idx) = state.editing_group {
                if let Some(shape_idx) = doc.hit_test_in_group(pos, group_idx) {
                    state.selected_index = Some(shape_idx);
                    state.selected_bool_group = None;
                    state.select_drag_start = Some(pos);
                    return ToolResult::SelectShape(Some(shape_idx));
                } else {
                    // Clicked outside group — exit editing mode
                    state.editing_group = None;
                    state.selected_index = None;
                    state.selected_bool_group = None;
                    // Fall through to normal hit test below
                }
            }

            // Normal hit test — check boolean groups first, then free shapes
            match doc.hit_test_any(pos) {
                Some(HitTarget::BoolGroup(group_idx)) => {
                    state.selected_bool_group = Some(group_idx);
                    state.selected_index = None;
                    state.select_drag_start = None;
                    ToolResult::RequestRedraw
                }
                Some(HitTarget::Shape(idx)) => {
                    state.selected_index = Some(idx);
                    state.selected_bool_group = None;
                    state.select_drag_start = Some(pos);
                    ToolResult::SelectShape(Some(idx))
                }
                None => {
                    state.selected_index = None;
                    state.selected_bool_group = None;
                    ToolResult::SelectShape(None)
                }
            }
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
