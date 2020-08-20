use crate::colors::*;
use crate::dun_s1::*;
use crate::dunspec::*;
use crate::features::*;
use crate::items::*;
use crate::material::*;
use crate::utils;
use noise::{NoiseFn, Seedable};
use rand::prelude::*;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Clone, Debug)]
pub struct DungeonTile {
    pub tiletype: TileType,
    pub tile_material: MaterialInfo,

    // TODO: define Mob, Item structs
    pub items: Vec<u8>,
    pub mobs: Vec<u8>,
}

// a DungeonS2 ("Dungeon Stage 2") has all
// the information that a DungeonS1 is missing,
// including items, mobs, material, etc
#[derive(Clone, Debug)]
pub struct DungeonS2 {
    pub d: Vec<Vec<DungeonTile>>,
    pub width: usize, pub height: usize,
    pub features: Vec<Feature>,
}

impl DungeonS2 {
    pub fn from_dungeon_s1(dg: &DungeonS1) -> DungeonS2 {
        let default_material: MaterialInfo = MaterialInfo {
            name: "morgothite".to_owned(),
            description: "a foul-smelling substance of unknown origins".to_owned(),
            class: MaterialClass::Other,
            rarity: 0,
            density: 4.8,
            color_fg: Color { red: 51, blue: 49, green: 40, alpha: 0 },
            color_bg: Color { red: 255, blue: 255, green: 159, alpha: 0 },
            block_glyph: '▒',
            melting_point: 1635.3722,
            combustible: None,
            smelt_result: None,
            hardness: 5.6,
            specific_heat: 4.189,
            emit_light: 0,
            occurs_naturally: Some(vec![ItemType::Block]),
            edible: false
        };

        let mut dungeon: Vec<Vec<DungeonTile>> = Vec::new();
        for y in 0..dg.height {
            let mut row = Vec::new();
            for x in 0..dg.width {
                let tile = DungeonTile {
                    tiletype: dg.d[y][x],
                    tile_material: default_material.clone(),
                    items: vec![], mobs: vec![],
                };

                row.push(tile);
            }

            dungeon.push(row);
        }

        DungeonS2 {
            d: dungeon,
            width: dg.width, height: dg.height,
            features: dg.features.clone(),
        }
    }

    // TODO: mineral_mapgen class/options struct
    // in other words, move this thing into a separate file :p
    pub fn decide_materials<N, R>(&mut self, materials: Vec<MaterialInfo>, noise: N,
        spec: &LayerSpecification, rng: &mut R)
    where
        R: Rng,
        N: NoiseFn<[f64; 2]> + Seedable,
    {
        // the exponent controls how often rare materials occur
        // on the map
        // the larger the integer, the rarer the materials
        // TODO: allow this to be controlled via fn arguments
        let exponent = 4.12;

        // arrange materials into a hashmap by rarity value
        let mut mats: HashMap<usize, Vec<MaterialInfo>> = HashMap::new();
        for i in materials {
            // ignore non-stone materials
            match &i.class {
                MaterialClass::Stone(info) => {
                    if info.stone_type != spec.stone_type {
                        continue;
                    }
                },
                _ => continue,
            }

            // ignore materials which don't occur naturally as
            // blocks (that is, walls)
            match &i.occurs_naturally {
                Some(occurs) => {
                    if !occurs.contains(&ItemType::Block) {
                        continue;
                    }
                },
                None => continue,
            }

            if !mats.contains_key(&(i.rarity as usize)) {
                mats.entry(i.rarity as usize).or_insert(Vec::new());
            }

            // insert
            mats.get_mut(&(i.rarity as usize)).unwrap().push(i.clone());
        };

        // iterate throughout map, choosig materials based on the noise
        // value
        for y in 0..self.height {
            for x in 0..self.width {
                let nx: f64 = (x as f64) / (self.width as f64)  - 0.5;
                let ny: f64 = (y as f64) / (self.height as f64) - 0.5;

                let noise =   1.0 * noise.get([ 1.0 * ny,  1.0 * nx])
                          +   0.5 * noise.get([ 2.0 * ny,  2.0 * nx])
                          +  0.25 * noise.get([ 4.0 * ny,  4.0 * nx])
                          + 0.125 * noise.get([ 8.0 * ny,  8.0 * nx])
                          + 0.062 * noise.get([16.0 * ny, 16.0 * nx])
                          + 0.031 * noise.get([32.0 * ny, 32.0 * nx]);
                // the value may be negative, so abs() it
                // multiply it by 255 to make it a value between 0..255
                let mut value = ((noise.abs().powf(exponent)) * 255.0) as usize;

                // if there's no material that has a rarity value equivalent
                // to the noise value, lower the noise value
                while !mats.contains_key(&value) {
                    value = value.saturating_sub(1);
                }

                // get a list of all the materials that this coord's
                // neighbors use
                let neighboring_mats = utils::get_all_neighbors(self.width, self.height, x, y)
                    .iter()
                    .map(|(ny, nx)| self.d[*ny][*nx].tile_material.clone())
                    .collect::<Vec<MaterialInfo>>();

                // if there are more than one material for the rarity value
                // then pick one based on what it's neighbors use
                // for example if we need to choose between granite
                // and basalt, and 5 neighbors use basalt but 3 use granite,
                // then basalt should be more likely to be picked
                self.d[y][x].tile_material = mats[&value].choose_weighted(rng, |m| {
                    let mut probability = 1.0;
                    for neighboring_mat in &neighboring_mats {
                        if neighboring_mat == m {
                            probability *= 2.5;
                        } else {
                            if score > 1.0 {
                                probability -= 0.5;
                            }
                        }
                    }

                    probability as usize
                }).unwrap().clone();
            }
        }
    }
}
