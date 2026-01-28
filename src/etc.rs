use rand::Rng;

use crate::consts::PROBABILITY_OF_MUTATION;

#[inline(always)]
pub fn is_mutated(pow: f64) -> bool {
    rand::thread_rng().gen_bool(PROBABILITY_OF_MUTATION * pow)
}
