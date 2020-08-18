mod dirs;
mod drunk;
mod dun_s1;
mod dun_s2;
mod maze;
mod colors;
mod material;
mod features;
mod cellular;
mod randrm;
mod rect;
mod items;

use crate::drunk::*;
use crate::dun_s1::*;
use crate::dun_s2::*;
use crate::maze::*;
use crate::cellular::*;
use crate::material::*;
use crate::randrm::*;

use rand::prelude::*;
use serde::Deserialize;
use noise::{BasicMulti, Seedable};
use std::{fs, fs::File};
use ron::de::from_reader;
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
enum MapgenAlgorithm {
    Drunkard(DrunkardOptions),
    Cellular(CellularAutomataOptions),
    RandomRooms(RandomRoomsOptions),
    Maze(MazeOptions),
}

#[derive(Debug, Deserialize)]
struct LayerSpecification {
    levels: usize,
    dimensions: (usize, usize),   // (width, height)
    algorithms: Vec<MapgenAlgorithm>,
}

#[derive(Debug, Deserialize)]
struct DungeonSpecification {
    layers: Vec<LayerSpecification>,
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut materials: Vec<MaterialInfo> = Vec::new();
    let mut dungeons_s1: Vec<DungeonS1> = Vec::new();

    // check arguments
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("{}: need DungeonSpecification file.", args[0]);
        eprintln!("usage: {} <file.ron>", args[0]);
        std::process::exit(1);
    }

    // load and parse material info files
    // TODO: do not hardcode paths
    for entry in WalkDir::new("../dat/mats/")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok()) {
            if fs::metadata(entry.path()).unwrap().is_dir() {
                continue;
            }

            let info_file = File::open(entry.path())
                .unwrap();
            match from_reader(info_file) {
                Ok(x) => materials.push(x),
                Err(e) => {
                    eprintln!("{}: failed to load info file: {}: {}",
                        args[0], entry.path().display(), e);
                },
            }
    }

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

            dungeons_s1.push(map);
        }
    }

    let mut dungeons_s2: Vec<DungeonS2> = Vec::new();
    dungeons_s1.iter().for_each(|d| {
        let mut new_d = DungeonS2::from_dungeon_s1(d);
        new_d.decide_materials(materials.clone(),
            BasicMulti::new().set_seed(rng.gen()));
        dungeons_s2.push(new_d);
    });

    for d in dungeons_s2 {
        display(&d);
    }
}

fn display(map: &DungeonS2) {
    for y in 0..(map.height) {
        for x in 0..(map.width) {
            let color = map.d[y][x].tile_material.color;
            let character: char;

            match map.d[y][x].tiletype {
                TileType::Wall  => character = '▒',
                TileType::Debug => character = '░',
                TileType::Floor => character = '+',
            }

            print!("{}[38;2;{};{};{}m{}", 0x1b as char, color.red,
                color.green, color.blue, character);
        }
        print!("\n{}[m", 0x1b as char);
    }
}
