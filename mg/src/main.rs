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
    let mut map = DungeonS1::new(205, 66);

    CellularAutomata::new(&mut map, &mut rng)
        .open_space_percentage(64)
        .wall_requirement(6)
        .island_requirement(2)
        .schedule_job(JobType::RandomFill)
        .schedule_job(JobType::FloorBar(1))
        .schedule_job(JobType::Generation(true))
        .schedule_job(JobType::Generation(true))
        .schedule_job(JobType::Generation(true))
        .schedule_job(JobType::Generation(false))
        .schedule_job(JobType::Generation(false))
        .schedule_job(JobType::Generation(false))
        .schedule_job(JobType::Generation(false))
        .do_work();

    Drunkard::new(&mut map, &mut rng)
        .center_weight(0.1)
        .previous_direction_weight(0.65)
        .max_iterations(5000)
        .filled_goal(0.25);
        //.walk();

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
