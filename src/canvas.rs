use iced::mouse;
use iced::widget::canvas::{self, Action, Event, Frame, Path, Stroke};
use iced::{Color, Point, Rectangle, Renderer, Theme};

use crate::app::Message;
use crate::document::Document;
use crate::tool::{PenAnchor, Tool, ToolPreview, ToolState};
use crate::viewport::Viewport;

pub struct EditorCanvas<'a> {
    pub document: &'a Document,
    pub tool: Tool,
    pub tool_state: &'a ToolState,
    pub viewport: &'a Viewport,
    pub selected_index: Option<usize>,
}

#[derive(Default)]
pub struct CanvasState {
    cursor_position: Option<Point>,
    last_press_position: Option<Point>,
    is_dragging: bool,
    is_panning: bool,
    pan_start: Option<Point>,
}

impl<'a> canvas::Program<Message> for EditorCanvas<'a> {
    type State = CanvasState;

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        let cursor_in_bounds = cursor.position_in(bounds)?;
        let world_pos = self.viewport.screen_to_world(cursor_in_bounds);

        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    state.is_dragging = true;
                    state.last_press_position = Some(world_pos);
                    state.cursor_position = Some(world_pos);
                    Some(
                        Action::publish(Message::CanvasPress(world_pos))
                            .and_capture(),
                    )
                }
                mouse::Event::ButtonPressed(mouse::Button::Middle) => {
                    state.is_panning = true;
                    state.pan_start = Some(cursor_in_bounds);
                    Some(Action::capture())
                }
                mouse::Event::CursorMoved { .. } => {
                    state.cursor_position = Some(world_pos);
                    if state.is_panning {
                        if let Some(pan_start) = state.pan_start {
                            let dx = cursor_in_bounds.x - pan_start.x;
                            let dy = cursor_in_bounds.y - pan_start.y;
                            state.pan_start = Some(cursor_in_bounds);
                            return Some(
                                Action::publish(Message::Pan(dx, dy)).and_capture(),
                            );
                        }
                    }
                    if state.is_dragging {
                        Some(
                            Action::publish(Message::CanvasDrag(world_pos))
                                .and_capture(),
                        )
                    } else {
                        Some(
                            Action::publish(Message::CanvasMove(world_pos))
                                .and_capture(),
                        )
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    state.is_dragging = false;
                    // Detect double-click (simple: if release is very close to last press)
                    Some(
                        Action::publish(Message::CanvasRelease(world_pos))
                            .and_capture(),
                    )
                }
                mouse::Event::ButtonReleased(mouse::Button::Middle) => {
                    state.is_panning = false;
                    state.pan_start = None;
                    Some(Action::capture())
                }
                mouse::Event::WheelScrolled { delta } => {
                    let scroll_y = match delta {
                        mouse::ScrollDelta::Lines { y, .. } => *y,
                        mouse::ScrollDelta::Pixels { y, .. } => *y / 50.0,
                    };
                    let factor = if scroll_y > 0.0 { 1.1 } else { 1.0 / 1.1 };
                    Some(
                        Action::publish(Message::Zoom(cursor_in_bounds, factor))
                            .and_capture(),
                    )
                }
                _ => None,
            },
            Event::Keyboard(kb_event) => match kb_event {
                iced::keyboard::Event::KeyPressed { key, .. } => {
                    use iced::keyboard::Key;
                    match key {
                        Key::Named(iced::keyboard::key::Named::Enter) => {
                            Some(Action::publish(Message::CanvasKeyEnter))
                        }
                        Key::Named(iced::keyboard::key::Named::Delete)
                        | Key::Named(iced::keyboard::key::Named::Backspace) => {
                            Some(Action::publish(Message::DeleteSelected))
                        }
                        _ => None,
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<iced::widget::canvas::Geometry<Renderer>> {
        let mut frame = Frame::new(renderer, bounds.size());

        // Draw grid background
        draw_grid(&mut frame, bounds, self.viewport);

        // Apply viewport transform for shapes
        frame.with_save(|frame| {
            frame.translate(iced::Vector::new(
                self.viewport.offset.x,
                self.viewport.offset.y,
            ));
            frame.scale(self.viewport.zoom);

            // Draw all shapes
            for (i, shape) in self.document.shapes.iter().enumerate() {
                shape.paint(frame);

                // Highlight selected shape
                if self.selected_index == Some(i) {
                    draw_selection_highlight(frame, shape);
                }
            }

            // Draw tool preview
            match self.tool_state.preview(self.tool) {
                ToolPreview::Shape(shape) => {
                    shape.paint(frame);
                }
                ToolPreview::PenInProgress { anchors } => {
                    draw_pen_preview(frame, &anchors, state.cursor_position);
                }
                ToolPreview::None => {}
            }
        });

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            match self.tool {
                Tool::Select => mouse::Interaction::Pointer,
                _ => mouse::Interaction::Crosshair,
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

fn draw_grid(frame: &mut Frame<Renderer>, bounds: Rectangle, viewport: &Viewport) {
    // Background
    let bg = Path::rectangle(Point::ORIGIN, bounds.size());
    frame.fill(&bg, Color::from_rgb(0.95, 0.95, 0.95));

    // Grid lines
    let grid_size = 20.0 * viewport.zoom;
    if grid_size < 5.0 {
        return; // Too zoomed out for grid
    }

    let stroke = Stroke::default()
        .with_color(Color::from_rgb(0.88, 0.88, 0.88))
        .with_width(1.0);

    let offset_x = viewport.offset.x % grid_size;
    let offset_y = viewport.offset.y % grid_size;

    let mut x = offset_x;
    while x < bounds.width {
        let path = Path::line(Point::new(x, 0.0), Point::new(x, bounds.height));
        frame.stroke(&path, stroke);
        x += grid_size;
    }

    let mut y = offset_y;
    while y < bounds.height {
        let path = Path::line(Point::new(0.0, y), Point::new(bounds.width, y));
        frame.stroke(&path, stroke);
        y += grid_size;
    }
}

fn draw_selection_highlight(frame: &mut Frame<Renderer>, shape: &crate::shape::ShapeItem) {
    let stroke = Stroke::default()
        .with_color(Color::from_rgb(0.2, 0.5, 1.0))
        .with_width(1.5);

    match shape {
        crate::shape::ShapeItem::Circle { center, radius, .. } => {
            let path = Path::circle(*center, *radius + 3.0);
            frame.stroke(&path, stroke);
        }
        crate::shape::ShapeItem::Rectangle { top_left, size, .. } => {
            let path = Path::rectangle(
                Point::new(top_left.x - 3.0, top_left.y - 3.0),
                iced::Size::new(size.width + 6.0, size.height + 6.0),
            );
            frame.stroke(&path, stroke);
        }
        crate::shape::ShapeItem::RegularPolygon {
            center, radius, sides, rotation, ..
        } => {
            let path = crate::shape::polygon_path(*center, *radius + 3.0, *sides, *rotation);
            frame.stroke(&path, stroke);
        }
        crate::shape::ShapeItem::Line { start, end, .. } => {
            let path = Path::line(*start, *end);
            frame.stroke(&path, stroke);
        }
        crate::shape::ShapeItem::Spline { segments, .. } => {
            if !segments.is_empty() {
                let path = Path::new(|builder| {
                    builder.move_to(segments[0].start);
                    for seg in segments {
                        builder.bezier_curve_to(seg.control_a, seg.control_b, seg.end);
                    }
                });
                frame.stroke(&path, stroke);
            }
        }
    }
}

fn draw_pen_preview(
    frame: &mut Frame<Renderer>,
    anchors: &[PenAnchor],
    cursor: Option<Point>,
) {
    if anchors.is_empty() {
        return;
    }

    let curve_stroke = Stroke::default()
        .with_color(Color::BLACK)
        .with_width(2.0);
    let handle_stroke = Stroke::default()
        .with_color(Color::from_rgb(0.5, 0.5, 0.5))
        .with_width(1.0);
    let anchor_color = Color::from_rgb(0.2, 0.5, 1.0);

    // Draw completed segments
    if anchors.len() >= 2 {
        let path = Path::new(|builder| {
            builder.move_to(anchors[0].point);
            for i in 0..anchors.len() - 1 {
                let a = &anchors[i];
                let b = &anchors[i + 1];
                builder.bezier_curve_to(a.handle_out, b.handle_in, b.point);
            }
        });
        frame.stroke(&path, curve_stroke);
    }

    // Draw preview segment to cursor
    if let Some(cursor_pos) = cursor {
        if let Some(last) = anchors.last() {
            let preview_path = Path::new(|builder| {
                builder.move_to(last.point);
                builder.bezier_curve_to(last.handle_out, cursor_pos, cursor_pos);
            });
            let preview_stroke = Stroke::default()
                .with_color(Color::from_rgba(0.0, 0.0, 0.0, 0.4))
                .with_width(1.5);
            frame.stroke(&preview_path, preview_stroke);
        }
    }

    // Draw anchor points and handles
    for anchor in anchors {
        // Handle lines
        let handle_line_in = Path::line(anchor.point, anchor.handle_in);
        let handle_line_out = Path::line(anchor.point, anchor.handle_out);
        frame.stroke(&handle_line_in, handle_stroke);
        frame.stroke(&handle_line_out, handle_stroke);

        // Handle dots
        let dot_in = Path::circle(anchor.handle_in, 3.0);
        let dot_out = Path::circle(anchor.handle_out, 3.0);
        frame.fill(&dot_in, Color::WHITE);
        frame.stroke(&dot_in, handle_stroke);
        frame.fill(&dot_out, Color::WHITE);
        frame.stroke(&dot_out, handle_stroke);

        // Anchor dot
        let dot = Path::circle(anchor.point, 4.0);
        frame.fill(&dot, anchor_color);
    }
}
