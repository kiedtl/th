mod cellular;
mod drunk;
mod dunspec;
mod maze;
mod mineral_placement;
mod mob_placement;
mod randrm;
mod utils;

use crate::cellular::*;
use crate::drunk::*;
use crate::dunspec::*;
use crate::maze::*;
use lib::dun_s1::*;
use lib::dun_s2::*;
use lib::dungeon::*;
use lib::id::*;
use lib::material::*;
use lib::mob::*;
use crate::mineral_placement::*;
use crate::mob_placement::*;
use crate::randrm::*;

use std::collections::HashMap;
use std::error::Error;
use std::{fs, fs::File};
use ron::de::from_reader;
use walkdir::WalkDir;

const PLAYER_MOB: &str = "elf";

fn main() {
    let mut rng = rand::thread_rng();
    let mut materials:   HashMap<String, MaterialInfo> = HashMap::new();
    let mut mobs:        HashMap<String, MobTemplate>  = HashMap::new();
    let mut dungeons_s1: Vec<DungeonS1> = Vec::new();
    let mut dungeons_s2: Vec<DungeonS2> = Vec::new();

    // check arguments
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("{}: need DungeonSpecification file.", args[0]);
        eprintln!("usage: {} <file.ron>", args[0]);
        std::process::exit(1);
    }

    // load and parse material info files
    // TODO: do not hardcode paths
    fn load_info_files<T>(arg0: &str, path: &str, accm: &mut HashMap<String, T>) -> Result<(), Box<dyn Error>>
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
                .generate(materials.values().cloned().collect());
            MobPlacer::new(&mut new_map, layer.inhabitants.clone(), &mut rng)
                .generate(&mut mobs.values().cloned().collect::<Vec<MobTemplate>>());
            dungeons_s1.push(map);
            dungeons_s2.push(new_map);
        }
    }

    // ensure that the info file isn't missing
    assert!(mobs.contains_key(PLAYER_MOB));
    let dungeon = Dungeon::from_dungeon_s2(config.world_name, &mut dungeons_s2,
        &mut rng, &mobs[PLAYER_MOB]);

    fs::write("map.ron", ron::to_string(&dungeon).unwrap().as_bytes()).unwrap();
}
