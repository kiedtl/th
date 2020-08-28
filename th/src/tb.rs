use termbox_sys::*;

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

pub fn tb_put_string(
    max_x: i32, max_y: i32,
    col: i32, row: i32,
    str: &str,
    fg: u32, bg: u32,
    wrap: bool
) -> (i32, i32) {
    let mut ccol = col;
    let mut crow = row;
    for c in str.chars() {
        unsafe {
            tb_put_cell(ccol, crow, &RawCell {
                ch: c as u32,
                fg: fg, bg: bg
            });
        }

        if (ccol + 1) == (max_x - 1) {
            if wrap && crow + 1 != max_y {
                crow += 1;
                ccol = col;
            } else {
                let dot = RawCell {
                    ch: '.' as u32,
                    fg: fg, bg: bg,
                };

                // draw some nice ellipses
                unsafe {
                    tb_put_cell(ccol - 2, crow, &dot);
                    tb_put_cell(ccol - 1, crow, &dot);
                    tb_put_cell(ccol - 0, crow, &dot);
                }

                break;
            }
        } else {
            ccol += 1;
        }
    }

    // clear to the end of the line
    let clear_cell = RawCell {
        ch: ' ' as u32,
        fg: 0xffffff, bg: 0x000000,
    };

    for ncol in ccol..max_x {
        unsafe {
            tb_put_cell(ncol, row, &clear_cell);
        }
    }

    (crow + 1, ccol)
}
