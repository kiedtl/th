mod dirs;
mod drunk;
mod features;
mod randrm;
mod rect;

use std::vec::Vec;
use pathfinding::prelude::{absdiff, astar};
use rand::prelude::*;

type Dungeon = [[bool; 205]; 50];

fn main() {
    // 50 rows; 205 columns
    let mut map = [[false; 205]; 50];
    let mut rng = rand::thread_rng();

    //drunk::walk(&mut map);
    cellular_automata(&mut map, &mut rng);
    randrm::tunnel(&mut map);
    display(map);
}

fn display(map: Dungeon) {
    for y in 0..50 {
        for x in 0..205 {
            if map[y][x] {
                print!("·");
            } else {
                print!("▓");
            }
            //print!("{}", map[y][x] as u8);
        }
        print!("\n");
    }
}

fn cellular_automata<R: Rng>(map: &mut Dungeon, rng: &mut R) {
    // fill map randomly
    for y in 0..50 {
        for x in 0..205 {
            map[y][x] = rng.gen_range(0, 100) < 64;
        }
    }

    // add a horizontal bar of walls in the center of the map
    // as it may prevent a continuous vertical wall from forming,
    // thus preventing isolated sections
    for y in ((50 / 2) as usize)..(((50 / 2) + 1) as usize) {
        for x in 0_usize..204_usize {
            map[y][x] = true;
        }
    }

    let mut generation = |islands: bool| {
        let oldmap: Dungeon = map.clone();
        for y in 0_usize..49_usize {
            for x in 0_usize..204_usize {
                // all eight surrounding tiles
                let neighbors: &[(usize, usize); 9] =
                    &[(y.saturating_sub(1), x.saturating_sub(1)),
                        (y.saturating_sub(1), x), (y.saturating_sub(1), x.saturating_add(1)),
                    (y, x.saturating_sub(1)), (y, x), (y, x.saturating_add(1)),
                    (y.saturating_add(1), x.saturating_sub(1)),
                        (y.saturating_add(1), x), (y.saturating_add(1), x.saturating_add(1))];
                let mut neighboring_walls = 0;

                for neighbor in neighbors {
                    //println!("with fix: {:?}; without: {:?}", fixidx(*neighbor), *neighbor);
                    let (ny, nx) = *neighbor;
                    if !oldmap[ny][nx] {
                        neighboring_walls += 1;
                    }
                }

                if neighboring_walls >= 6 {
                    map[y][x] = false;
                } else {
                    if islands && neighboring_walls <= 2 {
                        map[y][x] = false;
                    } else {
                        map[y][x] = true;
                    }
                }
            }
        }
    };


    generation(true);
    generation(true);
    generation(true);
    generation(true);

    generation(false);

    generation(true);

    generation(false);
    generation(false);
    generation(false);
}
