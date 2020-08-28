// the final finished dungeon in its final format
// yay

use chrono::prelude::*;
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
}
