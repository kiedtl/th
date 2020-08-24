use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0, 3) {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::East,
            _ => Direction::West,
            //4 => Direction::NorthWest,
            //5 => Direction::NorthEast,
            //6 => Direction::SouthWest,
            //_ => Direction::SouthEast,
        }
    }
}

