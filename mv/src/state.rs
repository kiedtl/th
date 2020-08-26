use termbox_sys::*;
use lib::dungeon::Dungeon;

pub struct State {
    pub screen_width: i32,
    pub screen_height: i32,
    pub dungeon: Dungeon,
    pub current_x: usize,
    pub current_y: usize,
    pub level: usize,
}

impl State {
    pub fn new(dungeon: Dungeon) -> State {
        let (screen_width, screen_height);

        unsafe {
            screen_width = tb_width();
            screen_height = tb_height();
        }

        State {
            screen_width: screen_width,
            screen_height: screen_height,
            dungeon: dungeon,
            current_x: 0, current_y: 0,
            level: 0,
        }
    }
}
