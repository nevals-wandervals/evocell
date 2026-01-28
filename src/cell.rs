use std::collections::HashSet;

use rand::Rng;

use crate::{
    etc::is_mutated,
    genome::{Genome, TypeSynthesis},
    math::{Direction, Position},
    traits::Mutable,
    world::World,
};

pub type Family = u8;

#[derive(Debug, Clone)]
pub struct Cell {
    pub family: Family,
    pub lifetime: u32,
    pub max_lifetime: u32,
    pub health: f32,
    pub energy: f32,
    pub toxin: f32,
    pub color: (u8, u8, u8),
    pub genome: Genome,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            family: rand::thread_rng().gen_range(0..255u8),
            lifetime: 0,
            max_lifetime: 16,
            health: 1.0,
            energy: 10.0,
            toxin: 0.0,
            color: (100, 100, 100),
            genome: Genome::new(),
        }
    }

    fn rand_color() -> (u8, u8, u8) {
        (
            rand::thread_rng().gen_range(50..200u8),
            rand::thread_rng().gen_range(50..200u8),
            rand::thread_rng().gen_range(50..200u8),
        )
    }

    pub fn reproduction(&mut self) -> Option<Self> {
        if self.energy > 2.5 {
            self.energy /= 2.0;
            self.lifetime = 0;
            self.toxin /= 2.0;

            let mut new_cell = self.clone();
            new_cell.genome.step = 0;
            new_cell.mutate();

            return Some(new_cell);
        }

        None
    }

    pub fn synthesize(&mut self, type_synthesis: TypeSynthesis) {
        match type_synthesis {
            TypeSynthesis::Energy => {
                self.energy += 5.0 / (self.energy * 2.25);
            }
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

    pub fn update_gravity(&mut self, self_pos: &mut Position, world: &mut World) {
        let (l_pos, r_pos) = (*self_pos + Direction::LeftDown, *self_pos + Direction::RightDown);
        let d_pos = *self_pos + Direction::Down;

        if !world.is_valid_pos(d_pos) {
            return;
        }

        let (valid_l, valid_r) = (world.is_valid_pos(l_pos), world.is_valid_pos(r_pos));

        if let None = world.get(d_pos) {
            *self_pos = d_pos;
        } else if valid_l  && let None = world.get(l_pos) {
            if valid_r && let None = world.get(r_pos) {
                let k = rand::thread_rng().gen_range(0..2u8);
                match k {
                    0 => *self_pos = l_pos,
                    1 => *self_pos = r_pos,
                    _=>{}
                }
            } else {
                *self_pos = l_pos;
            }
        } else if valid_r && let None = world.get(r_pos) {
            *self_pos = r_pos;
        }
    }

    pub fn update(&mut self, self_pos: &mut Position, world: &mut World) {
        self.update_gravity(self_pos, world);

        let gene = *self.genome.get();
        match gene {
            crate::genome::Gene::MovePosition(direction) => {
                if world.is_valid_pos(*self_pos + direction) {
                    *self_pos += direction
                }
            }
            crate::genome::Gene::MoveEnergy(direction) => {
                if world.is_valid_pos(*self_pos + direction) {
                    match world.get_mut(*self_pos + direction) {
                        Some(cell) => {
                            if self.family == cell.family {
                                if self.energy > cell.energy {
                                    let k = self.energy - cell.energy;
                                    self.energy -= k;
                                    cell.energy += k;
                                } else {
                                    let k = cell.energy - self.energy;
                                    self.energy += k;
                                    cell.energy -= k;
                                }
                            }
                        }
                        None => {}
                    }
                }
            }
            crate::genome::Gene::Reproduction(direction) => {
                if world.is_valid_pos(*self_pos + direction) {
                    let new_cell = self.reproduction();
                    match new_cell {
                        Some(cell) => {
                            let new_pos = *self_pos + direction;
                            world.add(new_pos, cell);
                        }
                        None => {}
                    }
                }
            }
            crate::genome::Gene::Synthesis(type_synthesis) => self.synthesize(type_synthesis),
            crate::genome::Gene::Attack(direction) => {
                // TODO:
                match world.get_mut(*self_pos + direction) {
                    Some(cell) => {
                        if self.family != cell.family {
                            let k = 3. * self.health;
                            self.energy += k;
                            cell.energy -= k;
                            cell.health -= k;
                        }
                    }
                    None => {}
                }
            }
            crate::genome::Gene::Stop => {
                self.genome.step = 0;
            }

            crate::genome::Gene::None => {
                // TODO:
            }
        }
        self.genome.next();
        self.energy -= 0.003 * self.max_lifetime as f32 * self.health;
        self.lifetime += 1;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime < self.max_lifetime
            && self.energy >= 1.3
            && self.energy < 1000.0
            && self.health > 0.0
    }
}

impl Mutable for Cell {
    fn mutate(&mut self) -> bool {
        if is_mutated(1.0) {
            self.genome.mutate();
            self.color = Self::rand_color();
            self.family = rand::thread_rng().gen_range(0..255u8);
            self.max_lifetime = rand::thread_rng().gen_range(0..1000);

            return true;
        }

        false
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MarkerCell {
    Global,
    Private(Family),
}
