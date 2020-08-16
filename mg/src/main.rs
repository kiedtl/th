mod dirs;
mod drunk;
mod dun_s1;
mod maze;
mod features;
mod cellular;
mod randrm;
mod rect;

use crate::drunk::*;
use crate::dun_s1::*;
use crate::maze::*;
use crate::cellular::*;
use crate::randrm::*;

use serde::Deserialize;
use std::fs::File;
use ron::de::from_reader;
use noise::{Abs, Perlin, OpenSimplex, NoiseFn, Seedable, BasicMulti, Billow, Fbm, HybridMulti, RidgedMulti, Checkerboard, Constant, Cylinders, SuperSimplex, Value, Worley};
use rand::prelude::*;

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
    let mut dungeons_s1: Vec<DungeonS1> = Vec::new();

    // OpenSimplex/BasicMulti/Fbm/HybridMulti
    // OpenSimplex
    // Cylinders for morgoth's lair
    // Value -- chunks/islands of high-value minerals
    let os = Perlin::new().set_seed(rng.gen());
    let mut map = DungeonS1::new(638, 153); //250, 64);
    for y in 0..(map.height - 1) {
        for x in 0..(map.width - 1) {
            //map.d[y][x] = 
            let nx: f64 = (x as f64) / (map.width as f64)  - 0.5;
            let ny: f64 = (y as f64) / (map.height as f64) - 0.5;

            let mut noise =   1.0 * os.get([ 1.0 * ny,  1.0 * nx])
                          +   0.5 * os.get([ 2.0 * ny,  2.0 * nx])
                          +  0.25 * os.get([ 4.0 * ny,  4.0 * nx])
                          + 0.125 * os.get([ 8.0 * ny,  8.0 * nx])
                          + 0.062 * os.get([16.0 * ny, 16.0 * nx])
                          + 0.031 * os.get([32.0 * ny, 32.0 * nx]);
            let color: u8 = (noise.abs().powf(4.12) * 255.0) as u8;

            //println!("{},{}", x, color);
            print!("{}[48;2;{};{};{}m {}[m", 0x1b as char, color, color, color, 0x1b as char);
        }

        println!("");
    }

    return; // DEBUG

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
                    MapgenAlgorithm::Maze(m) => {
                        Maze::new(&mut map, &mut rng, *m)
                            .create();
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
                TileType::Wall   => print!("▒"), //"·"),
                TileType::Debug  => print!("░"),
                TileType::Floor  => print!(" "),
                //TileType::Wall  => print!("#"),
                //TileType::Floor => print!("."),
            }
        }
        print!("\n");
    }
}
