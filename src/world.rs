use std::collections::HashSet;

use crate::{cell::Cell, consts::RADIUS_PETRI_DISH, math::Position};

const WIDTH: usize = RADIUS_PETRI_DISH * 2;
const HEIGHT: usize = RADIUS_PETRI_DISH * 2;

pub struct World {
    active_cells: HashSet<Position>,
    grid: [Option<Cell>; WIDTH * HEIGHT],
}

impl World {
    pub fn new() -> Self {
        Self {
            active_cells: HashSet::new(),
            grid: [const { None }; _],
        }
    }

    #[inline(always)]
    pub fn is_valid_index(&self, index: usize) -> bool {
        index < WIDTH * HEIGHT
    }

    #[inline(always)]
    pub fn is_valid_pos(&self, pos: Position) -> bool {
        pos.x() < WIDTH && pos.y() < HEIGHT
    }

    /// Получение ячейки из сетки, может использоваться с `is_valid_*()` в случаях,
    /// когда есть риск выхода за пределы или же чтобы убедиться, что данная ячейка крайняя.
    #[inline(always)]
    pub fn get(&self, index: usize) -> &Option<Cell> {
        &self.grid[index]
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> &mut Option<Cell> {
        &mut self.grid[index]
    }

    #[inline(always)]
    pub fn get_by_pos(&self, pos: Position) -> &Option<Cell> {
        &self.grid[pos.to_index()]
    }

    #[inline(always)]
    pub fn get_mut_by_pos(&mut self, pos: Position) -> &mut Option<Cell> {
        &mut self.grid[pos.to_index()]
    }

    pub fn add(&mut self, pos: Position, cell: Cell) -> Result<(), Cell> {
        if !self.is_valid_pos(pos) {
            return Err(cell);
        }

        if self.active_cells.insert(pos) {
            let idx = pos.to_index();
            self.grid[idx] = Some(cell);
            return Ok(());
        }

        Err(cell)
    }

    /// При попытки удалеения активной клетки, которой нет, или же позиция находится за пределами
    /// массива, результат один `Err(())`.
    /// Если нет вмешательств из вне, или в сам метод `del()`, то вызов `.unwrap()`, будет инвариантным.
    pub fn del(&mut self, pos: Position) -> Result<Cell, ()> {
        if self.active_cells.remove(&pos) {
            let idx = pos.to_index();
            return Ok(std::mem::take(&mut self.grid[idx]).unwrap());
        }

        Err(())
    }
}
