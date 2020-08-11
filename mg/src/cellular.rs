use std::vec::Vec;
use serde::Deserialize;
use rand::prelude::*;

use crate::dun_s1::*;

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum JobType {
    RandomFill,
    Generation(bool),  // allow_islands
    FloorBar(usize),   // height
}

#[derive(Clone, Debug, Deserialize)]
pub struct CellularAutomataOptions {
    open_space_percentage: usize,
    wall_requirement: usize,
    island_requirement: usize,
    schedule: Vec<JobType>,
}

impl CellularAutomataOptions {
    pub fn new() -> CellularAutomataOptions {
        CellularAutomataOptions {
            open_space_percentage: 64,
            wall_requirement: 6,
            island_requirement: 2,
            schedule: Vec::new(),
        }
    }

    pub fn schedule_job(mut self, job: JobType) -> CellularAutomataOptions {
        self.schedule.push(job);
        self
    }

    pub fn open_space_percentage(mut self, chance: usize) -> CellularAutomataOptions {
        self.open_space_percentage = chance;
        self
    }

    pub fn wall_requirement(mut self, requirement: usize) -> CellularAutomataOptions {
        self.wall_requirement = requirement;
        self
    }

    pub fn island_requirement(mut self, requirement: usize) -> CellularAutomataOptions {
        self.island_requirement = requirement;
        self
    }
}

pub struct CellularAutomata<'a, R: Rng> {
    map: &'a mut DungeonS1,
    options: CellularAutomataOptions,
    rng: &'a mut R,
}

impl<'a, R: Rng> CellularAutomata<'a, R> {
    pub fn new(
        map: &'a mut DungeonS1,
        rng: &'a mut R,
        opt: CellularAutomataOptions,
    ) -> CellularAutomata<'a, R> {
        CellularAutomata {
            map: map,
            options: opt,
            rng: rng,
        }
    }

    pub fn do_work(&mut self) {
        for job in self.options.schedule.clone() {
            match job {
                JobType::RandomFill => self.random_fill(),
                JobType::Generation(i) => self.generation(i),
                JobType::FloorBar(h) => self.floor_bar(h),
            }
        }
    }

    pub fn random_fill(&mut self) {
        // fill map randomly
        self.map.rand_fill(self.rng, self.options.open_space_percentage);
    }

    pub fn floor_bar(&mut self, height: usize) {
        // add a horizontal bar of floors in the center of the map
        // as it may prevent a continuous vertical wall from forming,
        // thus preventing isolated sections
        let halfway = (self.map.height / 2) as usize;
        for y in halfway..(halfway + height) {
            for x in 0_usize..self.map.width {
                self.map.set(x, y, TileType::Floor);
            }
        }
    }

    pub fn generation(&mut self, allow_islands: bool) {
        let oldmap = self.map.clone();
        for y in 0_usize..(self.map.height - 1) {
            for x in 0_usize..(self.map.width - 1) {
                // all eight surrounding tiles
                let neighbors: &[(usize, usize); 9] =
                    &[(y.saturating_sub(1), x.saturating_sub(1)),
                        (y.saturating_sub(1), x), (y.saturating_sub(1), x.saturating_add(1)),
                    (y, x.saturating_sub(1)), (y, x), (y, x.saturating_add(1)),
                    (y.saturating_add(1), x.saturating_sub(1)),
                        (y.saturating_add(1), x), (y.saturating_add(1), x.saturating_add(1))];
                let mut neighboring_walls = 0;

                for neighbor in neighbors {
                    let (ny, nx) = *neighbor;
                    if oldmap.d[ny][nx] == TileType::Wall {
                        neighboring_walls += 1;
                    }
                }

                if neighboring_walls >= self.options.wall_requirement {
                    self.map.set(x, y, TileType::Wall);
                } else {
                    if allow_islands &&
                        neighboring_walls <= self.options.island_requirement {
                            self.map.set(x, y, TileType::Wall);
                    } else {
                        self.map.set(x, y, TileType::Floor);
                    }
                }
            }
        }
    }
}
