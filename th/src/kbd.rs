use lib::dirs::*;
use lib::math::*;
use crate::state::*;
use termbox_sys::*;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum KeybindingAction {
    LevelUp,
    LevelDown,
    Move(Direction),
    Save
    Quit,
}

#[derive(Copy, Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
pub enum EventType {
    Mouse(i32, i32), // x, y
    Resize(i32, i32), // w, h
    Character(char),
    Key(u16), // TODO: key enum
}

impl EventType {
    pub fn from_rawevent(ev: &RawEvent) -> Result<EventType, String> {
        match ev.etype {
            TB_EVENT_KEY => {
                if ev.key > 0 {
                    Ok(EventType::Key(ev.key))
                } else if ev.ch > 0 {
                    let ch = std::char::from_u32(ev.ch).unwrap();
                    Ok(EventType::Character(ch))
                } else {
                    Err("event type is TB_EVENT_KEY; but both key and ch are null"
                        .to_string())
                }
            },
            TB_EVENT_RESIZE => Ok(EventType::Resize(ev.w, ev.h)),
            TB_EVENT_MOUSE => Ok(EventType::Mouse(ev.x, ev.y)),
            _ => Err(format!("invalid event type: {}", ev.etype)),
        }
    }
}

pub struct Keybinding {
    trigger: EventType,
    action: KeybindingAction,
}

pub struct Keybindings(Vec<Keybinding>);

impl Keybindings {
    // default keybindings
    pub fn new() -> Keybindings {
        let bindings = vec![
            Keybinding {
                trigger: EventType::Character('h'),
                action: KeybindingAction::Move(Direction::West),
            },

            Keybinding {
                trigger: EventType::Character('j'),
                action: KeybindingAction::Move(Direction::South),
            },

            Keybinding {
                trigger: EventType::Character('k'),
                action: KeybindingAction::Move(Direction::North),
            },

            Keybinding {
                trigger: EventType::Character('l'),
                action: KeybindingAction::Move(Direction::East),
            },

            Keybinding {
                trigger: EventType::Character('y'),
                action: KeybindingAction::Move(Direction::NorthWest),
            },

            Keybinding {
                trigger: EventType::Character('u'),
                action: KeybindingAction::Move(Direction::NorthEast),
            },

            Keybinding {
                trigger: EventType::Character('b'),
                action: KeybindingAction::Move(Direction::SouthWest),
            },

            Keybinding {
                trigger: EventType::Character('n'),
                action: KeybindingAction::Move(Direction::SouthEast),
            },

            // save
            Keybinding {
                trigger: EventType::Key(TB_KEY_CTRL_S),
                action: KeybindingAction::Save,
            },

            // save and quit
            Keybinding {
                trigger: EventType::Key(TB_KEY_CTRL_C),
                action: KeybindingAction::Quit,
            },

            Keybinding {
                trigger: EventType::Key(TB_KEY_CTRL_Q),
                action: KeybindingAction::Quit,
            },
        ];

        Keybindings(bindings)
    }

    pub fn as_table(&self) -> HashMap<EventType, KeybindingAction> {
        let mut table = HashMap::new();
        for kbd in &self.0 {
            if !table.contains_key(&kbd.trigger) {
                table.entry(kbd.trigger).or_insert(kbd.action);
            } else {
                *table.get_mut(&kbd.trigger).unwrap() = kbd.action;
            }
        }
        table
    }
}
