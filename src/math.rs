use variant_count::VariantCount;
use variantly::Variantly;

use crate::{
    consts::RADIUS_PETRI_DISH,
    etc::is_mutated,
    traits::{GetRandomVariant, Mutable},
};

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

impl std::ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Left => self.x -= 1,
            Direction::Top => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
        }
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Self::Output {
        let (x, y) = match rhs {
            Direction::Left => (-1, 0),
            Direction::Top => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
        };

        Position {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

#[derive(Debug, Clone, Copy, VariantCount, Variantly)]
pub enum Direction {
    Left,
    Top,
    Right,
    Down,
}

impl Mutable for Direction {
    fn mutate(&mut self) -> bool {
        if is_mutated() {
            *self = self.get_rand_variant();
            return true;
        }

        false
    }
}

impl GetRandomVariant for Direction {
    const VARIANT_COUNT: usize = Self::VARIANT_COUNT;

    fn get_rand_variant(self) -> Self {
        match Self::gen_idx_variant() {
            0 => Self::Left,
            1 => Self::Top,
            2 => Self::Right,
            3 => Self::Down,
            idx => panic!("Unknown variant index: {};", idx),
        }
    }
}

#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        Position::new($x, $y)
    };
}
