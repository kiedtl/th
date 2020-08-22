use crate::colors::*;
use crate::dun_s1::*;
use crate::features::*;
use crate::items::*;
use crate::material::*;
use crate::mob::*;
use std::vec::Vec;

#[derive(Clone, Debug)]
pub struct DungeonTile {
    pub tiletype: TileType,
    pub tile_material: MaterialInfo,

    // TODO: define Item structs
    pub items: Vec<u8>,
    pub mobs: Vec<Mob>,
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
}
