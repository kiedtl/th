use serde::Deserialize;
use noise::{
    NoiseFn,
    Perlin,
    OpenSimplex,
    BasicMulti,
    HybridMulti,
    Fbm,
    Cylinders,
    Value,
    Seedable,
};
use num::{
    traits::SaturatingAdd,
    traits::SaturatingSub,
    traits::One, traits::Zero,
    clamp,
};

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub enum NoiseAlgorithm {
    Perlin,      // mines
    OpenSimplex, // misc
    BasicMulti,  // misc
    HybridMulti, // misc
    Fbm,         // misc
    Cylinders,   // melkor
    Value,       // mines
}

impl NoiseAlgorithm {
    pub fn as_noisefn(&mut self, seed: u32) -> Box<dyn NoiseFn<[f64; 2]>> {
        match self {
            NoiseAlgorithm::Perlin =>
                Box::new(Perlin::new().set_seed(seed)),
            NoiseAlgorithm::OpenSimplex =>
                Box::new(OpenSimplex::new().set_seed(seed)),
            NoiseAlgorithm::BasicMulti =>
                Box::new(BasicMulti::new().set_seed(seed)),
            NoiseAlgorithm::HybridMulti =>
                Box::new(HybridMulti::new().set_seed(seed)),
            NoiseAlgorithm::Fbm =>
                Box::new(Fbm::new().set_seed(seed)),
            NoiseAlgorithm::Cylinders => Box::new(Cylinders::new()),
            NoiseAlgorithm::Value =>
                Box::new(Value::new().set_seed(seed)),
        }
    }
}

pub fn get_all_neighbors<T>(width: T, height: T, x: T, y: T) -> [(T, T); 8]
where
    T: SaturatingAdd + SaturatingSub + One + Zero + Copy + PartialOrd
{
    let nwidth  = width  - T::one();
    let nheight = height - T::one();

    // all eight surrounding tiles
    // and yes, I understand that the indentation/alignment
    // for this function is horrible
    //
    // clamp values so that they don't exceed the height/width
    // of the map
    //
    // use saturating_sub/saturating_add so that the resulting
    // values don't go below zero
    [(      y.saturating_sub(&T::one()),                            x.saturating_sub(&T::one())),
     (      y.saturating_sub(&T::one()),                            x),
     (      y.saturating_sub(&T::one()),                      clamp(x.saturating_add(&T::one()), T::zero(), nwidth)),
     (      y,                                                      x.saturating_sub(&T::one())),
     (      y,                                                clamp(x.saturating_add(&T::one()), T::zero(), nwidth)),
     (clamp(y.saturating_add(&T::one()), T::zero(), nheight),       x.saturating_sub(&T::one())),
     (clamp(y.saturating_add(&T::one()), T::zero(), nheight),       x),
     (clamp(y.saturating_add(&T::one()), T::zero(), nheight), clamp(x.saturating_add(&T::one()), T::zero(), nwidth))]
}
