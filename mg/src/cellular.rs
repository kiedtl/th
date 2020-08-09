use rand::prelude::*;

pub struct CellularAutomata<'a, R: Rng> {
    map: &'a mut [[bool; 205]; 50],
    open_space_percentage: usize,
    wall_requirement: usize,
    island_requirement: usize,
    rng: &'a mut R,
}

impl<R: Rng> CellularAutomata<'_, R> {
    pub fn new<'a>(
        map: &'a mut [[bool; 205]; 50],
        rng: &'a mut R
    ) -> CellularAutomata<'a, R> {
        CellularAutomata {
            map: map,
            open_space_percentage: 64,
            wall_requirement: 6,
            island_requirement: 2,
            rng: rng
        }
    }

    pub fn open_space_percentage(&mut self, chance: usize) {
        self.open_space_percentage = chance;
    }

    pub fn wall_requirement(&mut self, requirement: usize) {
        self.wall_requirement = requirement;
    }

    pub fn island_requirement(&mut self, requirement: usize) {
        self.island_requirement = requirement;
    }

    pub fn random_fill(&mut self) {
        // fill map randomly
        for y in 0..50 {
            for x in 0..205 {
                self.map[y][x] = self.rng.gen_range(0, 100) < 64;
            }
        }
    }

    pub fn add_floor_bar(&mut self, height: usize) {
        // add a horizontal bar of floors in the center of the map
        // as it may prevent a continuous vertical wall from forming,
        // thus preventing isolated sections
        for y in ((50 / 2) as usize)..(((50 / 2) + height) as usize) {
            for x in 0_usize..204_usize {
                self.map[y][x] = true;
            }
        }
    }

    pub fn generation(&mut self, allow_islands: bool) {
        let oldmap = self.map.clone();
        for y in 0_usize..49_usize {
            for x in 0_usize..204_usize {
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
                    if !oldmap[ny][nx] {
                        neighboring_walls += 1;
                    }
                }

                if neighboring_walls >= self.wall_requirement {
                    self.map[y][x] = false;
                } else {
                    if allow_islands &&
                        neighboring_walls <= self.island_requirement {
                            self.map[y][x] = false;
                    } else {
                        self.map[y][x] = true;
                    }
                }
            }
        }
    }
}
