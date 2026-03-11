use iced::Point;

use crate::shape::ShapeItem;

#[derive(Debug, Clone)]
pub enum Command {
    AddShape(usize, ShapeItem),
    RemoveShape(usize, ShapeItem),
    MoveShape(usize, f32, f32), // index, dx, dy
}

pub struct Document {
    pub shapes: Vec<ShapeItem>,
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shape: ShapeItem) {
        let idx = self.shapes.len();
        self.shapes.push(shape.clone());
        self.undo_stack.push(Command::AddShape(idx, shape));
        self.redo_stack.clear();
    }

    pub fn remove_shape(&mut self, idx: usize) -> Option<ShapeItem> {
        if idx < self.shapes.len() {
            let shape = self.shapes.remove(idx);
            self.undo_stack.push(Command::RemoveShape(idx, shape.clone()));
            self.redo_stack.clear();
            Some(shape)
        } else {
            None
        }
    }

    pub fn move_shape(&mut self, idx: usize, dx: f32, dy: f32) {
        if idx < self.shapes.len() {
            self.shapes[idx].translate(dx, dy);
            self.undo_stack.push(Command::MoveShape(idx, dx, dy));
            self.redo_stack.clear();
        }
    }

    pub fn undo(&mut self) {
        if let Some(cmd) = self.undo_stack.pop() {
            match &cmd {
                Command::AddShape(idx, _) => {
                    if *idx < self.shapes.len() {
                        self.shapes.remove(*idx);
                    }
                }
                Command::RemoveShape(idx, shape) => {
                    let idx = (*idx).min(self.shapes.len());
                    self.shapes.insert(idx, shape.clone());
                }
                Command::MoveShape(idx, dx, dy) => {
                    if *idx < self.shapes.len() {
                        self.shapes[*idx].translate(-dx, -dy);
                    }
                }
            }
            self.redo_stack.push(cmd);
        }
    }

    pub fn redo(&mut self) {
        if let Some(cmd) = self.redo_stack.pop() {
            match &cmd {
                Command::AddShape(_, shape) => {
                    self.shapes.push(shape.clone());
                }
                Command::RemoveShape(idx, _) => {
                    if *idx < self.shapes.len() {
                        self.shapes.remove(*idx);
                    }
                }
                Command::MoveShape(idx, dx, dy) => {
                    if *idx < self.shapes.len() {
                        self.shapes[*idx].translate(*dx, *dy);
                    }
                }
            }
            self.undo_stack.push(cmd);
        }
    }

    pub fn hit_test(&self, point: Point) -> Option<usize> {
        // Return topmost (last) shape that contains the point
        for (i, shape) in self.shapes.iter().enumerate().rev() {
            if shape.hit_test(point) {
                return Some(i);
            }
        }
        None
    }
}
