pub trait Mutable {
    fn mutate(&mut self);
}

pub trait GetRandomVariant {
    fn get_rand_variant(self) -> Self;
}
