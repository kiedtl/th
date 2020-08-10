use rand::prelude::*;
use crate::dirs::*;
use crate::dun_s1::*;

pub struct Drunkard<'a, R: Rng> {
    map: &'a mut DungeonS1,
    max_iterations: usize,
    center_weight: f64,
    previous_direction_weight: f64,
    filled_goal: f64,
    rng: &'a mut R,
}

impl<'a, R: Rng> Drunkard<'a, R> {
    pub fn new(map: &'a mut DungeonS1, rng: &'a mut R) -> Drunkard<'a, R> {
        Drunkard {
            map: map,
            max_iterations: 5000,
            center_weight: 0.1,
            previous_direction_weight: 0.65,
            filled_goal: 0.25,
            rng: rng,
        }
    }

    pub fn max_iterations(&mut self, iterations: usize) -> &mut Drunkard<'a, R> {
        self.max_iterations = iterations;
        self
    }

    pub fn center_weight(&mut self, weight: f64) -> &mut Drunkard<'a, R> {
        self.center_weight = weight;
        self
    }

    pub fn previous_direction_weight(&mut self, weight: f64) -> &mut Drunkard<'a, R> {
        self.previous_direction_weight = weight;
        self
    }

    pub fn filled_goal(&mut self, goal: f64) -> &mut Drunkard<'a, R> {
        self.filled_goal = goal;
        self
    }

    pub fn walk(&mut self) {
        let mut filled = 0;
        let mut previous_direction = Direction::North;
        let mut iterations = 0;

        let filled_goal  = ((self.map.width * self.map.height) as f64) * self.filled_goal;
        let mut walker_pos_x: isize = (self.map.width / 2)  as isize;
        let mut walker_pos_y: isize = (self.map.height / 2) as isize;

        let mut iteration = || {
            // probability of going in a direction
            let mut north = 1.0;
            let mut south = 1.0;
            let mut east  = 1.0;
            let mut west  = 1.0;

            if self.map.width > self.map.height {
                east += east * (self.map.width as f64 / self.map.height as f64);
                west += west * (self.map.width as f64 / self.map.height as f64);
            } else if self.map.height > self.map.width {
                north += north * (self.map.height as f64 / self.map.width as f64);
                south += north * (self.map.height as f64 / self.map.width as f64);
            }

            // weight the random walk against map edges
            if (walker_pos_x as f64) < (self.map.width as f64) * 0.25 {
                // walker is at far left
                east += self.center_weight;
            } else if (walker_pos_x as f64) > (self.map.width as f64) * 0.75 {
                // walker is at far right
                west += self.center_weight;
            }

            if (walker_pos_y as f64) < (self.map.height as f64) * 0.25 {
                // walker is at the top
                south += self.center_weight;
            } else if (walker_pos_y as f64) > (self.map.height as f64) * 0.75 {
                // walker is at the bottom
                north += self.center_weight;
            }

            match previous_direction {
                Direction::North => north += self.previous_direction_weight,
                Direction::South => south += self.previous_direction_weight,
                Direction::West  => west  += self.previous_direction_weight,
                Direction::East  => east  += self.previous_direction_weight,
                _ => (),
            }

            // normalize probabilities so they form a range from 0..1
            let total = north + south + east + west;
            north /= total;
            south /= total;
            east /= total;
            west /= total; // this is unused

            // choose the direction to walk into
            let dx: isize;
            let dy: isize;
            let mut direction = [(Direction::North, north), (Direction::South, south),
                (Direction::East, east), (Direction::West, west)]
                    .choose_weighted(&mut self.rng, |i| i.1).unwrap().0;

            if direction == Direction::North {
                dx = 0;
                dy = -1;
            } else if direction == Direction::South {
                dx = 0;
                dy = 1;
            } else if direction == Direction::East {
                dx = 1;
                dy = 0;
            } else if direction == Direction::West {
                dx = -1;
                dy = 0;
            } else {
                // wait wat
                dx = 0;
                dy = 0;
            }

            // the actual walking
            if (0 < walker_pos_x + dx && walker_pos_x + dx < (self.map.width as isize) - 1) &&
                (0 < walker_pos_y + dy && walker_pos_y + dy < (self.map.height as isize) - 1) {
                    walker_pos_x += dx;
                    walker_pos_y += dy;

                    if self.map.d[walker_pos_y as usize][walker_pos_x as usize] == TileType::Wall {
                        self.map.set(walker_pos_x as usize, walker_pos_y as usize, TileType::Floor);
                        filled += 1;
                    }

                    previous_direction = direction;
            }

            iterations += 1;
            ((filled as f64) <= filled_goal) ||
                iterations <= self.max_iterations
        };

        while iteration() {
        }
    }
}
