mod dirs;
mod drunk;
mod dun_s1;
mod cellular;
mod features;
mod thiparse;
mod randrm;
mod rect;

use crate::drunk::*;
use crate::dun_s1::*;
use crate::cellular::*;
use crate::thiparse::*;
use crate::randrm::*;
use rand::prelude::*;
use std::fs;

fn main() {
    let mut rng = rand::thread_rng();
    let mut dungeons_s1: Vec<DungeonS1> = Vec::new();

    let d: Vec<String> = fs::read_to_string("sample.thi").unwrap()
        .split('\n').map(|v| v.to_string()).collect();
    let i = InfoFileData::from_lines(d.clone());

    // get number of layers
    let layer_len = i.d["DUNGEON_INFO"]
        .section()["LAYERS"].values()[0]
        .parse::<usize>().unwrap(); // TODO: chk

    for l in 0..layer_len {
        let layer_info = i.d[&format!("LAYER_{}", l)].section();
        let levels = layer_info["CONTAINED_LEVELS"].values()[0]
            .parse::<usize>().unwrap();
        let width  = layer_info["DIMENSIONS"].values()[0]
            .parse::<usize>().unwrap();
        let height = layer_info["DIMENSIONS"].values()[1]
            .parse::<usize>().unwrap();
        let algorithms = layer_info["ALGORITHM"].values();

        for _ in 0..levels {
            let mut map = DungeonS1::new(width, height);
            // TODO: use builders
            // make all algorithm implementation
            // have a common trait or something
            // perhaps then we can rewrite this mess
            for algorithm in &algorithms {
                match algorithm.as_str() {
                    "drunk" => {
                        let mut d = Drunkard::new(&mut map, &mut rng);

                        if layer_info.contains_key("DRUNK_CENTER_WEIGHT") {
                            d.center_weight = layer_info["DRUNK_CENTER_WEIGHT"]
                                .values()[0].parse::<f64>().unwrap();
                        }

                        if layer_info.contains_key("DRUNK_PREVIOUS_DIRECTION_WEIGHT") {
                            d.previous_direction_weight = layer_info["DRUNK_PREVIOUS_DIRECTION_WEIGHT"]
                                .values()[0].parse::<f64>().unwrap();
                        }

                        if layer_info.contains_key("DRUNK_MAX_ITERATIONS") {
                            d.max_iterations = layer_info["DRUNK_MAX_ITERATIONS"]
                                .values()[0].parse::<usize>().unwrap();
                        }

                        if layer_info.contains_key("DRUNK_FILLED_GOAL") {
                            d.filled_goal = layer_info["DRUNK_FILLED_GOAL"]
                                .values()[0].parse::<f64>().unwrap();
                        }

                        d.walk();
                    },
                    _ => (),
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
