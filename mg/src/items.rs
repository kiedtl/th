use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum ItemType {
    Block,
    Weapon, Armor,
    Shoes, Headwear,
    Clothing,
    DiggingTool,
}
