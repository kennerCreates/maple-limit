use iced::Point;

use crate::shape::{ShapeItem, SplineSegment};
use super::{PenAnchor, ToolEvent, ToolPreview, ToolResult, ToolState};

pub fn handle(state: &mut ToolState, event: ToolEvent) -> ToolResult {
    match event {
        ToolEvent::Press(pos) => {
            // Start a new anchor
            state.pen_dragging = true;
            state.pen_anchors.push(PenAnchor {
                point: pos,
                handle_in: pos,
                handle_out: pos,
            });
            ToolResult::RequestRedraw
        }
        ToolEvent::Drag(pos) => {
            // Adjust tangent handles of current anchor
            if state.pen_dragging {
                if let Some(anchor) = state.pen_anchors.last_mut() {
                    anchor.handle_out = pos;
                    // Mirror: handle_in is the reflection
                    anchor.handle_in = Point::new(
                        2.0 * anchor.point.x - pos.x,
                        2.0 * anchor.point.y - pos.y,
                    );
                }
                ToolResult::RequestRedraw
            } else {
                ToolResult::None
            }
        }
        ToolEvent::Release(_pos) => {
            state.pen_dragging = false;
            ToolResult::RequestRedraw
        }
        ToolEvent::Move(_pos) => {
            // Update cursor position for preview curve
            if !state.pen_anchors.is_empty() {
                ToolResult::RequestRedraw
            } else {
                ToolResult::None
            }
        }
        ToolEvent::KeyEnter => {
            // Finalize the spline
            finalize(state)
        }
    }
}

fn finalize(state: &mut ToolState) -> ToolResult {
    if state.pen_anchors.len() < 2 {
        state.reset_pen();
        return ToolResult::None;
    }

    let segments = build_segments(&state.pen_anchors);
    let style = state.current_style.clone();
    state.reset_pen();

    ToolResult::ShapeCompleted(ShapeItem::Spline { segments, style })
}

fn build_segments(anchors: &[PenAnchor]) -> Vec<SplineSegment> {
    let mut segments = Vec::new();
    for i in 0..anchors.len() - 1 {
        let a = &anchors[i];
        let b = &anchors[i + 1];
        segments.push(SplineSegment {
            start: a.point,
            control_a: a.handle_out,
            control_b: b.handle_in,
            end: b.point,
        });
    }
    segments
}

pub fn preview(state: &ToolState) -> ToolPreview {
    if state.pen_anchors.is_empty() {
        return ToolPreview::None;
    }
    ToolPreview::PenInProgress {
        anchors: state.pen_anchors.clone(),
    }
}
