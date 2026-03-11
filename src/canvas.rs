use iced::mouse;
use iced::widget::canvas::{self, Action, Event, Frame, Path, Stroke};
use iced::{Color, Point, Rectangle, Renderer, Theme};

use crate::app::Message;
use crate::document::Document;
use crate::grid::{self, GridConfig, GridStyle};
use crate::theme::EditorColors;
use crate::tool::{PenAnchor, Tool, ToolPreview, ToolState};
use crate::viewport::Viewport;

pub struct EditorCanvas<'a> {
    pub document: &'a Document,
    pub tool: Tool,
    pub tool_state: &'a ToolState,
    pub viewport: &'a Viewport,
    pub selected_index: Option<usize>,
    pub grid: &'a GridConfig,
    pub colors: &'a EditorColors,
}

#[derive(Default)]
pub struct CanvasState {
    cursor_position: Option<Point>,
    last_press_position: Option<Point>,
    is_dragging: bool,
    is_panning: bool,
    pan_start: Option<Point>,
    shift_held: bool,
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
        let mut world_pos = self.viewport.screen_to_world(cursor_in_bounds);

        // Snap: when snap is on, always snap unless shift held (and vice versa)
        if self.grid.visible && (self.grid.snap ^ state.shift_held) {
            world_pos = grid::snap_to_grid(world_pos, self.grid);
        }

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
                mouse::Event::ButtonPressed(mouse::Button::Right) => {
                    Some(
                        Action::publish(Message::CanvasRightClick(world_pos))
                            .and_capture(),
                    )
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
                        Key::Named(iced::keyboard::key::Named::Shift) => {
                            state.shift_held = true;
                            None
                        }
                        _ => None,
                    }
                }
                iced::keyboard::Event::KeyReleased { key, .. } => {
                    use iced::keyboard::Key;
                    if matches!(key, Key::Named(iced::keyboard::key::Named::Shift)) {
                        state.shift_held = false;
                    }
                    None
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

        let colors = self.colors;

        // Draw grid background
        draw_grid(&mut frame, bounds, self.viewport, self.grid, colors);

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
                    draw_selection_highlight(frame, shape, colors);
                }
            }

            // Draw tool preview
            match self.tool_state.preview(self.tool) {
                ToolPreview::Shape(shape) => {
                    shape.paint(frame);
                }
                ToolPreview::PenInProgress { anchors } => {
                    draw_pen_preview(frame, &anchors, state.cursor_position, colors);
                }
                ToolPreview::PolylineInProgress { points } => {
                    draw_polyline_preview(frame, &points, state.cursor_position, &self.tool_state.current_style, colors);
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

fn draw_grid(frame: &mut Frame<Renderer>, bounds: Rectangle, viewport: &Viewport, grid: &GridConfig, colors: &EditorColors) {
    // Background
    let bg = Path::rectangle(Point::ORIGIN, bounds.size());
    frame.fill(&bg, colors.canvas_bg);

    if !grid.visible {
        return;
    }

    let grid_size = grid.size * viewport.zoom;
    if grid_size < 5.0 {
        return; // Too zoomed out for grid
    }

    match grid.style {
        GridStyle::Lines => {
            let stroke = Stroke::default()
                .with_color(colors.grid_line)
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
        GridStyle::Dots => {
            let dot_color = colors.grid_dot;
            let dot_radius = 1.5;

            let offset_x = viewport.offset.x % grid_size;
            let offset_y = viewport.offset.y % grid_size;

            let mut x = offset_x;
            while x < bounds.width {
                let mut y = offset_y;
                while y < bounds.height {
                    let dot = Path::circle(Point::new(x, y), dot_radius);
                    frame.fill(&dot, dot_color);
                    y += grid_size;
                }
                x += grid_size;
            }
        }
        GridStyle::Isometric => {
            let stroke = Stroke::default()
                .with_color(colors.grid_line)
                .with_width(1.0);

            let sqrt3_2 = 3.0_f32.sqrt() / 2.0;
            let col_width = grid_size * sqrt3_2;
            let slope = 1.0 / 3.0_f32.sqrt(); // tan(30°)

            // Vertical lines: world x = -b * s * sqrt3_2 → screen x = offset.x + world_x * zoom
            let offset_x = viewport.offset.x.rem_euclid(col_width);
            let mut x = offset_x;
            while x < bounds.width {
                let path = Path::line(Point::new(x, 0.0), Point::new(x, bounds.height));
                frame.stroke(&path, stroke);
                x += col_width;
            }

            // Diagonal lines going down-right at +30°.
            // At screen x=0, y-intercept for the line through world grid point is:
            //   y_intercept = offset.y + (a + b/2)*s*zoom - offset.x * slope
            // These repeat every grid_size in y_intercept space.
            let diag_spacing = grid_size;
            let max_dim = bounds.width + bounds.height;
            let diag_offset = (viewport.offset.y - viewport.offset.x * slope).rem_euclid(diag_spacing);

            let mut start_y = diag_offset - max_dim;
            while start_y < bounds.height + max_dim {
                let path = Path::line(
                    Point::new(0.0, start_y),
                    Point::new(bounds.width, start_y + bounds.width * slope),
                );
                frame.stroke(&path, stroke);
                start_y += diag_spacing;
            }

            // Diagonal lines going up-right at -30°.
            // y_intercept = offset.y + (a + b/2)*s*zoom + offset.x * slope
            let diag_offset2 = (viewport.offset.y + viewport.offset.x * slope).rem_euclid(diag_spacing);

            let mut start_y = diag_offset2 - max_dim;
            while start_y < bounds.height + max_dim {
                let path = Path::line(
                    Point::new(0.0, start_y),
                    Point::new(bounds.width, start_y - bounds.width * slope),
                );
                frame.stroke(&path, stroke);
                start_y += diag_spacing;
            }
        }
    }
}

fn draw_selection_highlight(frame: &mut Frame<Renderer>, shape: &crate::shape::ShapeItem, colors: &EditorColors) {
    let stroke = Stroke::default()
        .with_color(colors.selection_highlight)
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
        crate::shape::ShapeItem::RightTriangle { origin, width, height, .. } => {
            let verts = [
                *origin,
                Point::new(origin.x + width, origin.y),
                Point::new(origin.x, origin.y + height),
            ];
            let path = Path::new(|builder| {
                builder.move_to(verts[0]);
                builder.line_to(verts[1]);
                builder.line_to(verts[2]);
                builder.close();
            });
            frame.stroke(&path, stroke);
        }
        crate::shape::ShapeItem::Polyline { points, .. } => {
            if points.len() >= 2 {
                let path = Path::new(|builder| {
                    builder.move_to(points[0]);
                    for p in &points[1..] {
                        builder.line_to(*p);
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
    colors: &EditorColors,
) {
    if anchors.is_empty() {
        return;
    }

    let curve_stroke = Stroke::default()
        .with_color(colors.pen_curve)
        .with_width(2.0);
    let handle_stroke = Stroke::default()
        .with_color(colors.pen_handle_stroke)
        .with_width(1.0);
    let anchor_color = colors.pen_anchor;

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
                .with_color(colors.pen_preview)
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
        frame.fill(&dot_in, colors.pen_handle_fill);
        frame.stroke(&dot_in, handle_stroke);
        frame.fill(&dot_out, colors.pen_handle_fill);
        frame.stroke(&dot_out, handle_stroke);

        // Anchor dot
        let dot = Path::circle(anchor.point, 4.0);
        frame.fill(&dot, anchor_color);
    }
}

fn draw_polyline_preview(
    frame: &mut Frame<Renderer>,
    points: &[Point],
    cursor: Option<Point>,
    style: &crate::shape::Style,
    colors: &EditorColors,
) {
    if points.is_empty() {
        return;
    }

    let stroke_color = style.stroke_color.unwrap_or(Color::BLACK);
    let stroke = Stroke::default()
        .with_color(stroke_color)
        .with_width(style.stroke_width);

    // Draw placed segments
    if points.len() >= 2 {
        let path = Path::new(|builder| {
            builder.move_to(points[0]);
            for p in &points[1..] {
                builder.line_to(*p);
            }
        });
        frame.stroke(&path, stroke);
    }

    // Rubber-band line to cursor
    if let Some(cursor_pos) = cursor {
        if let Some(last) = points.last() {
            let preview_path = Path::line(*last, cursor_pos);
            let preview_stroke = Stroke::default()
                .with_color(colors.pen_preview)
                .with_width(style.stroke_width);
            frame.stroke(&preview_path, preview_stroke);
        }
    }

    // Draw vertex dots
    let dot_color = colors.polyline_dot;
    for p in points {
        let dot = Path::circle(*p, 3.0);
        frame.fill(&dot, dot_color);
    }
}
