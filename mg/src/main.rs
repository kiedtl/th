mod dirs;
mod drunk;
mod dun_s1;
mod cellular;
mod randrm;
mod rect;

use crate::drunk::*;
use crate::dun_s1::*;
use crate::cellular::*;
use crate::randrm::*;

use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
enum MapgenAlgorithm {
    Drunkard(DrunkardOptions),
    Cellular(CellularAutomataOptions),
    RandomRooms(RandomRoomsOptions),
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
    let mut dungeons_s1: Vec<DungeonS1> = Vec::new();

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("{}: need DungeonSpecification file.", args[0]);
        eprintln!("usage: {} <file.ron>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let fconf;
    match File::open(input_path) {
        Ok(f) => fconf = f,
        Err(e) => {
            println!("{}: \"{}\": {}", args[0], input_path, e);
            std::process::exit(1);
        },
    }

    let config: DungeonSpecification = match from_reader(fconf) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        }
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
                }
            }

            display(&map);
            dungeons_s1.push(map);
        }
    }
}

fn display(map: &DungeonS1) {
    for y in 0..map.height {
        for x in 0..map.width {
            match map.d[y][x] {
                //TileType::Floor => print!("·"),
                //TileType::Wall  => print!("▓"),
                TileType::Wall  => print!("#"),
                TileType::Floor => print!("."),
            }
        }
        print!("\n");
    }
}
