use crate::message::*;
use crate::priority::*;
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
}
