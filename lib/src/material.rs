use crate::colors::*;
use crate::id::*;
use crate::items::*;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum StoneType {
    Sedimentary,
    IgneousExtrusive,
    IgneousIntrusive,
    Metamorphic,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StoneInfo {
    pub stone_type: StoneType,
    pub found_near: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum MaterialClass {
    Stone(StoneInfo), Metal,
    Flesh, Bone,
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
    pub color_bg: Color,
    pub color_fg: Color,
    pub block_glyph: char,

    // all temperatures are in kelvin
    pub melting_point: f64,

    pub combustible: Option<f64>, // burn point

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

impl Id for MaterialInfo {
    fn id(&self) -> String {
        self.name.clone()
    }
}

impl MaterialInfo {
    pub fn found_near(&self) -> Option<Vec<String>> {
        if let MaterialClass::Stone(stoneinfo) = self.class.clone() {
            return Some(stoneinfo.found_near);
        } else {
            return None;
        }
    }
}
