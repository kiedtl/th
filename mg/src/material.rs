use crate::colors::*;
use crate::items::*;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum StoneType {
    Sedimentary,
    IgneousExtrusive,
    IgneousIntrusive,
    Metamorphic,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StoneInfo {
    pub stone_type: StoneType,
    pub found_near: Vec<MaterialInfo>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum MaterialClass {
    Stone(StoneInfo), Metal,
    Flesh, Bone,
    Other,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MaterialBurnInfo {
    pub burning_point: f64,
    pub burn_rate: usize, // gram per second
    pub energy_per_gram: usize, // joules
}

#[derive(Clone, Debug, Deserialize)]
pub struct MaterialInfo {
    // name of the material
    // e.g. "rhyolite"
    pub name: String,

    // description
    // e.g. "An unusually strong metal
    // typically used to make armour and weapons"
    pub description: String,

    // what kind of material?
    // e.g. Metal
    pub class: MaterialClass,

    pub rarity: u8,

    // density (in g/cm³)
    pub density: f64,

    // color in RGBA. alpha value determines
    // transparency of material
    pub color: Color,

    // all temperatures are in kelvin
    pub melting_point: f64,

    pub combustible: Option<MaterialBurnInfo>,

    // the material that results after smelting
    // this material
    // e.g. "hematite" => "iron"
    pub smelt_result: Option<String>,

    // hardness in mohs
    pub hardness: f64,

    // specific heat in kJ/(kg K)
    pub specific_heat: f64,

    // how much light material emits in lumens
    pub emit_light: usize,

    pub occurs_naturally: Option<Vec<ItemType>>,

    pub edible: bool,
}
