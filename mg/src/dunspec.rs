use crate::cellular::*;
use crate::drunk::*;
use crate::material::*;
use crate::maze::*;
use crate::randrm::*;
use serde::Deserialize;
use std::vec::Vec;

#[derive(Debug, Deserialize)]
pub enum MapgenAlgorithm {
    Drunkard(DrunkardOptions),
    Cellular(CellularAutomataOptions),
    RandomRooms(RandomRoomsOptions),
    Maze(MazeOptions),
}

#[derive(Debug, Deserialize)]
pub struct LayerSpecification {
    pub levels: usize,
    pub dimensions: (usize, usize),   // (width, height)
    pub algorithms: Vec<MapgenAlgorithm>,
    pub stone_type: StoneType,
}

#[derive(Debug, Deserialize)]
pub struct DungeonSpecification {
    pub layers: Vec<LayerSpecification>,
}
