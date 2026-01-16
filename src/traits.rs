use rand::Rng;

pub trait Mutable {
    fn mutate(&mut self);
}

pub trait GetRandomVariant {
    const VARIANT_COUNT: usize;
    
    fn get_rand_variant(self) -> Self;
    fn gen_idx_variant() -> usize {
        rand::thread_rng().gen_range(0..Self::VARIANT_COUNT)
    }
}
