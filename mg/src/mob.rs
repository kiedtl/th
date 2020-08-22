use crate::value;

enum MobMovement {
    Sedentary,
    LightlyActive,
    Active,
}

enum MobAlignment {
    Hostile,        // the creatures is hostile to the player
    NeutralHostile, // the creature is neutral but will turn hostile if attacked
    Neutral,        // the creature is neutral, and will flee if attacked
    Friendly,       // the creature is friendly, and will flee if attacked
}

struct MobSpecies {
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

    glyph: char,
    glyph_bg: Option<Color>,
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

    // plants/hydras will have this turned on
    immobile: true,

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

    // what age is the character right now? (in seconds)
    age: Value<u64>,
    max_age: Option<Value<u64>>, // demons don't die of old age

    // most demons will have this
    // controls whether character will be able to summon other demons
    // to their aid
    summoner: bool,

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
    max_in_dungeon: Option<usize>,
    max_in_layer: Option<usize>,
    max_in_level: Option<usize>,

    // TODO list
    // - list of body parts
    // - fire_breathing: bool,
    // - preferred_weapons: Vec<Item>,
    // - preferred_occupations: Vec<Occupation>,
    // - polymorph_into: Vec<String>, // for werewolves
    // - can_cast_spells: bool,
    // - preferred_spells: Vec<Spell>,
}
