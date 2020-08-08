// TODO: add more features!

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug)]
pub enum Feature {
    HorizTunnel,
    VertiTunnel,
    Room,
}

impl Distribution<Feature> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Feature {
        match rng.gen_range(0, 5) {
            0 => Feature::HorizTunnel,
            1 => Feature::VertiTunnel,
            _ => Feature::Room,
        }
    }
}
