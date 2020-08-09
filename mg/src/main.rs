mod dirs;
mod drunk;
mod cellular;
mod features;
mod randrm;
mod rect;

use crate::cellular::*;
use rand::prelude::*;

type Dungeon = [[bool; 205]; 50];

fn main() {
    // 50 rows; 205 columns
    let mut map = [[false; 205]; 50];
    let mut rng = rand::thread_rng();

    let mut cgen = CellularAutomata::new(&mut map, &mut rng);
    cgen.open_space_percentage(64);
    cgen.wall_requirement(6);
    cgen.island_requirement(2);

    cgen.random_fill();
    cgen.add_floor_bar(1);
    cgen.generation(true);
    cgen.generation(true);
    cgen.generation(true);
    cgen.generation(false);
    cgen.generation(false);
    cgen.generation(false);
    cgen.generation(false);

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
