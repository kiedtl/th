use lib::dirs::*;
use lib::math::*;
use crate::state::*;
use termbox_sys::*;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum KeybindingAction {
    LevelUp,
    LevelDown,
    CursorMove(Direction),
    CursorMoveFast(Direction),
    Quit,
}

impl KeybindingAction {
    pub fn execute(&self, st: &mut State) -> Result<(), String> {
        match self {
            KeybindingAction::Quit => {
                unsafe { tb_shutdown(); }
                std::process::exit(0);
            },
            KeybindingAction::CursorMove(d) => {
                match d {
                    Direction::West => st.current_x = st.current_x.saturating_sub(1),
                    Direction::South => st.current_y = clamp(st.current_y + 1,
                        0, st.dungeon.levels[st.level].height - 1),
                    Direction::North => st.current_y = st.current_y.saturating_sub(1),
                    Direction::East => st.current_x = clamp(st.current_x + 1, 0,
                        st.dungeon.levels[st.level].width - 1),
                    _ => (),
                }
            },

            KeybindingAction::CursorMoveFast(d) => {
                match d {
                    Direction::West => st.current_x = st.current_x.saturating_sub(8),
                    Direction::South => st.current_y = clamp(st.current_y + 8, 0,
                        st.dungeon.levels[st.level].height - 1),
                    Direction::North => st.current_y = st.current_y.saturating_sub(8),
                    Direction::East => st.current_x = clamp(st.current_x + 8, 0,
                        st.dungeon.levels[st.level].width - 1),
                    _ => (),
                }
            }

            KeybindingAction::LevelDown => st.level = clamp(st.level + 1, 0,
                st.dungeon.levels.len() - 1),
            KeybindingAction::LevelUp => st.level = st.level.saturating_sub(1),
        }

        Ok(())
    }
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
                action: KeybindingAction::CursorMove(Direction::West),
            },

            Keybinding {
                trigger: EventType::Character('j'),
                action: KeybindingAction::CursorMove(Direction::South),
            },

            Keybinding {
                trigger: EventType::Character('k'),
                action: KeybindingAction::CursorMove(Direction::North),
            },

            Keybinding {
                trigger: EventType::Character('l'),
                action: KeybindingAction::CursorMove(Direction::East),
            },

            Keybinding {
                trigger: EventType::Character('y'),
                action: KeybindingAction::CursorMove(Direction::NorthWest),
            },

            Keybinding {
                trigger: EventType::Character('u'),
                action: KeybindingAction::CursorMove(Direction::NorthEast),
            },

            Keybinding {
                trigger: EventType::Character('b'),
                action: KeybindingAction::CursorMove(Direction::SouthWest),
            },

            Keybinding {
                trigger: EventType::Character('n'),
                action: KeybindingAction::CursorMove(Direction::SouthEast),
            },

            // move keys quickly
            Keybinding {
                trigger: EventType::Character('H'),
                action: KeybindingAction::CursorMoveFast(Direction::West),
            },

            Keybinding {
                trigger: EventType::Character('J'),
                action: KeybindingAction::CursorMoveFast(Direction::South),
            },

            Keybinding {
                trigger: EventType::Character('K'),
                action: KeybindingAction::CursorMoveFast(Direction::North),
            },

            Keybinding {
                trigger: EventType::Character('L'),
                action: KeybindingAction::CursorMoveFast(Direction::East),
            },

            Keybinding {
                trigger: EventType::Character('Y'),
                action: KeybindingAction::CursorMoveFast(Direction::NorthWest),
            },

            Keybinding {
                trigger: EventType::Character('U'),
                action: KeybindingAction::CursorMoveFast(Direction::NorthEast),
            },

            Keybinding {
                trigger: EventType::Character('B'),
                action: KeybindingAction::CursorMoveFast(Direction::SouthWest),
            },

            Keybinding {
                trigger: EventType::Character('N'),
                action: KeybindingAction::CursorMoveFast(Direction::SouthEast),
            },

            // level down
            Keybinding {
                trigger: EventType::Character('>'),
                action: KeybindingAction::LevelDown,
            },

            Keybinding {
                trigger: EventType::Character('<'),
                action: KeybindingAction::LevelUp,
            },

            // quit
            Keybinding {
                trigger: EventType::Key(TB_KEY_CTRL_C),
                action: KeybindingAction::Quit,
            },

            Keybinding {
                trigger: EventType::Character('q'),
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

    pub fn handle_ev(&self, r_ev: &RawEvent, st: &mut State) -> Result<(), String> {
        let ev = EventType::from_rawevent(r_ev)?;
        let kbds = self.as_table();

        if kbds.contains_key(&ev) {
            kbds[&ev].execute(st)?;
        }

        Ok(())
    }
}
