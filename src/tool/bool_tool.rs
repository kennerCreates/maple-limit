use crate::boolean;
use crate::document::Document;
use crate::tool::{ToolEvent, ToolResult, ToolState};

pub fn handle(state: &mut ToolState, event: ToolEvent, doc: &Document) -> ToolResult {
    match event {
        ToolEvent::Press(pos, _) => {
            if let Some(idx) = doc.hit_test(pos) {
                // Only allow closed shapes
                if boolean::is_closed_shape(&doc.shapes[idx]) {
                    state.bool_selection.push(idx);
                    if state.bool_selection.len() >= 2 {
                        let a = state.bool_selection[0];
                        let b = state.bool_selection[1];
                        state.bool_selection.clear();
                        if a != b {
                            return ToolResult::CreateBooleanGroup(a, b, state.bool_op);
                        }
                    }
                    return ToolResult::SelectShape(Some(idx));
                }
            }
            ToolResult::None
        }
        ToolEvent::RightClick(_) => {
            // Cancel current selection
            state.bool_selection.clear();
            ToolResult::RequestRedraw
        }
        _ => ToolResult::None,
    }
}
