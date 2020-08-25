use pancurses::*;
use std::{fs, fs::File};
use ron::de::from_reader;
use lib::dungeon::*;
use walkdir::WalkDir;
use lib::material::*;
use lib::id::*;
use lib::colors::*;
use lib::dun_s1::*;
use std::error::Error;
use std::collections::HashMap;

fn main() {
    let window = initscr();
    start_color();
    use_default_colors();
    cbreak();
    noecho();
    window.refresh();

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


    let mapwin = newwin(
        window.get_max_y(),
        window.get_max_x() - 20,
        0, 0
    );
    mapwin.refresh();
    draw_map(mapwin, map, &materials, 20, 20, 0);

    window.getch();
    endwin();
}

pub fn draw_map(
    win: Window, map: Dungeon,
    materials: &HashMap<String, MaterialInfo>,
    cur_x: usize, cur_y: usize, lvl: usize
) {
    let level  = &map.levels[lvl];
    let starty = cur_y.saturating_sub((win.get_max_y() as usize) / 2);
    let endy   = cur_y + ((win.get_max_y() as usize) / 2);
    let startx = cur_x.saturating_sub((win.get_max_x() as usize) / 2);
    let endx   = cur_x + ((win.get_max_x() as usize) / 2);

    for y in starty..endy {
        for x in startx..endx {
            let tile_material = &materials[&level.d[y][x].tile_material];
            let mut bg = tile_material.color_bg;
            let mut fg = tile_material.color_fg;
            let mut glyph: char;

            match level.d[y][x].tiletype {
                TileType::Debug
                | TileType::Wall  => {
                    glyph = tile_material.block_glyph;
                },
                TileType::Floor => {
                    glyph = '+';
                    bg = Color::new(0, 0, 0, 0);
                },
            }

            if level.d[y][x].mobs.len() > 0 {
                let mob = &level.d[y][x].mobs[0];
                bg = Color::new(0, 0, 0, 0);
                glyph = mob.unicode_glyph;
                if let Some(mob_fg) = mob.glyph_fg {
                    fg = mob_fg;
                } else {
                    fg = Color::new(0, 0, 0, 0);
                }
            }

            let fg_256 = (((fg.red as f64)*6.0/256.0)*36.0
                + ((fg.green as f64)*6.0/256.0)*6.0 + ((fg.blue as f64)*6.0/256.0)) as i16;
            let bg_256 = (((bg.red as f64)*6.0/256.0)*36.0
                + ((bg.green as f64)*6.0/256.0)*6.0 + ((bg.blue as f64)*6.0/256.0)) as i16;
            init_pair(x as i16, fg_256, bg_256);
            //init_pair(2, 156, -1);

            //win.printw(format!("{}[38;2;{};{};{}m{}[48;2;{};{};{}m{}{}[m",
                //0x1b as char, fg.red, fg.green, fg.blue,
                //0x1b as char, bg.red, bg.green, bg.blue,
                //glyph, 0x1b as char));
            win.attrset(ColorPair(x as u8));
            win.printw(format!("{}", glyph));
        }
        win.printw("\n");
    }

    win.refresh();
}
