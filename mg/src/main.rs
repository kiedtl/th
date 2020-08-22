mod cellular;
mod colors;
mod dirs;
mod drunk;
mod dun_s1;
mod dun_s2;
mod dunspec;
mod features;
mod items;
mod material;
mod maze;
mod mineral_placement;
mod mob_placement;
mod mob;
mod randrm;
mod rect;
mod utils;
mod value;

use crate::cellular::*;
use crate::colors::*;
use crate::drunk::*;
use crate::dun_s1::*;
use crate::dun_s2::*;
use crate::dunspec::*;
use crate::material::*;
use crate::maze::*;
use crate::mob::*;
use crate::mineral_placement::*;
use crate::mob_placement::*;
use crate::randrm::*;

use std::{fs, fs::File};
use std::error::Error;
use ron::de::from_reader;
use walkdir::WalkDir;

fn main() {
    let mut rng = rand::thread_rng();
    let mut materials:   Vec<MaterialInfo> = Vec::new();
    let mut mobs:        Vec<MobTemplate>  = Vec::new();
    let mut dungeons_s1: Vec<DungeonS1>    = Vec::new();
    let mut dungeons_s2: Vec<DungeonS2>    = Vec::new();

    // check arguments
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("{}: need DungeonSpecification file.", args[0]);
        eprintln!("usage: {} <file.ron>", args[0]);
        std::process::exit(1);
    }

    // load and parse material info files
    // TODO: do not hardcode paths
    fn load_info_files<T>(arg0: &str, path: &str, accm: &mut Vec<T>) -> Result<(), Box<dyn Error>>
    where
        T: for<'a> serde::Deserialize<'a>
    {
        for entry_ in WalkDir::new(path) {
            let entry = entry_.unwrap();
            if fs::metadata(entry.path()).unwrap().is_dir() {
                continue;
            }
            let info_file = File::open(entry.path()).unwrap();
            match from_reader(info_file) {
                Ok(x)  => accm.push(x),
                Err(e) => {
                    eprintln!("{}: failed to load info file: {}: {}",
                        arg0, entry.path().display(), e);
                },
            }
        }
        Ok(())
    }
    load_info_files(&args[0], "../dat/mats/", &mut materials).unwrap();
    load_info_files(&args[0], "../dat/mobs/", &mut mobs).unwrap();

    // try to load configuration
    let input_path = &args[1];
    let fconf = match File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            println!("{}: \"{}\": {}", args[0], input_path, e);
            std::process::exit(1);
        },
    };

    // parse configuration
    let config: DungeonSpecification = match from_reader(fconf) {
        Ok(x) => x,
        Err(e) => {
            println!("{}: failed to load config: {}", args[0], e);
            std::process::exit(1);
        },
    };

    for layer in &config.layers {
        for _level in 0..layer.levels {
            let mut map = DungeonS1::new(layer.dimensions.0,
                layer.dimensions.1);
            for algorithm in &layer.algorithms {
                match &algorithm {
                    MapgenAlgorithm::Drunkard(d) => {
                        Drunkard::new(&mut map, &mut rng, *d)
                            .walk();
                    },
                    MapgenAlgorithm::Cellular(c) => {
                        CellularAutomata::new(&mut map, &mut rng, c.clone())
                            .do_work();
                    },
                    MapgenAlgorithm::RandomRooms(r) => {
                        RandomRooms::new(&mut map, &mut rng, *r)
                            .tunnel();
                    },
                    MapgenAlgorithm::Maze(m) => {
                        Maze::new(&mut map, &mut rng, *m)
                            .create();
                    },
                }
            }

            // decide minerals and mobs
            let mut new_map = DungeonS2::from_dungeon_s1(&map);
            MineralPlacer::new(&mut new_map, layer.composition, &mut rng)
                .generate(materials.clone());
            MobPlacer::new(&mut new_map, layer.inhabitants.clone(), &mut rng)
                .generate(&mut (mobs.clone()));
            dungeons_s1.push(map);
            dungeons_s2.push(new_map);
        }
    }

    for d in dungeons_s2 {
        display(&d);
    }
}

fn display(map: &DungeonS2) {
    for y in 0..(map.height) {
        for x in 0..(map.width) {
            let mut bg = map.d[y][x].tile_material.color_bg;
            let mut fg = map.d[y][x].tile_material.color_fg;
            let mut glyph: char;

            match map.d[y][x].tiletype {
                TileType::Debug
                | TileType::Wall  => {
                    glyph = map.d[y][x].tile_material.block_glyph;
                },
                TileType::Floor => {
                    glyph = '+';
                    bg = Color::new(0, 0, 0, 0);
                },
            }

            if map.d[y][x].mobs.len() > 0 {
                let mob = &map.d[y][x].mobs[0];
                bg = Color::new(0, 0, 0, 0);
                glyph = mob.unicode_glyph;
                if let Some(mob_fg) = mob.glyph_fg {
                    fg = mob_fg;
                } else {
                    fg = Color::new(0, 0, 0, 0);
                }
            }

            print!("{}[38;2;{};{};{}m{}[48;2;{};{};{}m{}{}[m",
                0x1b as char, fg.red, fg.green, fg.blue,
                0x1b as char, bg.red, bg.green, bg.blue,
                glyph, 0x1b as char);
        }
        print!("\n{}[m", 0x1b as char);
    }
}
