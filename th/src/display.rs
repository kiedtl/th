use crate::state::*;
use crate::tb::*;
use lib::colors::*;
use lib::dun_s1::*;
use lib::dun_s2::*;
use lib::material::*;
use lib::math::*;
use std::collections::HashMap;
use termbox_sys::*;

enum DisplayWindow {
    Map,
    Message,
    Detail,
}

impl DisplayWindow {
    // get dimensions of window
    // (startx, starty, endx, endy)
    fn dimensions(&self) -> (i32, i32, i32, i32) {
        match self {
            DisplayWindow::Map => {
                (0, 0, unsafe { tb_width() } - 30, unsafe { tb_height() } - 10)
            },
            DisplayWindow::Message => {
                let (_, _, map_x, map_y) = DisplayWindow::Map.dimensions();
                (0, map_y + 1, map_x, unsafe { tb_height() })
            },
            DisplayWindow::Detail => {
                let (_, _, map_x, _) = DisplayWindow::Map.dimensions();
                (map_x + 1, 0, unsafe { tb_width() },
                    unsafe { tb_height() })
            },
        }
    }
}

pub enum DisplayMode {
    SDL,
    Console,
}

pub struct Display<'a> {
    mode: DisplayMode,
    materials: &'a HashMap<String, MaterialInfo>,
    st: &'a State,
}

impl Display<'_> {
    pub fn new<'a>(
        st: &'a State, mode: DisplayMode,
        mats: &'a HashMap<String, MaterialInfo>,
    ) -> Display<'a> {
        match unsafe { tb_init() } {
            TB_EFAILED_TO_OPEN_TTY => {
                eprintln!("error: could not open terminal");
                std::process::exit(1);
            },
            TB_EUNSUPPORTED_TERMINAL => {
                eprintln!("error: unsupported terminal");
                eprintln!("hint: try using another terminal (such as xterm)");
                std::process::exit(1);
            },
            TB_EPIPE_TRAP_ERROR => {
                eprintln!("error: could not initialize screen");
                std::process::exit(1);
            },
            _ => (),
        }

        unsafe {
            tb_select_output_mode(TB_OUTPUT_TRUECOLOR);
            tb_set_clear_attributes(TB_WHITE, TB_BLACK);
            tb_clear();
        }

        Display { mode: mode, materials: mats, st: st }
    }

    pub fn present(&self) {
        unsafe {
            tb_present();
        }
    }

    pub fn draw(&self) {
        match &self.mode {
            DisplayMode::SDL => unimplemented!(),
            DisplayMode::Console => self.draw_console(),
        }
    }

    pub fn draw_console(&self) {
        self.draw_console_map();
        self.draw_console_messages();
    }

    pub fn draw_console_messages(&self) {
        let (mut xctr, mut yctr, max_x, max_y) =
            DisplayWindow::Message.dimensions();
        let mut msgctr = 0;

        while yctr < max_y && msgctr < self.st.messages.len() {
            let message = &self.st.messages[msgctr];
            let p = message.priority.as_usize();

            let tocol = |p| return clamp(p * 40, 0, 255);
            let fg = Color::new(tocol(p), tocol(p), tocol(p), 0)
                .as_u32();
            let bg = Color::new(0, 0, 0, 0).as_u32();

            let res = tb_put_string(max_x, max_y, xctr, yctr,
                &self.st.messages[msgctr].text, fg, bg, true);
            yctr = res.0; msgctr += 1;
        }
    }

    pub fn draw_console_map(&self) {
        let level = &self.st.dungeon.levels[self.st.dungeon.player.level];
        let cur_y = self.st.dungeon.player.coords.0 as i32;
        let cur_x = self.st.dungeon.player.coords.1 as i32;

        // xctr/yctr is the current position on the screen
        // max_x/max_y is the maximum size of a window
        let (mut xctr, mut yctr, max_x, max_y) =
            DisplayWindow::Map.dimensions();

        let starty = cur_y - (max_y / 2);
        let endy   = cur_y + (max_y / 2);
        let startx = cur_x - (max_x / 2);
        let endx   = cur_x + (max_x / 2);

        for y in starty..endy {
            for x in startx..endx {
                // if out of bounds of the map, just draw a black tile
                if (y < 0 || x < 0) ||
                    (y as usize >= level.height || x as usize >= level.width) {
                        unsafe {
                            tb_change_cell(xctr, yctr, ' ' as u32, 0xffffff, 0x000000);
                        }


                        if xctr >= max_x { break; }
                        else { xctr += 1; }
                        continue;
                }

                let mut cell = self.tile_as_cell(&level.d[y as usize][x as usize]);
                if x == cur_x && y == cur_y {
                    cell.bg = Color::new(255, 255, 255, 0)
                        .as_u32();
                    cell.fg = Color::new(0, 0, 0, 0).as_u32();
                    cell.ch = '@' as u32;
                }
                unsafe { tb_put_cell(xctr, yctr, &cell); }

                if xctr >= max_x { break; }
                else { xctr += 1; }
            }

            if yctr >= max_y { break; }
            else { yctr += 1; }

            xctr = 0;
        }
    }

    // helper func to get a single tile as a RawCell
    fn tile_as_cell(&self, tile: &DungeonTile) -> RawCell {
        let tile_material = &self.materials[&tile.tile_material];
        let mut bg = tile_material.color_bg;
        let mut fg = tile_material.color_fg;
        let mut glyph: char;

        match tile.tiletype {
            TileType::Debug
            | TileType::Wall  => {
                glyph = tile_material.block_glyph;
            },
            TileType::Floor => {
                glyph = ' ';
                bg = Color {
                    red: bg.red / 32,
                    green: bg.green / 32,
                    blue: bg.blue / 32,
                    alpha: bg.alpha,
                };
            },
        }

        if let Some(mob) = &tile.mobs {
            bg = Color::new(0, 0, 0, 0);
            glyph = mob.unicode_glyph;
            if let Some(mob_fg) = mob.glyph_fg {
                fg = mob_fg;
            } else {
                fg = Color::new(0, 0, 0, 0);
            }
        }

        RawCell {
            ch: glyph as u32,
            fg: fg.as_u32(),
            bg: bg.as_u32(),
        }
    }

    pub fn close(&self) {
        unsafe {
            tb_shutdown();
        }
    }
}
