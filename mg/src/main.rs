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

use serde::Deserialize;
use std::fs::File;
use rand::prelude::*;
use ron::de::from_reader;

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

    let mut map = DungeonS1::new(205, 205);
    map.set(1, 1, TileType::Floor);
    maze(&mut map, &mut rng, 1, 1);
    display(&map);
    dungeons_s1.push(map);
    return;

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
                TileType::Floor => print!(" "), //"·"),
                TileType::Wall  => print!("▒"),
                //TileType::Wall  => print!("#"),
                //TileType::Floor => print!("."),
            }
        }
        print!("\n");
    }
}

fn maze<R: Rng>(map: &mut DungeonS1, rng: &mut R, cy: isize, cx: isize) {
    let mut neighbors = &mut [((-1, 0), (-2, 0)), ((0,  -1), (0,  -2)),
                          ((0,  1), (0,  2)), ((1,   0), (2,  0))];
    neighbors.shuffle(rng);
    for neighbor in neighbors {
        let (iy, ix) = (cy + (neighbor.0).0, cx + (neighbor.0).1);
        let (ny, nx) = (cy + (neighbor.1).0, cx + (neighbor.1).1);
        if nx > 0 && ny > 0 &&
            nx < ((map.width - 1) as isize) && ny < ((map.height - 1) as isize) &&
                map.d[ny as usize][nx as usize] == TileType::Wall {

                map.set(nx as usize, ny as usize, TileType::Floor);
                map.set(ix as usize, iy as usize, TileType::Floor);
                maze(map, rng, ny, nx)
        }
    }
}
