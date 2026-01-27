use crate::consts::RADIUS_PETRI_DISH;

const WIDTH_WORLD: i32 = RADIUS_PETRI_DISH * 2;

#[inline(always)]
pub const fn get_index(pos: Position) -> i32 {
    pos.y * WIDTH_WORLD + pos.x
}

// #[inline(always)]
// pub const fn get_position(index: usize) -> Position {
//     Position::new(index % WIDTH_WORLD, index / WIDTH_WORLD)
// }

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub const fn to_index(self) -> usize {
        get_index(self) as usize
    }

    #[inline(always)]
    pub fn x(&self) -> i32 {
        self.x
    }

    #[inline(always)]
    pub fn y(&self) -> i32 {
        self.y
    }

    #[inline(always)]
    pub fn get_mut_x(&mut self) -> &mut i32 {
        &mut self.x
    }

    #[inline(always)]
    pub fn get_mut_y(&mut self) -> &mut i32 {
        &mut self.y
    }

    #[inline(always)]
    pub fn set_x(&mut self, value: i32) {
        self.x = value;
    }

    #[inline(always)]
    pub fn set_y(&mut self, value: i32) {
        self.y = value;
    }
}
