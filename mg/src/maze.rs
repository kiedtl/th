// create a perfect maze using a
// recursive backtracking algorithm.

use std::vec::Vec;
use serde::Deserialize;
use rand::prelude::*;

use crate::dun_s1::*;

#[derive(Clone, Debug, Deserialize)]
pub struct MazeOptions {
    // If set to None, will not remove dead ends.
    // If set to Some<i>, will remove <i> blocks
    // from each dead end.
    // Beware that in a perfect maze, every path
    // is a dead end.
    remove_dead_ends: Option<usize>,
}

impl MazeOptions {
    pub fn new() -> MazeOptions {
        MazeOptions {
            remove_dead_ends: Some(1),
        }
    }

    pub fn remove_dead_ends(mut self, val: Option<usize>) -> MazeOptions {
        self.remove_dead_ends = val;
        self
    }
}

pub struct Maze<'a, R: Rng> {
    map: &'a mut DungeonS1,
    options: MazeOptions,
    rng: &'a mut R,
}

impl<R: Rng> Maze<'_, R> {
    pub fn new<'a>(map: &'a mut DungeonS1,
        rng: &'a mut R, opt: MazeOptions) -> Maze<'a, R> {
            Maze {
                map: map,
                options: opt,
                rng: rng,
            }
    }

    pub fn create(&mut self) {
        let start = (1, 1);
        self.map.set(start.0, start.1, TileType::Floor);
        self.walk(start.0 as isize, start.1 as isize);
        self.remove_dead_ends();
    }

    fn remove_dead_ends(&mut self) {
        let goal;

        match self.options.remove_dead_ends {
            Some(g) => goal = g,
            None => return,
        }

        for _ in 0..goal {
            for y in 0isize..((self.map.height - 1) as isize) {
                for x in 0isize..((self.map.width - 1) as isize) {
                    let neighbors = &[(-1, 0), (0,  -1), (0,  1), (1,  0)];

                    let mut neighbor_walls = 0;
                    for neighbor in neighbors {
                        let (ny, nx) = (y + neighbor.0, x + neighbor.1);
                        if nx <= 0 || ny <= 0 {
                            continue;
                        }

                        if nx >= ((self.map.width - 1) as isize)
                            || ny >= ((self.map.height - 1) as isize) {
                                continue;
                        }

                        if self.map.d[ny as usize][nx as usize] == TileType::Wall {
                            neighbor_walls += 1;
                        }
                    }

                    if neighbor_walls >= 3 {
                        // found a dead end, fill it in
                        self.map.set(x as usize, y as usize, TileType::Wall);
                    }
                }
            }
        }
    }

    fn walk(&mut self, cx: isize, cy: isize) {
        // squares on the North, East, West, South
        let mut neighbors = &mut [((-1, 0), (-2, 0)), ((0,  -1), (0,  -2)),
                                  ((0,  1), (0,  2)), ((1,   0), (2,  0))];

        neighbors.shuffle(self.rng);

        // iterate through shuffled neighbors and create paths in
        // each direction if there's place for it
        for neighbor in neighbors {
            let (iy, ix) = (cy + (neighbor.0).0, cx + (neighbor.0).1);
            let (ny, nx) = (cy + (neighbor.1).0, cx + (neighbor.1).1);

            if nx <= 0 || ny <= 0 {
                continue;
            }

            if nx >= ((self.map.width - 1) as isize)
                || ny >= ((self.map.height - 1) as isize) {
                    continue;
            }

            if self.map.d[ny as usize][nx as usize] != TileType::Wall {
                continue;
            }

            self.map.set(nx as usize, ny as usize, TileType::Floor);
            self.map.set(ix as usize, iy as usize, TileType::Floor);

            // recurse
            self.walk(nx, ny)
        }
    }
}
