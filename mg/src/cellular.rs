use std::vec::Vec;
use rand::prelude::*;
use crate::dun_s1::*;

#[derive(Clone, Debug)]
pub enum JobType {
    RandomFill,
    Generation(bool),  // allow_islands
    FloorBar(usize),   // height
}

pub struct CellularAutomata<'a, R: Rng> {
    map: &'a mut DungeonS1,
    open_space_percentage: usize,
    wall_requirement: usize,
    island_requirement: usize,
    rng: &'a mut R,
    schedule: Vec<JobType>,
}

impl<'a, R: Rng> CellularAutomata<'a, R> {
    pub fn new(
        map: &'a mut DungeonS1,
        rng: &'a mut R
    ) -> CellularAutomata<'a, R> {
        CellularAutomata {
            map: map,
            open_space_percentage: 64,
            wall_requirement: 6,
            island_requirement: 2,
            rng: rng,
            schedule: Vec::new(),
        }
    }

    pub fn schedule_job(&'a mut self, job: JobType) -> &'a mut CellularAutomata<'a, R> {
        self.schedule.push(job);
        self
    }

    pub fn do_work(&mut self) {
        for job in self.schedule.clone() {
            match job {
                JobType::RandomFill => self.random_fill(),
                JobType::Generation(i) => self.generation(i),
                JobType::FloorBar(h) => self.floor_bar(h),
            }
        }
    }

    pub fn open_space_percentage(&'a mut self, chance: usize) -> &'a mut CellularAutomata<'a, R> {
        self.open_space_percentage = chance;
        self
    }

    pub fn wall_requirement(&'a mut self, requirement: usize) -> &'a mut CellularAutomata<'a, R> {
        self.wall_requirement = requirement;
        self
    }

    pub fn island_requirement(&'a mut self, requirement: usize) -> &'a mut CellularAutomata<'a, R> {
        self.island_requirement = requirement;
        self
    }

    pub fn random_fill(&mut self) {
        // fill map randomly
        self.map.rand_fill(self.rng, self.open_space_percentage);
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

                if neighboring_walls >= self.wall_requirement {
                    self.map.set(x, y, TileType::Wall);
                } else {
                    if allow_islands &&
                        neighboring_walls <= self.island_requirement {
                            self.map.set(x, y, TileType::Wall);
                    } else {
                        self.map.set(x, y, TileType::Floor);
                    }
                }
            }
        }
    }
}
