use std::vec::Vec;
use crate::materials::*;
use rand::prelude::*;
use crate::dun_s1::*;

#[derive(Clone, Debug)]
pub struct DungeonTile {
    tiletype: TileType,
    tile_material: &Material,

    // TODO: define Mob, Item structs
    items: Vec<u8>,
    mobs: Vec<u8>,
}

// a DungeonS2 ("Dungeon Stage 2") has all
// the information that a DungeonS1 is missing,
// including items, mobs, material, etc
#[derive(Clone, Debug)]
pub struct DungeonS2 {
    d: Vec<Vec<DungeonTile>>,
    width: usize, height: usize,
    features: Vec<Feature>,
}

impl DungeonS2 {
    pub fn from_dungeon_s1(dg: &DungeonS1) -> DungeonS2 {
        let default_material: MaterialInfo = MaterialInfo {
            name: "morgothite".to_owned(),
            description: "a foul-smelling substance of unknown origins".to_owned(),
            class: MaterialClass::Other,
            rarity: 0,
            density: 4.8,
            color: Color { red: 51, blue: 49, green: 40, alpha: 0 },
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
        for y in 0..(dg.height - 1) {
            let mut row = Vec::new();
            for x in 0..(dg.width - 1) {
                let tile = DungeonTile {
                    tiletype: dg.d[y][x],
                    tile_material: default_material,
                    items: vec![], mobs: vec![],
                }

                row.push(tile);
            }

            dungeon.push(row);
        }

        DungeonS2 {
            d: dungeon,
            width: dg.width, height: dg.height,
            features: dg.features,
        }
    }

    pub fn decide_materials(&mut self, materials: Vec<Materials>, noise_algo: String) {
    }
}
