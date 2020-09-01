use crate::coord::*;
use crate::kbd::*;
use crate::message::*;
use lib::priority::*;
use lib::dun_s1::*;
use lib::dungeon::Dungeon;
use std::error::Error;
use ron::de::from_reader;
use std::{fs::File, fs};

// this contains the entire state of the game
pub struct State {
    pub dungeon: Dungeon,
    pub messages: Vec<Message>,
    pub map_path: String,
}

impl State {
    pub fn from_file(path: &str) -> Result<State, Box<dyn Error>> {
        let fmap = File::open(path.to_string())?;
        let dungeon: Dungeon = from_reader(fmap)?;

        Ok(State {
            dungeon: dungeon,
            messages: vec![
                Message::new("Welcome to Thangorodrim Heights: Crown Jewels of Angband",
                    Priority::Max),
                Message::new("See the nonexistant manpage for gameplay details.",
                    Priority::Max),
            ],
            map_path: path.to_string(),
        })
    }

    pub fn save_to_file(&mut self) -> Result<(), Box<dyn Error>> {
        fs::write(&self.map_path, ron::to_string(&self.dungeon)?.as_bytes())?;
        self.messages.push(
            Message::new(
                &format!("Saved game to {}", &self.map_path), Priority::Normal));
        Ok(())
    }

    pub fn handle_action(&mut self, ac: KeybindingAction) {
        match ac {
            KeybindingAction::LevelUp => (),
            KeybindingAction::LevelDown => (),
            KeybindingAction::Move(d) => {
                let level = self.dungeon.player.level;
                let cur_pos = Coord::from(self.dungeon.player.coords);
                let new_pos = cur_pos.neighbor_in_direction(d)
                    .clamp_x(self.dungeon.levels[level].width)
                    .clamp_y(self.dungeon.levels[level].height)
                    .as_yx();
                if self.dungeon.at(level, new_pos).tiletype == TileType::Wall {
                    // impassable for heavens sake
                    // do nothing
                    // in the future, though, we'll check if the player
                    // has a pickaxe in his inventory and if so, demolish
                    // the wall
                } else {
                    self.dungeon.player.coords = new_pos;
                    self.dungeon.move_mob(level, cur_pos.as_yx(),
                        level, new_pos, true).unwrap();
                }
            },
            KeybindingAction::Wait => (),
            _ => (),
        }
    }
}
