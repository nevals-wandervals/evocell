pub type Family = usize;

#[derive(Debug)]
pub struct Cell {
    pub marker: MarkerCell,
    pub lifetime: u32,
    pub energy: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum MarkerCell {
    Global,
    Private(Family),
}
