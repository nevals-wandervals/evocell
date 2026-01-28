use std::collections::HashMap;

use crate::{cell::Cell, consts::RADIUS_PETRI_DISH, math::Position};

pub const WIDTH: i32 = 360;
pub const HEIGHT: i32 = RADIUS_PETRI_DISH * 2;

pub struct World {
    active_cells: HashMap<Position, Cell>,
    buffer: HashMap<Position, Cell>,
    width: i32,
    height: i32,
}

impl World {
    pub fn new() -> Self {
        Self {
            active_cells: HashMap::new(),
            buffer: HashMap::new(),
            width: WIDTH,
            height: HEIGHT,
        }
    }

    #[inline(always)]
    pub fn is_valid_pos(&self, pos: Position) -> bool {
        pos.x() < self.width && pos.y() < self.height && pos.x() > 0 && pos.y() > 0
    }

    #[inline(always)]
    pub fn get(&self, pos: Position) -> Option<&Cell> {
        self.active_cells.get(&pos)
    }

    #[inline(always)]
    pub fn get_mut(&mut self, pos: Position) -> Option<&mut Cell> {
        self.buffer.get_mut(&pos)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Position, &Cell)> {
        self.active_cells.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Position, &mut Cell)> {
        self.buffer.iter_mut()
    }

    pub fn count_cells(&self) -> usize {
        self.active_cells.len()
    }

    /// true - added
    /// false dont added
    pub fn add(&mut self, pos: Position, cell: Cell) -> bool {
        self.with_valid_pos(pos, |buffer| buffer.insert(pos, cell).is_none())
    }

    /// true - del
    /// false - no del
    pub fn del(&mut self, pos: Position) -> bool {
        self.with_valid_pos(pos, |buffer| buffer.remove(&pos).is_some())
    }

    fn with_valid_pos<F>(&mut self, pos: Position, f: F) -> bool
    where
        F: FnOnce(&mut HashMap<Position, Cell>) -> bool,
    {
        if !self.is_valid_pos(pos) {
            return false;
        }
        f(&mut self.buffer)
    }

    pub fn update(&mut self) {
        let mut poss: Vec<Position> = self.active_cells.keys().cloned().collect();
        for pos in poss.iter_mut() {
            let mut cell = *self.get(*pos).unwrap();
            cell.update(pos, self);

            if cell.is_alive() {
                self.add(*pos, cell);
            }
        }

        self.active_cells = std::mem::take(&mut self.buffer);
    }
}
