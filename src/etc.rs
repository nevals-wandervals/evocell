use rand::Rng;

use crate::consts::PROBABILITY_OF_MUTATION;

pub fn is_mutated() -> bool {
    rand::thread_rng().gen_bool(PROBABILITY_OF_MUTATION)
}
