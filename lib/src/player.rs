use crate::dun_s1::*;
use crate::dun_s2::*;
use crate::mob::*;
use serde::{Serialize, Deserialize};
use rand::prelude::*;
use crate::utils;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub coords: (usize, usize),
    pub level: usize,
    pub in_fov: Vec<(usize, usize)>,
    pub memory: Vec<(usize, usize)>,
}

impl Player {
    // find a suitable space for the player
    // and dump it there
    //
    // note that the only checks done to ensure a tile
    // is a "suitable" tile is to ensure its empty (that is, no wall there),
    // and that there isn't already another mob there.
    // thus, the "suitable" spot may end up being pretty dangerous,
    // for example, right in a pool of magma,
    // or maybe in the middle of a hord of burning brutes.
    pub fn new<R>(level: &mut DungeonS2, mob_table: &mut HashMap<u64, Mob>,
        level_no: usize, r: &mut R, m: &MobTemplate) -> Player
    where
        R: Rng
    {
        let mob = m.generate_mob(r);
        let mob_id = utils::calculate_hash(&mob);

        // add mob to mob table
        mob_table.insert(mob_id, mob);

        // try and find a place we can place the player

        let mut p_coords: Vec<(usize, usize)> = Vec::new();
        // iterate through each non-wall tile on the dungeon in random order,
        // until we find a suitable tile
        for y in 0..level.height {
            for x in 0..level.width {
                if level.d[y][x].tiletype == TileType::Floor {
                    p_coords.push((y, x));
                }
            }
        }

        p_coords.shuffle(r);

        for coord in &p_coords {
            if let Some(_) = level.d[coord.0][coord.1].mobs {
                // there's a mob here already, move on
                continue;
            } else {
                level.d[coord.0][coord.1].mobs = Some(mob_id);
                return Player {
                    coords: *coord,
                    level: level_no,
                    in_fov: Vec::new(),
                    memory: Vec::new(),
                };
            }
        }

        // we still didn't get a place
        // maybe the whole dungeon's full of mobs?!
        // well anyways we'll just grab a random
        // coordinate, remove the inhabitant,
        // and place our luckless player there
        let coord = p_coords[r.gen_range(0, p_coords.len())];
        level.d[coord.0][coord.1].mobs = Some(mob_id);
        Player {
            coords: coord,
            level: level_no,
            in_fov: Vec::new(),
            memory: Vec::new(),
        }
    }
}
