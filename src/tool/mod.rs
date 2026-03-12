pub mod bool_tool;
pub mod line;
pub mod spline;
pub mod select;
pub mod shape;

use iced::Point;

use crate::boolean::BoolOp;
use crate::shape::{ShapeItem, Style};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Select,
    Shape,
    Line,
    Spline,
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeType {
    Triangle,
    Rectangle,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
    Nonagon,
    Decagon,
    Hendecagon,
    Dodecagon,
    Circle,
}

impl ShapeType {
    pub fn sides(&self) -> Option<usize> {
        match self {
            ShapeType::Triangle => Some(3),
            ShapeType::Pentagon => Some(5),
            ShapeType::Hexagon => Some(6),
            ShapeType::Heptagon => Some(7),
            ShapeType::Octagon => Some(8),
            ShapeType::Nonagon => Some(9),
            ShapeType::Decagon => Some(10),
            ShapeType::Hendecagon => Some(11),
            ShapeType::Dodecagon => Some(12),
            ShapeType::Circle => None,
            ShapeType::Rectangle => None,
        }
    }
}

impl std::fmt::Display for ShapeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShapeType::Triangle => write!(f, "Triangle (3)"),
            ShapeType::Rectangle => write!(f, "Rectangle"),
            ShapeType::Pentagon => write!(f, "Pentagon (5)"),
            ShapeType::Hexagon => write!(f, "Hexagon (6)"),
            ShapeType::Heptagon => write!(f, "Heptagon (7)"),
            ShapeType::Octagon => write!(f, "Octagon (8)"),
            ShapeType::Nonagon => write!(f, "Nonagon (9)"),
            ShapeType::Decagon => write!(f, "Decagon (10)"),
            ShapeType::Hendecagon => write!(f, "Hendecagon (11)"),
            ShapeType::Dodecagon => write!(f, "Dodecagon (12)"),
            ShapeType::Circle => write!(f, "Circle"),
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
    CreateBooleanGroup(usize, usize, BoolOp),
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
    // Shape / Line
    pub drag_start: Option<Point>,
    pub drag_current: Option<Point>,

    // Select
    pub selected_index: Option<usize>,
    pub select_drag_start: Option<Point>,
    pub selected_bool_group: Option<usize>,
    pub editing_group: Option<usize>, // which boolean group is being edited (entered)

    // Pen
    pub pen_anchors: Vec<PenAnchor>,
    pub pen_dragging: bool,

    // Polyline (Line tool)
    pub line_points: Vec<Point>,

    // Bool tool
    pub bool_op: BoolOp,
    pub bool_selection: Vec<usize>,

    // Config
    pub shape_type: ShapeType,
    pub skew_angle: f32, // skew angle in degrees (for Rectangle)
    pub current_style: Style,
}

impl Default for ToolState {
    fn default() -> Self {
        Self {
            drag_start: None,
            drag_current: None,
            selected_index: None,
            select_drag_start: None,
            selected_bool_group: None,
            editing_group: None,
            pen_anchors: Vec::new(),
            pen_dragging: false,
            line_points: Vec::new(),
            bool_op: BoolOp::Union,
            bool_selection: Vec::new(),
            shape_type: ShapeType::Rectangle,
            skew_angle: 0.0,
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

    pub fn reset_bool(&mut self) {
        self.bool_selection.clear();
    }

    pub fn preview(&self, tool: Tool) -> ToolPreview {
        match tool {
            Tool::Shape => shape::preview(self),
            Tool::Line => line::preview(self),
            Tool::Spline => spline::preview(self),
            Tool::Select | Tool::Bool => ToolPreview::None,
        }
    }
}
