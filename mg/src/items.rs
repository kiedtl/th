use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
pub enum ItemType {
    Block,
    Weapon, Armor,
    Shoes, Headwear,
    Clothing,
    DiggingTool,
}
