use crate::cellular::*;
use crate::drunk::*;
use crate::maze::*;
use crate::mineral_placement::*;
use crate::mob_placement::*;
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
    pub composition: MineralPlacementOptions,
    pub inhabitants: MobPlacementOptions,
    pub algorithms: Vec<MapgenAlgorithm>,
}

#[derive(Debug, Deserialize)]
pub struct DungeonSpecification {
    pub world_name: String,
    pub layers: Vec<LayerSpecification>,
}
