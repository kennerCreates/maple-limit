use iced::Point;

use crate::boolean::{self, BoolOp, BooleanGroup};
use crate::shape::{ShapeItem, Style};

#[derive(Debug, Clone)]
pub enum Command {
    AddShape(usize, ShapeItem),
    RemoveShape(usize, ShapeItem),
    MoveShape(usize, f32, f32),
    UpdateShape(usize, ShapeItem, ShapeItem), // index, old_shape, new_shape
    CreateBooleanGroup(usize, BooleanGroup),
    DissolveBooleanGroup(usize, BooleanGroup),
    ChangeBooleanOp(usize, BoolOp, BoolOp), // group_idx, old_op, new_op
}

/// Distinguishes whether a hit landed on a free shape or a boolean group.
#[derive(Debug, Clone, Copy)]
pub enum HitTarget {
    Shape(usize),
    BoolGroup(usize),
}

pub struct Document {
    pub shapes: Vec<ShapeItem>,
    pub boolean_groups: Vec<BooleanGroup>,
    pub group_membership: Vec<Option<usize>>, // parallel to shapes; Some(group_idx) if in a group
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
}

impl Document {
    pub fn new() -> Self {
        Self {
            shapes: Vec::new(),
            boolean_groups: Vec::new(),
            group_membership: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shape: ShapeItem) {
        let idx = self.shapes.len();
        self.shapes.push(shape.clone());
        self.group_membership.push(None);
        self.undo_stack.push(Command::AddShape(idx, shape));
        self.redo_stack.clear();
    }

    pub fn remove_shape(&mut self, idx: usize) -> Option<ShapeItem> {
        if idx < self.shapes.len() {
            // If this shape is in a boolean group, dissolve it first
            if let Some(group_idx) = self.group_membership[idx] {
                self.dissolve_boolean_group(group_idx);
            }

            let shape = self.shapes.remove(idx);
            self.group_membership.remove(idx);

            // Adjust group membership indices for shapes after the removed one
            // Adjust shape indices in boolean groups
            for group in &mut self.boolean_groups {
                if group.shape_a > idx {
                    group.shape_a -= 1;
                }
                if group.shape_b > idx {
                    group.shape_b -= 1;
                }
            }

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

            // Recompute boolean group if this shape is a member
            if let Some(group_idx) = self.group_membership[idx] {
                self.recompute_group(group_idx);
            }
        }
    }

    pub fn update_shape(&mut self, idx: usize, new_shape: ShapeItem) {
        if idx < self.shapes.len() {
            let old_shape = self.shapes[idx].clone();
            self.shapes[idx] = new_shape.clone();
            self.undo_stack.push(Command::UpdateShape(idx, old_shape, new_shape));
            self.redo_stack.clear();

            // Recompute boolean group if this shape is a member
            if let Some(group_idx) = self.group_membership[idx] {
                self.recompute_group(group_idx);
            }
        }
    }

    pub fn create_boolean_group(&mut self, shape_a: usize, shape_b: usize, op: BoolOp, style: Style) {
        if shape_a >= self.shapes.len() || shape_b >= self.shapes.len() {
            return;
        }
        if !boolean::is_closed_shape(&self.shapes[shape_a])
            || !boolean::is_closed_shape(&self.shapes[shape_b])
        {
            return;
        }

        let mut group = BooleanGroup {
            op,
            shape_a,
            shape_b,
            style,
            cached_result: Vec::new(),
        };

        // Compute initial result
        if let (Some(poly_a), Some(poly_b)) = (
            boolean::shape_to_polygon(&self.shapes[shape_a]),
            boolean::shape_to_polygon(&self.shapes[shape_b]),
        ) {
            group.cached_result = boolean::compute_boolean(&poly_a, &poly_b, op);
        }

        let group_idx = self.boolean_groups.len();
        self.boolean_groups.push(group.clone());
        self.group_membership[shape_a] = Some(group_idx);
        self.group_membership[shape_b] = Some(group_idx);

        self.undo_stack.push(Command::CreateBooleanGroup(group_idx, group));
        self.redo_stack.clear();
    }

    pub fn dissolve_boolean_group(&mut self, group_idx: usize) {
        if group_idx >= self.boolean_groups.len() {
            return;
        }

        let group = self.boolean_groups.remove(group_idx);

        // Clear membership for shapes that were in this group
        for membership in &mut self.group_membership {
            if *membership == Some(group_idx) {
                *membership = None;
            } else if let Some(g) = membership {
                // Adjust indices for groups after the removed one
                if *g > group_idx {
                    *g -= 1;
                }
            }
        }

        self.undo_stack.push(Command::DissolveBooleanGroup(group_idx, group));
        self.redo_stack.clear();
    }

    pub fn change_boolean_op(&mut self, group_idx: usize, new_op: BoolOp) {
        if group_idx >= self.boolean_groups.len() {
            return;
        }

        let old_op = self.boolean_groups[group_idx].op;
        self.boolean_groups[group_idx].op = new_op;
        self.recompute_group(group_idx);

        self.undo_stack.push(Command::ChangeBooleanOp(group_idx, old_op, new_op));
        self.redo_stack.clear();
    }

    pub fn recompute_group(&mut self, group_idx: usize) {
        if group_idx >= self.boolean_groups.len() {
            return;
        }
        let group = &self.boolean_groups[group_idx];
        let shape_a = group.shape_a;
        let shape_b = group.shape_b;
        let op = group.op;

        if shape_a < self.shapes.len() && shape_b < self.shapes.len() {
            if let (Some(poly_a), Some(poly_b)) = (
                boolean::shape_to_polygon(&self.shapes[shape_a]),
                boolean::shape_to_polygon(&self.shapes[shape_b]),
            ) {
                self.boolean_groups[group_idx].cached_result =
                    boolean::compute_boolean(&poly_a, &poly_b, op);
            }
        }
    }

    pub fn undo(&mut self) {
        if let Some(cmd) = self.undo_stack.pop() {
            match &cmd {
                Command::AddShape(idx, _) => {
                    if *idx < self.shapes.len() {
                        self.shapes.remove(*idx);
                        self.group_membership.remove(*idx);
                    }
                }
                Command::RemoveShape(idx, shape) => {
                    let idx = (*idx).min(self.shapes.len());
                    self.shapes.insert(idx, shape.clone());
                    self.group_membership.insert(idx, None);
                }
                Command::MoveShape(idx, dx, dy) => {
                    if *idx < self.shapes.len() {
                        self.shapes[*idx].translate(-dx, -dy);
                        if let Some(g) = self.group_membership[*idx] {
                            self.recompute_group(g);
                        }
                    }
                }
                Command::UpdateShape(idx, old_shape, _new_shape) => {
                    if *idx < self.shapes.len() {
                        self.shapes[*idx] = old_shape.clone();
                        if let Some(g) = self.group_membership[*idx] {
                            self.recompute_group(g);
                        }
                    }
                }
                Command::CreateBooleanGroup(group_idx, _group) => {
                    if *group_idx < self.boolean_groups.len() {
                        let g = self.boolean_groups.remove(*group_idx);
                        // Clear membership
                        if g.shape_a < self.group_membership.len() {
                            self.group_membership[g.shape_a] = None;
                        }
                        if g.shape_b < self.group_membership.len() {
                            self.group_membership[g.shape_b] = None;
                        }
                        // Adjust remaining group indices
                        for membership in &mut self.group_membership {
                            if let Some(gi) = membership {
                                if *gi > *group_idx {
                                    *gi -= 1;
                                }
                            }
                        }
                    }
                }
                Command::DissolveBooleanGroup(group_idx, group) => {
                    // Re-create the group
                    let group_idx = *group_idx;
                    let shape_a = group.shape_a;
                    let shape_b = group.shape_b;
                    self.boolean_groups.insert(group_idx, group.clone());
                    // Adjust existing memberships that reference groups at or above the re-inserted index
                    for membership in &mut self.group_membership {
                        if let Some(gi) = membership {
                            if *gi >= group_idx {
                                *gi += 1;
                            }
                        }
                    }
                    // Restore membership for the group's shapes
                    if shape_a < self.group_membership.len() {
                        self.group_membership[shape_a] = Some(group_idx);
                    }
                    if shape_b < self.group_membership.len() {
                        self.group_membership[shape_b] = Some(group_idx);
                    }
                }
                Command::ChangeBooleanOp(group_idx, old_op, _new_op) => {
                    if *group_idx < self.boolean_groups.len() {
                        self.boolean_groups[*group_idx].op = *old_op;
                        self.recompute_group(*group_idx);
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
                    self.group_membership.push(None);
                }
                Command::RemoveShape(idx, _) => {
                    if *idx < self.shapes.len() {
                        self.shapes.remove(*idx);
                        self.group_membership.remove(*idx);
                    }
                }
                Command::MoveShape(idx, dx, dy) => {
                    if *idx < self.shapes.len() {
                        self.shapes[*idx].translate(*dx, *dy);
                        if let Some(g) = self.group_membership[*idx] {
                            self.recompute_group(g);
                        }
                    }
                }
                Command::UpdateShape(idx, _old_shape, new_shape) => {
                    if *idx < self.shapes.len() {
                        self.shapes[*idx] = new_shape.clone();
                        if let Some(g) = self.group_membership[*idx] {
                            self.recompute_group(g);
                        }
                    }
                }
                Command::CreateBooleanGroup(group_idx, group) => {
                    let group_idx = *group_idx;
                    self.boolean_groups.insert(group_idx, group.clone());
                    if group.shape_a < self.group_membership.len() {
                        self.group_membership[group.shape_a] = Some(group_idx);
                    }
                    if group.shape_b < self.group_membership.len() {
                        self.group_membership[group.shape_b] = Some(group_idx);
                    }
                }
                Command::DissolveBooleanGroup(group_idx, group) => {
                    if *group_idx < self.boolean_groups.len() {
                        self.boolean_groups.remove(*group_idx);
                        if group.shape_a < self.group_membership.len() {
                            self.group_membership[group.shape_a] = None;
                        }
                        if group.shape_b < self.group_membership.len() {
                            self.group_membership[group.shape_b] = None;
                        }
                        for membership in &mut self.group_membership {
                            if let Some(gi) = membership {
                                if *gi > *group_idx {
                                    *gi -= 1;
                                }
                            }
                        }
                    }
                }
                Command::ChangeBooleanOp(group_idx, _old_op, new_op) => {
                    if *group_idx < self.boolean_groups.len() {
                        self.boolean_groups[*group_idx].op = *new_op;
                        self.recompute_group(*group_idx);
                    }
                }
            }
            self.undo_stack.push(cmd);
        }
    }

    /// Hit test for free shapes only (not in boolean groups).
    pub fn hit_test(&self, point: Point) -> Option<usize> {
        for (i, shape) in self.shapes.iter().enumerate().rev() {
            if self.group_membership[i].is_some() {
                continue; // Skip grouped shapes
            }
            if shape.hit_test(point) {
                return Some(i);
            }
        }
        None
    }

    /// Hit test for boolean group results.
    pub fn hit_test_bool_group(&self, point: Point) -> Option<usize> {
        for (i, group) in self.boolean_groups.iter().enumerate().rev() {
            if boolean::hit_test_contours(point, &group.cached_result) {
                return Some(i);
            }
        }
        None
    }

    /// Combined hit test returning either a free shape or boolean group.
    pub fn hit_test_any(&self, point: Point) -> Option<HitTarget> {
        // Check boolean groups first (they render on top)
        if let Some(g) = self.hit_test_bool_group(point) {
            return Some(HitTarget::BoolGroup(g));
        }
        if let Some(s) = self.hit_test(point) {
            return Some(HitTarget::Shape(s));
        }
        None
    }

    /// Hit test for source shapes within a specific boolean group.
    pub fn hit_test_in_group(&self, point: Point, group_idx: usize) -> Option<usize> {
        if group_idx >= self.boolean_groups.len() {
            return None;
        }
        let group = &self.boolean_groups[group_idx];
        // Test shape_b first (drawn on top)
        if group.shape_b < self.shapes.len() && self.shapes[group.shape_b].hit_test(point) {
            return Some(group.shape_b);
        }
        if group.shape_a < self.shapes.len() && self.shapes[group.shape_a].hit_test(point) {
            return Some(group.shape_a);
        }
        None
    }
}
