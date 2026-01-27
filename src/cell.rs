use std::collections::HashSet;

use rand::Rng;

use crate::{
    genome::{Genome, TypeSynthesis},
    math::Position,
    traits::Mutable,
    world::World,
};

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
            color: Self::rand_color(),
            genome: Genome::new(),
        }
    }

    fn rand_color() -> (u8, u8, u8) {
        (
            rand::thread_rng().gen_range(50..200u8),
            rand::thread_rng().gen_range(50..200u8),
            rand::thread_rng().gen_range(50..200u8)
        )
    }
 
    pub fn reproduction(&mut self) -> Option<Self> {
        if self.energy > 2.5 {
            self.energy /= 2.0;
            self.lifetime = 0;
            self.toxin /= 2.0;

            let mut new_cell = self.clone();
            let is_mutate = new_cell.genome.mutate();
            if is_mutate {
                new_cell.color = Self::rand_color();
            }

            return Some(new_cell);
        }

        None
    }

    pub fn synthesize(&mut self, type_synthesis: TypeSynthesis) {
        match type_synthesis {
            TypeSynthesis::Energy => self.energy += 5.89,
            TypeSynthesis::Toxin => {
                self.energy -= 1.0;
                self.toxin += 1.0;
            }
            TypeSynthesis::Health => {
                self.energy -= 1.0;
                self.health += 1.0;
            }
        }
    }

    pub fn update(&mut self, self_pos: &mut Position, world: &mut World) {
        let gene = *self.genome.get();
        match gene {
            crate::genome::Gene::MovePosition(direction) => *self_pos += direction,
            crate::genome::Gene::MoveEnergy(direction) => {
                // match world.get_mut(*self_pos + direction) {
                //     Some(cell) => {
                //         // TODO: Family
                //         self.energy /= 2.0;
                //         cell.energy += self.energy;
                //     },
                //     None => {}
                // }
            }
            crate::genome::Gene::Reproduction(direction) => {
                let new_cell = self.reproduction();
                match new_cell {
                    Some(cell) => {
                        let new_pos = *self_pos + direction;
                        world.add(new_pos, cell);
                    }
                    None => {}
                }
            }
            crate::genome::Gene::Synthesis(type_synthesis) => self.synthesize(type_synthesis),
            crate::genome::Gene::Attack => {
                // TODO:
            }
            crate::genome::Gene::Stop => {
                // TODO:
            }
        }
        self.genome.next();
        self.lifetime += 1;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime < 256 && self.energy > 2.0 //&& self.health > 5.0
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MarkerCell {
    Global,
    Private(Family),
}
