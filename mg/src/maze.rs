// create a perfect maze using a
// recursive backtracking algorithm.

use serde::Deserialize;
use rand::prelude::*;
use crate::rect::*;
use crate::dirs::*;
use crate::features::*;
use crate::dun_s1::*;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct MazeOptions {
    // If set to None, will not remove dead ends.
    // If set to Some<i>, will remove <i> blocks
    // from each dead end.
    // Beware that in a perfect maze, every path
    // is a dead end.
    remove_dead_ends: Option<usize>,

    // connect maze to existing features?
    connect_to_features: bool,
}

impl MazeOptions {
    pub fn new() -> MazeOptions {
        MazeOptions {
            remove_dead_ends: Some(1),
            connect_to_features: false,
        }
    }

    pub fn remove_dead_ends(mut self, val: Option<usize>) -> MazeOptions {
        self.remove_dead_ends = val;
        self
    }

    pub fn connect_to_features(mut self, val: bool) -> MazeOptions {
        self.connect_to_features = val;
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
        // get random start
        let mut startx: usize = self.rng.gen_range(0, self.map.width);
        let mut starty: usize = self.rng.gen_range(0, self.map.height);

        while self.point_intersects_features(startx as isize, starty as isize) {
            startx = self.rng.gen_range(0, self.map.width);
            starty = self.rng.gen_range(0, self.map.height);
        }

        self.map.set(startx, starty, TileType::Debug);

        self.walk(startx as isize, starty as isize);

        if self.options.connect_to_features {
            self.connect_to_features();
        }

        match self.options.remove_dead_ends {
            Some(g) => self.remove_dead_ends(g),
            None => (),
        }

    }

    pub fn connect_to_features(&mut self) {
        let max_doors   = 2;

        for feature in &self.map.features {
            let f;
            match feature {
                Feature::Tunnel(r)
                | Feature::Room(r) => {
                    f = r;
                },
            }

            let mut doorctr = 0;

            // take each wall in random order and try
            // to dig a door space there
            let walls = &mut [Direction::North, Direction::South,
                Direction::East, Direction::West];
            walls.shuffle(self.rng);

            for wall_side in walls {
                if doorctr >= max_doors {
                    break;
                }

                let wall = f.wall(wall_side);

                // get number of coords that have at least
                // two floors next to them
                for coord in wall {
                    // TODO: create coord struct, with
                    // get_neighbors function
                    let neighbors: &[(isize, isize)] =
                        &[(-1, 0), (0,  -1), (0,  1), (1,  0)];

                    let mut neighboring_floors = 0;
                    let mut neighboring_walls  = 0;
                    for neighbor in neighbors {
                        let (ny, nx) = ((coord.0 as isize) + neighbor.0,
                            (coord.1 as isize) + neighbor.1);
                        if nx <= 0 || ny <= 0 {
                            continue;
                        }

                        if nx >= ((self.map.width - 1) as isize)
                            || ny >= ((self.map.height - 1) as isize) {
                                continue;
                        }

                        if self.map.d[ny as usize][nx as usize] == TileType::Floor {
                            neighboring_floors += 1;
                        } else {
                            neighboring_walls += 1;
                        }
                    }

                    if neighboring_floors == 2 && neighboring_walls == 2 &&
                        self.map.d[coord.0][coord.1] == TileType::Wall {
                            self.map.d[coord.0][coord.1] = TileType::Floor;
                            doorctr += 1;
                            break;
                    }
                }
            }
        }
    }

    fn remove_dead_ends(&mut self, goal: usize) {
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

    fn point_intersects_features(&self, cx: isize, cy: isize) -> bool {
        for f in &self.map.features {
            match f {
                Feature::Room(r)
                | Feature::Tunnel(r) => {
                    // TODO: use rect::Rect::intersects
                    for y in (r.y1.saturating_sub(2))..r.y2 + 1 {
                        for x in (r.x1.saturating_sub(2))..r.x2 + 1 {
                            if cx == (x as isize) && cy == (y as isize) {
                                return true;
                            }
                        }
                    }
                },
            }
        }

        false
    }

    fn walk(&mut self, cx: isize, cy: isize) {
        // squares on the North, East, West, South
        let neighbors = &mut [((-1, 0), (-2, 0)), ((0,  -1), (0,  -2)),
                              ((0,  1), (0,  2)), ((1,   0), (2,  0))];

        neighbors.shuffle(self.rng);

        // iterate through shuffled neighbors and create paths in
        // each direction if there's enough place
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

            if self.map.d[ny as usize][nx as usize] != TileType::Wall ||
                self.map.d[iy as usize][ix as usize] != TileType::Wall {
                    continue;
            }

            if self.point_intersects_features(nx, ny) ||
                self.point_intersects_features(ix, iy) {
                    continue;
            }

            self.map.set(nx as usize, ny as usize, TileType::Floor);
            self.map.set(ix as usize, iy as usize, TileType::Floor);

            // recurse
            self.walk(nx, ny);
        }
    }
}
