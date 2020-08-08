mod drunk;
mod randrm;
mod rect;

use std::vec::Vec;
use std::cmp::{min, max};
use rand::prelude::*;

fn main() {
    // 50 rows; 205 columns
    let mut map = [[0.0; 205]; 50];

    drunk::walk(&mut map);
    randrm::tunnel(&mut map);
    display(map);
}

fn display(map: [[f64; 205]; 50]) {
    for y in 0..50 {
        for x in 0..205 {
            if map[y][x] > 3.0 {//|| map[y][x] <= 1.0 {
                print!(" ");
            } else {
                print!("#");
            }
            //print!("{}", map[y][x] as u8);
        }
        print!("\n");
    }
}
