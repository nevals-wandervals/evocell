use rand::Rng;

use crate::{
    consts::COUNT_GENES,
    etc::is_mutated,
    traits::{GetRandomVariant, Mutable},
};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
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
    fn get_rand_variant(self) -> Self {
        let idx = rand::thread_rng().gen_range(0..5);
        match idx {
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

#[derive(Debug, Clone, Copy)]
pub enum TypeSynthesis {
    Energy,
    Toxin,
}

impl Mutable for TypeSynthesis {
    fn mutate(&mut self) {
        if is_mutated() {
            *self = self.get_rand_variant();
        }
    }
}

impl GetRandomVariant for TypeSynthesis {
    fn get_rand_variant(self) -> Self {
        let idx = rand::thread_rng().gen_range(0..3);
        match idx {
            0 => Self::Energy,
            1 => Self::Toxin,
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
    fn get_rand_variant(self) -> Self {
        let idx = rand::thread_rng().gen_range(0..4);
        match idx {
            0 => Self::Left,
            1 => Self::Top,
            2 => Self::Right,
            3 => Self::Down,
            _ => unimplemented!(),
        }
    }
}
