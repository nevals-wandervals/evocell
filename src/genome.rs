use variant_count::VariantCount;
use variantly::Variantly;

use crate::{
    consts::COUNT_GENES,
    etc::is_mutated,
    math::Direction,
    traits::{GetRandomVariant, Mutable},
};

#[derive(Debug, Clone, Copy)]
pub struct Genome {
    pub step: usize,
    inner: [Gene; COUNT_GENES],
}

impl Genome {
    pub fn new() -> Self {
        let mut genes = [Gene::default(); COUNT_GENES];
        genes[0] = Gene::Synthesis(TypeSynthesis::Energy);
        genes[1] = Gene::Synthesis(TypeSynthesis::Energy);
        genes[3] = Gene::Reproduction(Direction::Down);
        genes[4] = Gene::Synthesis(TypeSynthesis::Energy);
        genes[5] = Gene::MoveEnergy(Direction::Right);
        genes[6] = Gene::Synthesis(TypeSynthesis::Energy);
        genes[7] = Gene::Reproduction(Direction::Right);
        genes[8] = Gene::Synthesis(TypeSynthesis::Energy);
        genes[9] = Gene::Reproduction(Direction::Left);
        genes[10] = Gene::Synthesis(TypeSynthesis::Energy);
        genes[11] = Gene::Reproduction(Direction::Top);
        genes[12] = Gene::Stop;

        Self {
            step: 0,
            inner: genes,
        }
    }

    #[inline]
    pub fn get(&self) -> &Gene {
        &self.inner[self.step]
    }

    #[inline]
    pub fn next(&mut self) {
        self.step += 1;
        if self.step >= self.inner.len() {
            self.step = 0;
        }
    }
}

impl Mutable for Genome {
    fn mutate(&mut self) -> bool {
        self.inner.iter_mut().for_each(|gene| {
            gene.mutate();
        });
        self.step = 0;

        true
    }
}

#[derive(Debug, Clone, Copy, VariantCount, Variantly)]
pub enum Gene {
    MovePosition(Direction),
    MoveEnergy(Direction),
    Reproduction(Direction),
    Synthesis(TypeSynthesis),
    Attack(Direction),
    Stop,
    None,
}

impl Mutable for Gene {
    fn mutate(&mut self) -> bool {
        if is_mutated() {
            *self = self.get_rand_variant();

            return true;
        }

        false
    }
}

impl GetRandomVariant for Gene {
    const VARIANT_COUNT: usize = Self::VARIANT_COUNT;

    fn get_rand_variant(self) -> Self {
        match Self::gen_idx_variant() {
            0 => Self::MovePosition(Direction::Down.get_rand_variant()),
            1 => Self::MoveEnergy(Direction::Down.get_rand_variant()),
            2 => Self::Reproduction(Direction::Down.get_rand_variant()),
            3 => Self::Synthesis(TypeSynthesis::Energy.get_rand_variant()),
            4 => Self::Attack(Direction::Down.get_rand_variant()),
            5 => Self::Stop,
            6 => Self::None,
            idx => panic!("Unknown variant index: {};", idx),
        }
    }
}

impl Default for Gene {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, VariantCount, Variantly)]
pub enum TypeSynthesis {
    Energy,
    Toxin,
    Health,
}

impl Mutable for TypeSynthesis {
    fn mutate(&mut self) -> bool {
        if is_mutated() {
            *self = self.get_rand_variant();
            return true;
        }

        false
    }
}

impl GetRandomVariant for TypeSynthesis {
    const VARIANT_COUNT: usize = Self::VARIANT_COUNT;

    fn get_rand_variant(self) -> Self {
        match Self::gen_idx_variant() {
            0 => Self::Energy,
            1 => Self::Toxin,
            2 => Self::Health,
            idx => panic!("Unknown variant index: {};", idx),
        }
    }
}
