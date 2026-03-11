use iced::{Point, Size};

use crate::shape::{ShapeItem, Style};
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
                let (tl, size) = rect_from_points(start, pos);
                if size.width > 1.0 && size.height > 1.0 {
                    ToolResult::ShapeCompleted(ShapeItem::Rectangle {
                        top_left: tl,
                        size,
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
        let (tl, size) = rect_from_points(start, current);
        ToolPreview::Shape(ShapeItem::Rectangle {
            top_left: tl,
            size,
            style: Style {
                stroke_color: state.current_style.stroke_color,
                stroke_width: state.current_style.stroke_width,
                fill_color: state.current_style.fill_color,
            },
        })
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
