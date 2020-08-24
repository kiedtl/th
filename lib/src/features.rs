// TODO: add more features!

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Serialize, Deserialize};
use crate::rect::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Feature {
    Tunnel(Rect),
    Room(Rect),
}

impl Distribution<Feature> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Feature {
        match rng.gen_range(0, 1) {
            0 => Feature::Tunnel(Rect::new(0, 0, 0, 0)),
            _ => Feature::Room(Rect::new(0, 0, 0, 0)),
        }
    }
}
