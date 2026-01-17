use crate::consts::RADIUS_PETRI_DISH;

const WIDTH_WORLD: usize = RADIUS_PETRI_DISH * 2;

#[inline(always)]
pub const fn get_index(pos: Position) -> usize {
    pos.y as usize * WIDTH_WORLD + pos.x as usize
}

#[inline(always)]
pub const fn get_position(index: usize) -> Position {
    Position::new(index % WIDTH_WORLD, index / WIDTH_WORLD)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub const fn to_index(self) -> usize {
        get_index(self)
    }

    #[inline(always)]
    pub fn x(&self) -> usize {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> usize {
        self.y
    }

    #[inline(always)]
    pub fn get_mut_x(&mut self) -> &mut usize {
        &mut self.x
    }

    #[inline(always)]
    pub fn get_mut_y(&mut self) -> &mut usize {
        &mut self.y
    }

    #[inline(always)]
    pub fn set_x(&mut self, value: usize) {
        self.x = value;
    }

    #[inline(always)]
    pub fn set_y(&mut self, value: usize) {
        self.y = value;
    }
}
