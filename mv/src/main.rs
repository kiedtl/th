use std::{fs, fs::File};
use ron::de::from_reader;
use lib::dungeon::*;
use walkdir::WalkDir;
use lib::material::*;
use lib::id::*;
use lib::colors::*;
use lib::dun_s1::*;
use lib::dun_s2::*;
use lib::math::*;
use std::error::Error;
use std::collections::HashMap;
use termbox_sys::*;

const NIL_RAW_EVENT: RawEvent = RawEvent { etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0, x: 0, y: 0 };

fn main() {
    // set a custom panic handler that calls tb_shutdown
    // before printing anything
    std::panic::set_hook(Box::new(|panic_info| {
        unsafe { tb_shutdown(); }
        println!("aborting due to fatal error (see below):");
        println!("{}", panic_info);
        println!("stack backtrace:");
        println!("{:?}", backtrace::Backtrace::new());
        println!("please report this issue upstream at {}.",
            "https://github.com/kiedtl/th");
    }));

    let screen_width;
    let screen_height;

    unsafe {
        tb_init();
        tb_select_output_mode(TB_OUTPUT_TRUECOLOR);
        tb_set_clear_attributes(TB_WHITE, TB_BLACK);
        tb_clear();
        screen_width = tb_width();
        screen_height = tb_height();
    }

    // check arguments
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("{}: need dungeon file.", args[0]);
        eprintln!("usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    // try to load configuration
    let input_path = &args[1];
    let fmap = match File::open(input_path) {
        Ok(d) => d,
        Err(e) => {
            println!("{}: \"{}\": {}", args[0], input_path, e);
            std::process::exit(1);
        },
    };

    // parse configuration
    let map: Dungeon = match from_reader(fmap) {
        Ok(x) => x,
        Err(e) => {
            println!("{}: failed to load map: {}", args[0], e);
            std::process::exit(1);
        },
    };

    fn load_info_files<T>(arg0: &str, path: &str, accm: &mut HashMap<String, T>) -> Result<(), Box<
dyn Error>>
    where
        T: for<'a> serde::Deserialize<'a> + Id + Clone
    {
        for entry_ in WalkDir::new(path) {
            let entry = entry_.unwrap();
            if fs::metadata(entry.path()).unwrap().is_dir() {
                continue;
            }
            let info_file = File::open(entry.path()).unwrap();
            let data: Result<T, ron::error::Error> = from_reader(info_file);
            match data {
                Ok(x)  => {
                    let idxr = x.id();
                    *accm.entry(idxr).or_insert(x) = x.clone();
                },
                Err(e) => {
                    eprintln!("{}: failed to load info file: {}: {}",
                        arg0, entry.path().display(), e);
                },
            }
        }
        Ok(())
    }

    let mut materials: HashMap<String, MaterialInfo> = HashMap::new();
    load_info_files(&args[0], "../dat/mats/", &mut materials).unwrap();

    let mut cursor_x = 0;
    let mut cursor_y = 0;
    let mut current_level = 0;

    draw_map(&map, screen_width / 2, screen_height - 5,
        &materials, cursor_x, cursor_y, current_level);
    draw_desc(&map, screen_width, screen_height,
        &materials, cursor_x, cursor_y, current_level);

    // write keybindings
    let kbds: Vec<(&str, &str)> = vec![
        ("<>",   "change level"),
        ("hjkl", "move cursor west/south/north/east"),
        ("HJKL", "like hjkl, but move 8 tiles at a time"),
        ("q",    "quit"),
    ];

    for kbd_i in 0..(kbds.len()) {
        let kbd = kbds[kbd_i];
        let row = screen_height - 5 + (kbd_i as i32);
        let mut col: i32 = 0;

        col = tb_put_string(screen_width / 2, screen_height, col, row,
            kbd.0, 0x000000, 0xffffff, false).1;
        if col < 5 { col += 5 - col; }
        tb_put_string(screen_width / 2, screen_height, col, row,
            kbd.1, 0xffffff, 0x000000, false).1;
    }

    unsafe { tb_present(); }

    loop {
        let mut ev = NIL_RAW_EVENT;
        let t = unsafe { tb_poll_event(&mut ev) };

        if t == -1 { // TODO: check -2, -3
            unsafe { tb_shutdown(); }
            panic!("fatal termbox error");
        }

        if t == (TB_EVENT_KEY as i32) {
            match std::char::from_u32(ev.ch).unwrap() {
                'q' => break,
                'h' => cursor_x = cursor_x.saturating_sub(1),
                'j' => cursor_y = clamp(cursor_y + 1, 0, map.levels[current_level].height - 1),
                'k' => cursor_y = cursor_y.saturating_sub(1),
                'l' => cursor_x = clamp(cursor_x + 1, 0, map.levels[current_level].width - 1),
                'H' => cursor_x = cursor_x.saturating_sub(8),
                'J' => cursor_y = clamp(cursor_y + 8, 0, map.levels[current_level].height - 1),
                'K' => cursor_y = cursor_y.saturating_sub(8),
                'L' => cursor_x = clamp(cursor_x + 8, 0, map.levels[current_level].width - 1),
                '>' => current_level = clamp(current_level + 1, 0, map.levels.len() - 1),
                '<' => current_level = current_level.saturating_sub(1),
                _ => (),
            }

            draw_map(&map, screen_width / 2, screen_height - 5,
                &materials, cursor_x, cursor_y, current_level);
            draw_desc(&map, screen_width, screen_height,
                &materials, cursor_x, cursor_y, current_level);
            unsafe { tb_present(); }
        }

        // TODO: handle TB_EVENT_RESIZE
        // TODO: handle TB_EVENT_MOUSE
    }

    unsafe { tb_shutdown(); }
}

pub fn draw_desc(
    map: &Dungeon, max_x: i32, max_y: i32,
    materials: &HashMap<String, MaterialInfo>,
    cur_x: usize, cur_y: usize, lvl: usize,
) {
    let startx = (max_x / 2 + 1) as i32;
    let mut row: i32 = 0i32;
    let mut col: i32 = startx;

    // draw tile
    let tile = &map.levels[lvl].d[cur_y][cur_x];
    let cell = tile_as_cell(tile, materials);
    unsafe { tb_put_cell(col, row, &cell); }
    col += 2;

    // draw coordinates
    let dim_str = format!("(row {}/{}, col {}/{}, lvl {}/{})",
        cur_y + 1, &map.levels[lvl].height, cur_x + 1, &map.levels[lvl].width,
        lvl + 1, &map.levels.len());
    tb_put_string(max_x, max_y, col, row, &dim_str, 0xffffff, 0x000000, false);
    row += 2;
    col = startx;

    // draw name of material
    let material = &materials[&tile.tile_material];
    tb_put_string(max_x, max_y, col, row, &material.name,
        0xffffff, 0x000000, false);
}

pub fn draw_map(
    map: &Dungeon, max_x: i32, max_y: i32,
    materials: &HashMap<String, MaterialInfo>,
    cur_x: usize, cur_y: usize, lvl: usize
) {
    let level  = &map.levels[lvl];

    let starty = (cur_y as isize) - ((max_y as isize) / 2);
    let endy   = (cur_y + ((max_y as usize) / 2)) as isize;
    let startx = (cur_x as isize) - ((max_x as isize) / 2);
    let endx   = (cur_x + ((max_x as usize) / 2)) as isize;

    // screen position
    let mut yctr = 0usize;
    let mut xctr = 0usize;

    for y in starty..endy {
        for x in startx..endx {
            // if out of bounds of the map, just draw a black tile
            if (y < 0 || x < 0) ||
                (y as usize >= level.height || x  as usize >= level.width) {
                    unsafe {
                        tb_change_cell(xctr as i32, yctr as i32,
                            ' ' as u32, 0xffffff, 0x000000);
                    }

                    if xctr >= max_x as usize { break; }
                    else { xctr += 1; }
                    continue;
            }

            let mut cell = tile_as_cell(&level.d[y as usize][x as usize],
                materials);
            if x as usize == cur_x && y as usize == cur_y {
                cell.bg = Color::new(200, 200, 0, 0).as_u32();
            }
            unsafe { tb_put_cell(xctr as i32, yctr as i32, &cell); }

            if xctr >= max_x as usize { break; }
            else { xctr += 1; }
        }

        if yctr >= max_y as usize { break; }
        else { yctr += 1; }

        xctr = 0;
    }
}

fn tile_as_cell(tile: &DungeonTile, materials: &HashMap<String, MaterialInfo>) -> RawCell {
    let tile_material = &materials[&tile.tile_material];
    let mut bg = tile_material.color_bg;
    let mut fg = tile_material.color_fg;
    let mut glyph: char;

    match tile.tiletype {
        TileType::Debug
        | TileType::Wall  => {
            glyph = tile_material.block_glyph;
        },
        TileType::Floor => {
            glyph = '+';
            bg = Color::new(0, 0, 0, 0);
        },
    }

    if tile.mobs.len() > 0 {
        let mob = &tile.mobs[0];
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

fn tb_put_string(
    max_x: i32, max_y: i32,
    col: i32, row: i32,
    str: &str,
    fg: u32, bg: u32,
    wrap: bool
) -> (i32, i32) {
    let mut ccol = col;
    let mut crow = row;
    for c in str.chars() {
        unsafe {
            tb_put_cell(ccol, crow, &RawCell {
                ch: c as u32,
                fg: fg, bg: bg
            });
        }

        if (ccol + 1) == (max_x - 1) {
            if wrap && crow + 1 != max_y {
                crow += 1;
                ccol = col;
            } else {
                let dot = RawCell {
                    ch: '.' as u32,
                    fg: fg, bg: bg,
                };

                // draw some nice ellipses
                unsafe {
                    tb_put_cell(ccol - 2, crow, &dot);
                    tb_put_cell(ccol - 1, crow, &dot);
                    tb_put_cell(ccol - 0, crow, &dot);
                }

                break;
            }
        } else {
            ccol += 1;
        }
    }

    // clear to the end of the line
    let clear_cell = RawCell {
        ch: ' ' as u32,
        fg: 0xffffff, bg: 0x000000,
    };

    for ncol in ccol..max_x {
        unsafe {
            tb_put_cell(ncol, row, &clear_cell);
        }
    }

    (crow, ccol)
}
