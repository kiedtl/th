use crate::dirs::*;

pub struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from(s: (usize, usize)) -> Self {
        Coord {
            y: s.0,
            x: s.1,
        }
    }
}

impl From<(i32, i32)> for Coord {
    fn from(s: (i32, i32)) -> Self {
        Coord {
            y: s.0 as usize,
            x: s.1 as usize,
        }
    }
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord {
            x: x,
            y: y,
        }
    }

    pub fn as_yx<T>(&self) -> (T, T)
    where
        T: From<usize> {
        (T::from(self.y), T::from(self.x))
    }

    pub fn as_xy<T>(&self) -> (T, T)
    where
        T: From<usize>
    {
        (T::from(self.x), T::from(self.y))
    }

    pub fn neighbor_in_direction(&self, d: Direction) -> Coord {
        match d {
            Direction::North =>
                Coord { x: self.x, y: self.y.saturating_sub(1) },
            Direction::South =>
                Coord { x: self.x, y: self.y + 1 },
            Direction::East =>
                Coord { x: self.x + 1, y: self.y },
            Direction::West =>
                Coord { x: self.x.saturating_sub(1), y: self.y },
            Direction::NorthEast =>
                Coord { x: self.x + 1, y: self.y.saturating_sub(1) },
            Direction::NorthWest =>
                Coord { x: self.x.saturating_sub(1), y: self.y.saturating_sub(1) },
            Direction::SouthEast =>
                Coord { x: self.x + 1, y: self.y + 1 },
            Direction::SouthWest =>
                Coord { x: self.x.saturating_sub(1), y: self.y + 1 },
        }
    }

    pub fn clamp_x(mut self, max: usize) -> Coord {
        let newx = match () {
            _ if self.x < max => self.x,
            _ if self.x > max => max,
            _ => self.x,
        };

        self.x = newx;
        self
    }

    pub fn clamp_y(mut self, max: usize) -> Coord {
        let newy = match () {
            _ if self.y < max => self.y,
            _ if self.y > max => max,
            _ => self.y,
        };

        self.y = newy;
        self
    }
}
