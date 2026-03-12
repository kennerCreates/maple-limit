use crate::document::{Document, HitTarget};
use super::{ToolEvent, ToolResult, ToolState};

pub fn handle(state: &mut ToolState, event: ToolEvent, doc: &Document) -> ToolResult {
    match event {
        ToolEvent::Press(pos, shift) => {
            // If we're editing a boolean group, hit test within that group
            if let Some(group_idx) = state.editing_group {
                if let Some(shape_idx) = doc.hit_test_in_group(pos, group_idx) {
                    if shift {
                        toggle_selection(&mut state.selected_indices, shape_idx);
                    } else {
                        state.selected_indices = vec![shape_idx];
                    }
                    state.selected_bool_group = None;
                    state.select_drag_start = Some(pos);
                    return ToolResult::RequestRedraw;
                } else {
                    // Clicked outside group — exit editing mode
                    state.editing_group = None;
                    state.selected_indices.clear();
                    state.selected_bool_group = None;
                    // Fall through to normal hit test below
                }
            }

            // Normal hit test — check boolean groups first, then free shapes
            match doc.hit_test_any(pos) {
                Some(HitTarget::BoolGroup(group_idx)) => {
                    state.selected_bool_group = Some(group_idx);
                    state.selected_indices.clear();
                    state.select_drag_start = None;
                    ToolResult::RequestRedraw
                }
                Some(HitTarget::Shape(idx)) => {
                    if shift {
                        toggle_selection(&mut state.selected_indices, idx);
                    } else {
                        // If clicking on an already-selected shape in a multi-selection,
                        // keep the multi-selection (allows dragging the group)
                        if !state.selected_indices.contains(&idx) {
                            state.selected_indices = vec![idx];
                        }
                    }
                    state.selected_bool_group = None;
                    state.select_drag_start = Some(pos);
                    ToolResult::RequestRedraw
                }
                None => {
                    if !shift {
                        state.selected_indices.clear();
                    }
                    state.selected_bool_group = None;
                    ToolResult::RequestRedraw
                }
            }
        }
        ToolEvent::Drag(pos) => {
            if !state.selected_indices.is_empty() {
                if let Some(drag_start) = state.select_drag_start {
                    let dx = pos.x - drag_start.x;
                    let dy = pos.y - drag_start.y;
                    state.select_drag_start = Some(pos);
                    return ToolResult::MoveSelected(dx, dy);
                }
            }
            ToolResult::None
        }
        ToolEvent::Release(_) => {
            state.select_drag_start = None;
            ToolResult::None
        }
        _ => ToolResult::None,
    }
}

fn toggle_selection(selected: &mut Vec<usize>, idx: usize) {
    if let Some(pos) = selected.iter().position(|&i| i == idx) {
        selected.remove(pos);
    } else {
        selected.push(idx);
    }
}
