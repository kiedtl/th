use crate::dun_s1::*;
use crate::features::*;
use crate::mob::*;
use serde::{Serialize, Deserialize};
use std::vec::Vec;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DungeonTile {
    pub tiletype: TileType,
    pub tile_material: String,

    // TODO: define Item structs
    pub items: Vec<u8>,
    pub mobs: Option<Mob>,

    // in kelvin, of course
    pub temperature: f64,
}

// a DungeonS2 ("Dungeon Stage 2") has all
// the information that a DungeonS1 is missing,
// including items, mobs, material, etc
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DungeonS2 {
    pub d: Vec<Vec<DungeonTile>>,
    pub width: usize, pub height: usize,
    pub features: Vec<Feature>,
}

impl DungeonS2 {
    pub fn from_dungeon_s1(dg: &DungeonS1) -> DungeonS2 {
        let mut dungeon: Vec<Vec<DungeonTile>> = Vec::new();
        for y in 0..dg.height {
            let mut row = Vec::new();
            for x in 0..dg.width {
                let tile = DungeonTile {
                    tiletype: dg.d[y][x],
                    tile_material: "".to_string(),
                    items: vec![], mobs: None,
                    temperature: 303.15, // 85°F
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
}
