pub mod line;
pub mod pen;
pub mod rectangle;
pub mod select;
pub mod shape;

use iced::Point;

use crate::shape::{ShapeItem, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Select,
    Rectangle,
    Shape,
    Line,
    Pen,
}

impl Tool {
    pub fn label(&self) -> &str {
        match self {
            Tool::Select => "Select",
            Tool::Rectangle => "Rectangle",
            Tool::Shape => "Shape",
            Tool::Line => "Line",
            Tool::Pen => "Pen",
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ToolEvent {
    Press(Point),
    Drag(Point),
    Release(Point),
    Move(Point),
    KeyEnter,
    RightClick(Point),
}

#[derive(Debug, Clone)]
pub enum ToolResult {
    None,
    ShapeCompleted(ShapeItem),
    SelectShape(Option<usize>),
    MoveSelected(f32, f32),
    RequestRedraw,
}

/// Preview geometry to draw while a tool is in progress.
#[derive(Debug, Clone)]
pub enum ToolPreview {
    None,
    Shape(ShapeItem),
    PenInProgress {
        anchors: Vec<PenAnchor>,
    },
    PolylineInProgress {
        points: Vec<Point>,
    },
}

#[derive(Debug, Clone)]
pub struct PenAnchor {
    pub point: Point,
    pub handle_in: Point,
    pub handle_out: Point,
}

pub struct ToolState {
    // Rectangle / Shape / Line
    pub drag_start: Option<Point>,
    pub drag_current: Option<Point>,

    // Select
    pub selected_index: Option<usize>,
    pub select_drag_start: Option<Point>,

    // Pen
    pub pen_anchors: Vec<PenAnchor>,
    pub pen_dragging: bool,

    // Polyline (Line tool)
    pub line_points: Vec<Point>,

    // Config
    pub shape_sides: usize,
    pub right_triangle: bool,
    pub current_style: Style,
}

impl Default for ToolState {
    fn default() -> Self {
        Self {
            drag_start: None,
            drag_current: None,
            selected_index: None,
            select_drag_start: None,
            pen_anchors: Vec::new(),
            pen_dragging: false,
            line_points: Vec::new(),
            shape_sides: 6,
            right_triangle: false,
            current_style: Style::default(),
        }
    }
}

impl ToolState {
    pub fn reset_drag(&mut self) {
        self.drag_start = None;
        self.drag_current = None;
    }

    pub fn reset_pen(&mut self) {
        self.pen_anchors.clear();
        self.pen_dragging = false;
    }

    pub fn reset_line(&mut self) {
        self.line_points.clear();
    }

    pub fn preview(&self, tool: Tool) -> ToolPreview {
        match tool {
            Tool::Rectangle => rectangle::preview(self),
            Tool::Shape => shape::preview(self),
            Tool::Line => line::preview(self),
            Tool::Pen => pen::preview(self),
            Tool::Select => ToolPreview::None,
        }
    }
}
