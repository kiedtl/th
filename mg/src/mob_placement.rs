use crate::dun_s1::*;
use crate::dun_s2::*;
use crate::mob::*;
use crate::utils;
use crate::value;
use rand::prelude::*;
use serde::Deserialize;
use std::vec::Vec;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct MobPlacementOptions {
    noise_exponent: f64,
    allowed_classes: Vec<MobClass>,
    noise_algorithm: utils::NoiseAlgorithm,
    noise_overlap: usize, // TODO
    noise_seed: value::Value<u32>,
}

#[derive(Debug)]
pub struct MobPlacer<'a, R: Rng> {
    map: &'a mut DungeonS2,
    options: MobPlacementOptions,
    rng: &'a mut R,
}

impl<R: Rng> MobPlacer<'_, R> {
    pub fn new<'a>(
        map: &'a mut DungeonS2,
        options: MobPlacementOptions,
        rng: &'a mut R
    ) -> MobPlacer<'a, R> {
        MobPlacer {
            map: map,
            options: options,
            rng: rng,
        }
    }

    pub fn generate(&mut self, mobs: &mut Vec<MobTemplate>) {
        let noise = self.options.noise_algorithm
            .as_noisefn(self.rng.gen());

        // remove invalid mobs
        for mob_i in 0..mobs.len() {
            let mob = &mobs[mob_i];
            if !self.options.allowed_classes.contains(&mob.class) {
                mobs.remove(mob_i);
            }
        }

        // track how many of each mob is in the map
        let mut mob_ctr: Vec<usize> = vec![0; mobs.len()];

        // get list of all tiles in the map
        let mut coords: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                if self.map.d[y][x].tiletype == TileType::Floor {
                    coords.push((y, x));
                }
            }
        }

        // iterate through the map randomly
        // choosing materials based on the noise value for that coordinate
        coords.shuffle(self.rng);
        for coord in coords {
            let (y, x) = coord;
            let nx: f64 = (x as f64) / (self.map.width as f64)  - 0.5;
            let ny: f64 = (y as f64) / (self.map.height as f64) - 0.5;

            let mut noise: f64 =   1.0 * noise.get([ 1.0 * ny,  1.0 * nx]);
                               //+   0.5 * noise.get([ 2.0 * ny,  2.0 * nx])
                               //+  0.25 * noise.get([ 4.0 * ny,  4.0 * nx])
                               //+ 0.125 * noise.get([ 8.0 * ny,  8.0 * nx])
                               //+ 0.062 * noise.get([16.0 * ny, 16.0 * nx])
                               //+ 0.031 * noise.get([32.0 * ny, 32.0 * nx]);
            // the value may be negative, so abs() it
            noise = noise.abs().powf(self.options.noise_exponent);
            let mut value = noise.ceil() as usize * (mobs.len() * 2);

            let mut chosen_mob: Option<MobTemplate> = None;
            while value > 0 {
                if mobs.len() >= value {
                    if mob_ctr[value - 1] < mobs[value - 1].max_in_map {
                        chosen_mob = Some(mobs[value - 1].clone());
                        mob_ctr[value - 1] += 1;
                    }
                }

                value = value.saturating_sub(1);
            }

            if let Some(template) = chosen_mob {
                self.map.d[y][x].mobs.push(template.generate_mob(self.rng));
            } else {
                continue;
            }
        }
    }
}
