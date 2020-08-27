use termbox_sys::*;
use lib::dungeon::Dungeon;

// this contains the entire state of the game
pub struct State {
    // dungeon/player/level
    pub dungeon: Dungeon,
    pub player: (i32, i32),
    pub level: usize,

    // other stuff
    pub messages: Vec<Message>,
}

impl State {
    pub fn new(dungeon: Dungeon, player: (i32, i32)) -> State {
        State {
            dungeon: dungeon,
            player: player,
            level: 0,
            messages: Vec::new(),
        }
    }
}
