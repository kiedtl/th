use crate::colors::*;
use crate::id::*;
use crate::value::*;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MobBody {
    Eye,
    Ear,
    Head,
    Hand,
    Claw,
    Foot,
    Tail,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MobGender {
    Male,
    NonBinary,
    Female,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MobClass {
    UpperMob,
    MiddleMob,
    SauronsDenMob,
    LowerMob,
    MorgothsLairMob,
    LowestMob,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MobMovement {
    Immobile,
    Sedentary,
    LightlyActive,
    Active,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MobAlignment {
    Hostile,        // the creatures is hostile to the player
    NeutralHostile, // the creature is neutral but will turn hostile if attacked
    Neutral,        // the creature is neutral, and will flee if attacked
    Friendly,       // the creature is friendly, and will flee if attacked
}

#[derive(Clone, Debug, Deserialize)]
pub struct MobTemplate {
    // must be unique
    // e.g "burning_brute"
    pub id: String,

    pub class: MobClass,

    // e.g. "Burning Brute"
    pub short_name: String,

    // e.g. "Brute of Burning"
    pub long_name: String,

    // tagline: used only for uniques
    // sort of like a title
    // e.g., the tagline for morgoth would be something like "lord of angband"
    pub tagline: String,

    // e.g. "A winged demon made of boiling granite,
    // capable of fearsome fire attacks"
    pub description: String,

    // probability of genders
    pub allowed_genders: Vec<(MobGender, u8)>,

    pub body: Vec<MobBody>,

    pub ascii_glyph: char,
    pub unicode_glyph: char,
    pub glyph_fg: Option<Color>,

    pub alignment: MobAlignment,
    pub height: Value<u16>, // centimeters
    pub width: Value<u16>, // centimeters also
    pub weight: Value<u16>, // kilograms
    pub composition: String, // what material is it made of

    pub normal_body_temperature: Value<usize>,
    pub min_body_temperature: Value<usize>,
    pub max_body_temperature: Value<usize>,

    pub needs_food: bool,
    pub needs_drink: bool,
    pub needs_sleep: bool,

    // stuff the character will yell at the player from time to time
    // greetings are yelled when the character first spots the player
    // misc_lines are yelled during combat
    // farewells are yelled just as the player leavs the character's
    // line of vision
    pub greetings: Vec<String>,
    pub misc_lines: Vec<String>,
    pub farewells: Vec<String>,

    // strength and agility should be obvious
    pub strength: Value<u8>,
    pub agility: Value<u8>,

    // endurance controls how quickly a character faints from overexertion,
    // as well as the total blood supply
    pub endurance: Value<u8>,

    // metabolism controls how quickly lost blood is replenished, how fast
    // lost health is regained, and how often the character must eat
    pub metabolism: Value<u8>,

    // willpower controls to what extent the character can resist
    // ranged spells
    pub willpower: Value<u8>,

    // focus determines the accuracy of ranged attacks the character makes,
    // as well as how easily the character gets distracted
    pub focus: Value<u8>,

    // bravery controls how easily the character gets scared
    // ranges from 0 (coward) to 255 (fearless)
    pub bravery: Value<u8>,

    pub intelligence: Value<u8>,
    pub aggressive: Value<u8>,

    pub movement: MobMovement,

    // what age is the character right now? (in years)
    pub age: Value<u64>,
    pub max_age: Option<Value<u64>>, // demons don't die of old age

    // most demons will have this
    // controls whether character will be able to summon other demons
    // to their aid
    pub summoner: bool,
    pub summonable: bool,

    pub undead: bool,

    // *most* undead creatures will have this
    // a creature that is opposed_to_life will attack any living thing, even
    // a servant of Morgoth, even Morgoth himself lol
    pub opposed_to_life: bool,

    // controls how many corpses that character can raise non-necromancers
    // will have this set to 0
    pub necromancer: Value<u8>,

    // can the thing suck blood
    // if set to true, then needs_{sleep, food, drink} is ignored
    // i mean, why would a vampire eat or sleep
    pub vampire: bool,

    pub is_unique: bool,
    pub max_in_map: usize,

    // TODO list
    // - list of body parts
    // - fire_breathing: bool,
    // - preferred_weapons: Vec<Item>,
    // - preferred_occupations: Vec<Occupation>,
    // - polymorph_into: Vec<String>, // for werewolves
    // - can_cast_spells: bool,
    // - preferred_spells: Vec<Spell>,
}

impl Id for MobTemplate {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl MobTemplate {
    pub fn generate_mob<R>(&self, rng: &mut R) -> Mob 
    where
        R: Rng
    {
        let max_age: Option<u64>;
        if let Some(v) = self.max_age {
            max_age = Some(v.get(rng));
        } else {
            max_age = None;
        }

        Mob {
            from_mob_template: self.id.clone(),
            gender: self.allowed_genders
                .choose_weighted(rng, |g| g.1).unwrap().0,
            body: self.body.clone(),
            ascii_glyph: self.ascii_glyph,
            unicode_glyph: self.unicode_glyph,
            glyph_fg: self.glyph_fg,
            alignment: self.alignment,
            height: self.height.get(rng),
            width: self.width.get(rng),
            weight: self.weight.get(rng),
            normal_body_temperature: self.normal_body_temperature.get(rng),
            min_body_temperature: self.max_body_temperature.get(rng),
            max_body_temperature: self.min_body_temperature.get(rng),
            strength: self.strength.get(rng),
            agility: self.agility.get(rng),
            endurance: self.endurance.get(rng),
            metabolism: self.metabolism.get(rng),
            willpower: self.willpower.get(rng),
            focus: self.focus.get(rng),
            bravery: self.bravery.get(rng),
            intelligence: self.intelligence.get(rng),
            aggressive: self.aggressive.get(rng),
            age: self.age.get(rng),
            max_age: max_age,
            undead: self.undead,
            opposed_to_life: self.opposed_to_life,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mob {
    // fields that are not unique to each specific mob (e.g. short_name,
    // vampire, or needs_drink) are not put here.
    pub from_mob_template: String,

    pub gender: MobGender,
    pub body: Vec<MobBody>,

    pub ascii_glyph: char,
    pub unicode_glyph: char,
    pub glyph_fg: Option<Color>,

    pub alignment: MobAlignment,
    pub height: u16,
    pub width: u16,
    pub weight: u16,

    pub normal_body_temperature: usize,
    pub min_body_temperature: usize,
    pub max_body_temperature: usize,

    pub strength: u8,
    pub agility: u8,
    pub endurance: u8,
    pub metabolism: u8,
    pub willpower: u8,
    pub focus: u8,
    pub bravery: u8,
    pub intelligence: u8,
    pub aggressive: u8,

    pub age: u64,
    pub max_age: Option<u64>,

    pub undead: bool,
    pub opposed_to_life: bool,
}
