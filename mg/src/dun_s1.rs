use std::vec::Vec;
use rand::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    // TODO: ditch
    Wall, Floor,
}

// a DungeonS1 ("Dungeon Stage 1") is a dungeon
// with only <wall>/<open_space> designated --
// just the layout. so no mobs, items, et cetera.
#[derive(Clone, Debug)]
pub struct DungeonS1 {
    pub d: Vec<Vec<TileType>>,
    pub width: usize, pub height: usize,
}

impl DungeonS1 {
    pub fn new(width: usize, height: usize) -> DungeonS1 {
        let mut dungeon: Vec<Vec<TileType>> = Vec::new();
        for _ in 0..height {
            dungeon.push(vec![TileType::Wall; width]);
        }

        DungeonS1 {
            d: dungeon,
            width: width, height: height,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, t: TileType) {
        self.d[y][x] = t;
    }

    pub fn rand_fill<R: Rng>(&mut self, rng: &mut R, floor_percentage: usize) {
        for y in 1..self.height {
            for x in 1..self.width {
                if rng.gen_range(0, 100) < floor_percentage {
                    self.set(x, y, TileType::Floor);
                } else {
                    self.set(x, y, TileType::Wall);
                }
            }
        }
    }
}
