use termbox_sys::*;

#[inline]
pub fn tb_setup() {
    match unsafe { tb_init() } {
        TB_EFAILED_TO_OPEN_TTY => {
            eprintln!("error: could not open terminal");
            std::process::exit(1);
        },
        TB_EUNSUPPORTED_TERMINAL => {
            eprintln!("error: unsupported terminal");
            eprintln!("hint: try using another terminal (such as alacritty)");
            std::process::exit(1);
        },
        TB_EPIPE_TRAP_ERROR => {
            eprintln!("error: could not initialize screen");
            std::process::exit(1);
        },
        _ => (),
    }

    unsafe {
        tb_select_output_mode(TB_OUTPUT_TRUECOLOR);
        tb_set_clear_attributes(TB_WHITE, TB_BLACK);
        tb_clear();
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
