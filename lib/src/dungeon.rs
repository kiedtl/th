// the final finished dungeon in its final format
// yay

use chrono::prelude::*;
use crate::dun_s2::*;
use serde::{Serialize, Deserialize};
use std::vec::Vec;

#[derive(Clone, Serialize, Deserialize)]
pub struct Dungeon {
    pub world_name: String,
    pub created_on: i64,
    pub levels: Vec<DungeonS2>,
}

impl Dungeon {
    pub fn from_dungeon_s2(name: String, lvls: Vec<DungeonS2>) -> Dungeon {
        Dungeon {
            world_name: name,
            created_on: Local::now().timestamp(),
            levels: lvls,
        }
    }
}
