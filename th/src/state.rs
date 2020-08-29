use crate::coord::*;
use crate::kbd::*;
use crate::message::*;
use crate::priority::*;
use lib::dun_s1::*;
use lib::dungeon::Dungeon;

// this contains the entire state of the game
pub struct State {
    pub dungeon: Dungeon,

    // other stuff
    pub messages: Vec<Message>,
}

impl State {
    pub fn new(dungeon: Dungeon) -> State {
        State {
            dungeon: dungeon,
            messages: vec![
                Message::new("Welcome to Thangorodrim Heights: Crown Jewels of Angband",
                    Priority::Max),
                Message::new("See the nonexistant manpage for gameplay details.",
                    Priority::Max),
            ],
        }
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
                    self.messages.push(
                        Message::new("You bash your head against the wall.",
                            Priority::Normal));
                } else {
                    self.dungeon.player.coords = new_pos;
                    self.dungeon.move_mob(level, cur_pos.as_yx(),
                        level, new_pos, true).unwrap();
                }
            },
            KeybindingAction::Save => (),
            KeybindingAction::Quit => (),
        }
    }
}
