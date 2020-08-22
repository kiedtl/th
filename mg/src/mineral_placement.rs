use crate::dun_s2::*;
use crate::material::*;
use crate::items::*;
use crate::utils;
use crate::value;
use rand::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub struct MineralPlacementOptions {
    noise_exponent: f64,
    allowed_stone: StoneType,
    noise_algorithm: utils::NoiseAlgorithm,
    noise_overlap: usize, // TODO
    noise_seed: value::Value<u32>,
}

#[derive(Debug)]
pub struct MineralPlacer<'a, R: Rng> {
    map: &'a mut DungeonS2,
    options: MineralPlacementOptions,
    rng: &'a mut R,
}

impl<R: Rng> MineralPlacer<'_, R> {
    pub fn new<'a>(
        map: &'a mut DungeonS2,
        options: MineralPlacementOptions,
        rng: &'a mut R
    ) -> MineralPlacer<'a, R> {
        MineralPlacer {
            map: map,
            options: options,
            rng: rng,
        }
    }

    pub fn generate(&mut self, materials: Vec<MaterialInfo>) {
        let noise = self.options.noise_algorithm
            .as_noisefn(self.rng.gen());

        // arrange materials into a hashmap by rarity value
        let mut mats: HashMap<usize, Vec<MaterialInfo>> = HashMap::new();
        for i in materials {
            // ignore materials which don't occur naturally as
            // blocks/walls
            match &i.occurs_naturally {
                Some(occurs) => {
                    if !occurs.contains(&ItemType::Block) {
                        continue;
                    }
                },
                None => continue,
            }

            // ignore non-stone materials and stones that
            // aren't of the correct type
            match &i.class {
                MaterialClass::Stone(info) => {
                    if info.stone_type != self.options.allowed_stone {
                        continue;
                    }
                },
                _ => continue,
            }

            // insert
            if !mats.contains_key(&(i.rarity as usize)) {
                mats.entry(i.rarity as usize).or_insert(Vec::new());
            }
            mats.get_mut(&(i.rarity as usize)).unwrap().push(i.clone());
        };

        // get list of all tiles in the map
        let mut coords: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.map.height {
            for x in 0..self.map.width {
                coords.push((y, x));
            }
        }

        // iterate through the map
        // choosing materials based on the noise value for that coordinate
        //coords.shuffle(self.rng);
        for coord in coords {
            let (y, x) = coord;
            let nx: f64 = (x as f64) / (self.map.width as f64)  - 0.5;
            let ny: f64 = (y as f64) / (self.map.height as f64) - 0.5;

            let noise: f64 =   1.0 * noise.get([ 1.0 * ny,  1.0 * nx])
                           +   0.5 * noise.get([ 2.0 * ny,  2.0 * nx])
                           +  0.25 * noise.get([ 4.0 * ny,  4.0 * nx])
                           + 0.125 * noise.get([ 8.0 * ny,  8.0 * nx])
                           + 0.062 * noise.get([16.0 * ny, 16.0 * nx])
                           + 0.031 * noise.get([32.0 * ny, 32.0 * nx]);
            // the value may be negative, so abs() it
            // multiply it by 255 to make it a value between 0..255
            let mut value = ((noise.abs().powf(self.options.noise_exponent)) * 255.0) as usize;

            // get a list of all the materials that this coord's
            // neighbors use
            let neighboring_mats = utils::get_all_neighbors(self.map.width, self.map.height, x, y)
                .iter()
                .map(|(ny, nx)| self.map.d[*ny][*nx].tile_material.clone())
                .collect::<Vec<MaterialInfo>>();

            // helper function to check if a material can be placed
            let has_correct_neighbors = |m: &MaterialInfo| {
                // if there are items in the found_near list,
                // and none of the neighbors use a material in that list,
                // then the material cannot be used
                let found_near = m.found_near().unwrap();
                if found_near.len() > 0 {
                    let mut has_correct_neighbors = false;
                    for neighboring_mat in &neighboring_mats {
                        if found_near.contains(&m.name) ||
                            found_near.contains(&neighboring_mat.name) {
                                has_correct_neighbors = true;
                        }
                    }
                  return has_correct_neighbors;
                }
                true
            };

            // if there's no material that has a rarity value equivalent
            // to the noise value, or none of the materials in that
            // rarity group can be placed due to not having correct neighbors,
            // lower the noise value.
            loop {
                if mats.contains_key(&value) {
                    let mut one_have_correct_neighbors = false;
                    for mat in &mats[&value] {
                        if has_correct_neighbors(mat) {
                            one_have_correct_neighbors = true;
                        }
                    }

                    if one_have_correct_neighbors {
                        break;
                    }
                }

                value = value.saturating_sub(1);
            }

            // if there are more than one material for the rarity value
            // then pick one based on what it's neighbors use
            // for example if we need to choose between granite
            // and basalt, and 5 neighbors use basalt but 3 use granite,
            // then basalt should be more likely to be picked
            self.map.d[y][x].tile_material = mats[&value].choose_weighted(self.rng, |m| {
                let mut probability: f64 = 1.0;

                for neighboring_mat in &neighboring_mats {
                    if neighboring_mat == m {
                        probability *= 2.6;
                    } else {
                        if probability > 0.4 {
                            probability /= 1.9;
                        }
                    }
                }

                probability
            }).unwrap().clone();
        }
    }
}
