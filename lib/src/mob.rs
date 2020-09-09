use crate::colors::*;
use crate::id::*;
use crate::value::*;
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};

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

#[derive(Copy, Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum MobGender {
    Male,
    NonBinary,
    Female,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MobClass {
    ThrallMob,
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
    VeryActive,
}

impl MobMovement {
    pub fn chance_of_movement(&self) -> usize {
        // return chance in 100 of mob moving
        match self {
            MobMovement::Immobile => 0,
            MobMovement::Sedentary => 10,
            MobMovement::LightlyActive => 30,
            MobMovement::Active => 80,
            MobMovement::VeryActive => 100,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Serialize, Deserialize)]
pub enum MobAlignment {
    // attacks if attacked, attacks if not attacked
    Hostile,

    // attacks if attacked, peaceful otherwise
    Neutral,

    // prefers to flee if attacked. other Hostile mobs
    // will attack it. will attack Hostile mobs.
    Friendly,
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

            max_strength: self.strength.get(rng),
            max_agility: self.agility.get(rng),
            max_endurance: self.endurance.get(rng),
            max_metabolism: self.metabolism.get(rng),
            max_willpower: self.willpower.get(rng),
            max_focus: self.focus.get(rng),
            max_intelligence: self.intelligence.get(rng),

            strength: 100,
            agility: 100,
            endurance: 100,
            metabolism: 100,
            willpower: 100,
            focus: 100,
            intelligence: 100,

            bravery: self.bravery.get(rng),
            aggressive: self.aggressive.get(rng),
            age: self.age.get(rng),
            max_age: max_age,
            undead: self.undead,
            opposed_to_life: self.opposed_to_life,
            current_mode: MobMode::Wander,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MobMode {
    AttackMob,
    Eat,
    Drink,
    Sleep,
    CompleteJob,
    FindJob,
    Wander,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mob {
    // fields that are not unique to each specific mob (e.g. short_name,
    // vampire, or needs_drink) are not put here.

    // these fields are UNIQUE to each mob
    // and will never change
    // <unique>
    pub from_mob_template: String,

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

    pub gender: MobGender,

    pub max_strength: u8,
    pub max_agility: u8,
    pub max_endurance: u8,
    pub max_metabolism: u8,
    pub max_willpower: u8,
    pub max_focus: u8,
    pub max_intelligence: u8,
    // </unique>

    // percentage of max_<field>
    // so for example, a mob may have a max
    // strength of 100 but due to, say,
    // hunger or drowsiness will have
    // only 20% of max_strength (so
    // strength=20 instead of 100)
    //
    // these fields will change for each mob
    // as the game progresses
    pub strength: u8,
    pub agility: u8,
    pub endurance: u8,
    pub metabolism: u8,
    pub willpower: u8,
    pub focus: u8,
    pub intelligence: u8,

    pub bravery: u8,
    pub aggressive: u8,

    pub age: u64,
    pub max_age: Option<u64>,

    pub undead: bool,
    pub opposed_to_life: bool,

    pub current_mode: MobMode,
}

impl std::hash::Hash for Mob {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher
    {
        // hash unique, non-changing fields
        self.from_mob_template.hash(state);
        self.ascii_glyph.hash(state);
        self.glyph_fg.hash(state);
        self.alignment.hash(state);
        self.height.hash(state);
        self.width.hash(state);
        self.weight.hash(state);
        self.normal_body_temperature.hash(state);
        self.min_body_temperature.hash(state);
        self.max_body_temperature.hash(state);
        self.gender.hash(state);
        self.max_strength.hash(state);
        self.max_agility.hash(state);
        self.max_endurance.hash(state);
        self.max_metabolism.hash(state);
        self.max_focus.hash(state);
        self.max_intelligence.hash(state);
    }
}
