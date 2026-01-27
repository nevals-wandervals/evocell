use std::collections::HashSet;

use rand::Rng;

use crate::{genome::Genome, math::Position};

pub type Family = usize;

#[derive(Debug, Clone)]
pub struct Cell {
    pub marker: MarkerCell,
    pub lifetime: u32,
    pub health: f32,
    pub energy: f32,
    pub toxin: f32,
    pub color: (u8, u8, u8),
    pub genome: Genome,
}

impl Cell {
    pub fn new() -> Self {
        let k_color = rand::thread_rng().gen_range(50..200u8);

        Self {
            marker: MarkerCell::Global,
            lifetime: 0,
            health: 10.0,
            energy: 10.0,
            toxin: 0.0,
            color: (k_color, k_color, k_color),
            genome: Genome::new(),
        }
    }

    pub fn update(
        &mut self,
        active_cells: &mut HashSet<Position>,
        grid: &mut Vec<Option<Cell>>,
        self_pos: Position,
    ) {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MarkerCell {
    Global,
    Private(Family),
}
