use variant_count::VariantCount;

use crate::{
    consts::COUNT_GENES,
    etc::is_mutated,
    traits::{GetRandomVariant, Mutable},
};

#[derive(Debug, Clone, Copy)]
pub struct Genome {
    pub step: usize,
    pub inner: [Gene; COUNT_GENES],
}

impl Genome {
    pub fn new() -> Self {
        Self {
            step: 0,
            inner: [Gene::default(); COUNT_GENES],
        }
    }

    pub fn get(&self) -> Gene {
        self.inner[self.step]
    }

    pub fn next(&mut self) {
        self.step += 1;
        if self.step >= self.inner.len() {
            self.step = 0;
        }
    }
}

impl Mutable for Genome {
    fn mutate(&mut self) {
        if is_mutated() {
            self.inner.iter_mut().for_each(|gene| gene.mutate());
        }
    }
}

#[derive(Debug, Clone, Copy, VariantCount)]
pub enum Gene {
    Move(Direction),
    MoveEnergy(Direction),
    Reproduction(Direction),
    Synthesis(TypeSynthesis),
    Attack,
}

impl Mutable for Gene {
    fn mutate(&mut self) {
        if is_mutated() {
            *self = self.get_rand_variant();
        }
    }
}

impl GetRandomVariant for Gene {
    const VARIANT_COUNT: usize = Self::VARIANT_COUNT;
    fn get_rand_variant(self) -> Self {
        match Self::gen_idx_variant() {
            0 => Self::Move(Direction::Down.get_rand_variant()),
            1 => Self::MoveEnergy(Direction::Down.get_rand_variant()),
            2 => Self::Reproduction(Direction::Down.get_rand_variant()),
            3 => Self::Synthesis(TypeSynthesis::Energy.get_rand_variant()),
            4 => Self::Attack,
            _ => unimplemented!(),
        }
    }
}

impl Default for Gene {
    fn default() -> Self {
        Self::Synthesis(TypeSynthesis::Energy)
    }
}

#[derive(Debug, Clone, Copy, VariantCount)]
pub enum TypeSynthesis {
    Energy,
    Toxin,
    Health,
}

impl Mutable for TypeSynthesis {
    fn mutate(&mut self) {
        if is_mutated() {
            *self = self.get_rand_variant();
        }
    }
}

impl GetRandomVariant for TypeSynthesis {
    const VARIANT_COUNT: usize = Self::VARIANT_COUNT;
    fn get_rand_variant(self) -> Self {
        match Self::gen_idx_variant() {
            0 => Self::Energy,
            1 => Self::Toxin,
            2 => Self::Health,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy, VariantCount)]
pub enum Direction {
    Left,
    Top,
    Right,
    Down,
}

impl Mutable for Direction {
    fn mutate(&mut self) {
        if is_mutated() {
            *self = self.get_rand_variant();
        }
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
            _ => unimplemented!(),
        }
    }
}
