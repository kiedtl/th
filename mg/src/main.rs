mod dirs;
mod drunk;
mod dun_s1;
mod cellular;
mod features;
mod randrm;
mod rect;

use crate::dun_s1::*;
use crate::drunk::*;
use crate::cellular::*;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let mut map = DungeonS1::new(205, 50);

    //let mut cgen = CellularAutomata::new(&mut map, &mut rng);
    //cgen.open_space_percentage(64);
    //cgen.wall_requirement(6);
    //cgen.island_requirement(2);

    //cgen.random_fill();
    //cgen.add_floor_bar(1);
    //cgen.generation(true);
    //cgen.generation(true);
    //cgen.generation(true);
    //cgen.generation(false);
    //cgen.generation(false);
    //cgen.generation(false);
    //cgen.generation(false);

    let mut d = Drunkard::new(&mut map, &mut rng)
        .center_weight(0.1)
        .previous_direction_weight(0.65)
        .max_iterations(5000)
        .filled_goal(0.25)
        .walk();

    display(&map);
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
