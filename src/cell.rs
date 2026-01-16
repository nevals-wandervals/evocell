use crate::genome::Genome;

pub type Family = usize;

#[derive(Debug)]
pub struct Cell {
    pub marker: MarkerCell,
    pub lifetime: u32,
    pub health: f32,
    pub energy: f32,
    pub toxin: f32,
    pub genome: Genome,
}

#[derive(Debug, Clone, Copy)]
pub enum MarkerCell {
    Global,
    Private(Family),
}
