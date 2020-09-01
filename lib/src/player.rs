use crate::dun_s1::*;
use crate::dun_s2::*;
use crate::mob::*;
use serde::{Serialize, Deserialize};
use rand::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub coords: (usize, usize), // y, x
    pub level: usize,
    pub in_fov: Vec<(usize, usize)>,
    pub memory: Vec<(usize, usize)>,
}

impl Player {
    // place player randomly on the first
    // level of the dungeon
    pub fn new<R>(lvl: &mut DungeonS2, r: &mut R, m: &MobTemplate) -> Player
    where
        R: Rng
    {
        let mob = m.generate_mob(r);
        let mut p_coords: Vec<(usize, usize)> = Vec::new();

        for y in 0..lvl.height {
            for x in 0..lvl.width {
                if lvl.d[y][x].tiletype == TileType::Floor {
                    p_coords.push((y, x));
                }
            }
        }

        p_coords.shuffle(r);

        for coord in &p_coords {
            if let Some(_) = lvl.d[coord.0][coord.1].mobs {
                // there's a mob here already, move on
                continue;
            } else {
                lvl.d[coord.0][coord.1].mobs = Some(mob);
                return Player {
                    coords: *coord,
                    level: 0usize,
                    in_fov: Vec::new(),
                    memory: Vec::new(),
                };
            }
        }

        // we still didn't get a place
        // maybe the whole dungeon's full of mobs?!
        // well anyways we'll just grab a random
        // coordinate, remove the inhabitant,
        // and place our luckless player there lol
        let coord = p_coords[r.gen_range(0, p_coords.len())];
        lvl.d[coord.0][coord.1].mobs = Some(mob);
        Player {
            coords: coord,
            level: 0,
            in_fov: Vec::new(),
            memory: Vec::new(),
        }
    }
}
