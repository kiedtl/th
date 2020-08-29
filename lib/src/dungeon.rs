// the final finished dungeon in its final format
// yay

use chrono::prelude::*;
use crate::dun_s1::*;
use crate::dun_s2::*;
use crate::player::*;
use crate::mob::*;
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use std::vec::Vec;

#[derive(Clone, Serialize, Deserialize)]
pub struct Dungeon {
    pub world_name: String,
    pub created_on: i64,
    pub levels: Vec<DungeonS2>,
    pub player: Player,
}

impl Dungeon {
    // create dungeon and place player yay
    pub fn from_dungeon_s2<R>(name: String, lvls: &mut Vec<DungeonS2>,
        rng: &mut R, mob: &MobTemplate) -> Dungeon
    where
        R: Rng
    {
        let player = Player::new(&mut lvls[0], rng, mob);
        Dungeon {
            world_name: name,
            created_on: Local::now().timestamp(),
            levels: lvls.to_vec(),
            player: player,
        }
    }

    pub fn move_mob(
        &mut self,
        oldlevel: usize, old_pos: (usize, usize),
        newlevel: usize, new_pos: (usize, usize), swap: bool
    ) -> Result<(), ()> {
        assert!(newlevel < self.levels.len());
        assert!(new_pos.0 < self.levels[newlevel].height);
        assert!(new_pos.1 < self.levels[newlevel].width);

        assert!(self.levels[newlevel].d[new_pos.0][new_pos.1].tiletype
            == TileType::Floor);

        if let Some(mob) = self.at(oldlevel, old_pos).mobs {
            if let Some(othermod) = self.at(newlevel, new_pos).mobs {
                if !swap {
                    return Err(());
                }

                self.levels[oldlevel].d[old_pos.0][old_pos.1].mobs = Some(othermod);
                self.levels[newlevel].d[new_pos.0][new_pos.1].mobs = Some(mob);
            } else {
                self.levels[newlevel].d[new_pos.0][new_pos.1].mobs = Some(mob);
                self.levels[oldlevel].d[old_pos.0][old_pos.1].mobs = None;
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn at(&self, level: usize, coords: (usize, usize)) -> DungeonTile {
        self.levels[level].d[coords.0][coords.1].clone()
    }
}
