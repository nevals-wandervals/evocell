use crate::pos;
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
        let (x, y) = rhs.into();
        self.x += x;
        self.y += y;
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Position;
    fn add(self, rhs: Direction) -> Self::Output {
        self + Into::<(i32, i32)>::into(rhs)
    }
}

impl std::ops::Add<(i32, i32)> for Position {
    type Output = Position;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        pos!(self.x + rhs.0, self.y + rhs.1)
    }
}

#[derive(Debug, Clone, Copy, VariantCount, Variantly)]
pub enum Direction {
    LeftDown,
    Left,
    LeftTop,
    Top,
    RightTop,
    Right,
    RightDown,
    Down,
}

impl Into<(i32, i32)> for Direction {
    fn into(self) -> (i32, i32) {
        match self {
            Direction::LeftDown => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::LeftTop => (-1, -1),
            Direction::Top => (0, -1),
            Direction::RightTop => (1, -1),
            Direction::Right => (1, 0),
            Direction::RightDown => (1, 1),
            Direction::Down => (0, 1),
        }
    }
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
            0 => Self::LeftDown,
            1 => Self::Left,
            2 => Self::LeftTop,
            3 => Self::Top,
            4 => Self::RightTop,
            5 => Self::Right,
            6 => Self::RightDown,
            7 => Self::Down,
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
