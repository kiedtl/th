use crate::colors::*;
use crate::value::*;
use rand::prelude::*;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
enum MobMovement {
    Immobile,
    Sedentary,
    LightlyActive,
    Active,
}

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
enum MobAlignment {
    Hostile,        // the creatures is hostile to the player
    NeutralHostile, // the creature is neutral but will turn hostile if attacked
    Neutral,        // the creature is neutral, and will flee if attacked
    Friendly,       // the creature is friendly, and will flee if attacked
}

#[derive(Clone, Debug, Deserialize)]
struct MobTemplate {
    // must be unique
    // e.g "burning_brute"
    id: String,

    // e.g. "Burning Brute"
    short_name: String,

    // e.g. "Brute of Burning"
    long_name: String,

    // tagline: used only for uniques
    // sort of like a title
    // e.g., the tagline for morgoth would be something like "lord of angband"
    tagline: String,

    // e.g. "A winged demon made of boiling granite,
    // capable of fearsome fire attacks"
    description: String,

    ascii_glyph: char,
    unicode_glyph: char,
    glyph_fg: Option<Color>,

    alignment: MobAlignment,
    height: Value<u16>, // centimeters
    width: Value<u16>, // centimeters also
    weight: Value<u16>, // kilograms
    composition: String, // what material is it made of

    normal_body_temperature: Value<usize>,
    min_body_temperature: Value<usize>,
    max_body_temperature: Value<usize>,

    needs_food: bool,
    needs_drink: bool,
    needs_sleep: bool,

    // stuff the character will yell at the player from time to time
    // greetings are yelled when the character first spots the player
    // misc_lines are yelled during combat
    // farewells are yelled just as the player leavs the character's
    // line of vision
    greetings: Vec<String>,
    misc_lines: Vec<String>,
    farewells: Vec<String>,

    // strength and agility should be obvious
    strength: Value<u8>,
    agility: Value<u8>,

    // endurance controls how quickly a character faints from overexertion,
    // as well as the total blood supply
    endurance: Value<u8>,

    // metabolism controls how quickly lost blood is replenished, how fast
    // lost health is regained, and how often the character must eat
    metabolism: Value<u8>,

    // willpower controls to what extent the character can resist
    // ranged spells
    willpower: Value<u8>,

    // focus determines the accuracy of ranged attacks the character makes,
    // as well as how easily the character gets distracted
    focus: Value<u8>,

    // bravery controls how easily the character gets scared
    // ranges from 0 (coward) to 255 (fearless)
    bravery: Value<u8>,

    intelligence: Value<u8>,
    aggressive: Value<u8>,

    movement: MobMovement,

    // what age is the character right now? (in years)
    age: Value<u64>,
    max_age: Option<Value<u64>>, // demons don't die of old age

    // most demons will have this
    // controls whether character will be able to summon other demons
    // to their aid
    summoner: bool,
    summonable: bool,

    undead: bool,

    // *most* undead creatures will have this
    // a creature that is opposed_to_life will attack any living thing, even
    // a servant of Morgoth, even Morgoth himself lol
    opposed_to_life: bool,

    // controls how many corpses that character can raise non-necromancers
    // will have this set to 0
    necromancer: Value<u8>,

    // can the thing suck blood
    // if set to true, then needs_{sleep, food, drink} is ignored
    // i mean, why would a vampire eat or sleep
    vampire: bool,

    is_unique: bool,
    max_in_map: Option<usize>,

    // TODO list
    // - list of body parts
    // - fire_breathing: bool,
    // - preferred_weapons: Vec<Item>,
    // - preferred_occupations: Vec<Occupation>,
    // - polymorph_into: Vec<String>, // for werewolves
    // - can_cast_spells: bool,
    // - preferred_spells: Vec<Spell>,
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

struct Mob {
    // fields that are not unique to each specific mob (e.g. short_name,
    // vampire, or needs_drink) are not put here.
    from_mob_template: String,

    alignment: MobAlignment,
    height: u16,
    width: u16,
    weight: u16,

    normal_body_temperature: usize,
    min_body_temperature: usize,
    max_body_temperature: usize,

    strength: u8,
    agility: u8,
    endurance: u8,
    metabolism: u8,
    willpower: u8,
    focus: u8,
    bravery: u8,
    intelligence: u8,
    aggressive: u8,

    age: u64,
    max_age: Option<u64>,

    undead: bool,
    opposed_to_life: bool,
}
