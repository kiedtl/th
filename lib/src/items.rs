use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    Block,
    Weapon, Armor,
    Shoes, Headwear,
    Clothing,
    DiggingTool,
}
