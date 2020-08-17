use crate::colors::*;
use crate::items::*;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum StoneType {
    IgneousExtrusive,
    Metamorphic,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StoneInfo {
    stone_type: StoneType,
    found_near: Vec<MaterialInfo>,
}

#[derive(Clone, Debug, Deserialize)]
pub enum MaterialClass {
    Stone(StoneInfo), Metal,
    Flesh, Bone,
    Other,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MaterialBurnInfo {
    burning_point: f64,
    burn_rate: usize, // gram per second
    energy_per_gram: usize, // joules
}

#[derive(Clone, Debug, Deserialize)]
pub struct MaterialInfo {
    // name of the material
    // e.g. "rhyolite"
    name: String,

    // description
    // e.g. "An unusually strong metal
    // typically used to make armour and weapons"
    description: String,

    // what kind of material?
    // e.g. Metal
    class: MaterialClass,

    rarity: u8,

    // density (in g/cm³)
    density: f64,

    // color in RGBA. alpha value determines
    // transparency of material
    color: Color,

    // all temperatures are in kelvin
    melting_point: f64,

    combustible: Option<MaterialBurnInfo>,

    // the material that results after smelting
    // this material
    // e.g. "hematite" => "iron"
    smelt_result: Option<String>,

    // hardness in mohs
    hardness: f64,

    // specific heat in kJ/(kg K)
    specific_heat: f64,

    // how much light material emits in lumens
    emit_light: usize,

    occurs_naturally: Option<Vec<ItemType>>,

    edible: bool,
}
