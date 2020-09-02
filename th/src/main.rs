mod coord;
mod display;
mod kbd;
mod message;
mod state;
mod tb;
mod tick;

use crate::display::*;
use crate::kbd::*;
use crate::state::*;
use crate::tb::*;
use lib::info_files::*;
use lib::mob::*;
use lib::material::*;
use termbox_sys::*;
use std::collections::HashMap;

fn main() {
    // set a custom panic handler that calls tb_shutdown
    // before printing anything
    std::panic::set_hook(Box::new(|panic_info| {
        unsafe { tb_shutdown(); }
        println!("aborting due to fatal error (see below):");
        println!("{}", panic_info);
        println!("stack backtrace:");
        println!("{:?}", backtrace::Backtrace::new());
        println!("please report this issue upstream at {}.",
            "https://github.com/kiedtl/th");
    }));

    // check arguments
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("{}: need dungeon file.", args[0]);
        eprintln!("usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let materials: HashMap<String, MaterialInfo> =
        load_info_files("../dat/mats/").unwrap();

    let mobs: HashMap<String, MobTemplate> =
        load_info_files("../dat/mobs/").unwrap();

    // try to load map
    let mut st = match State::from_file(&args[1]) {
        Ok(s) => s,
        Err(e) => {
            println!("{}: \"{}\": {}", args[0], args[1], e);
            std::process::exit(1);
        },
    };

    tick::player_tick(&mut st);

    // keybindings
    let kbd = Keybindings::new();
    let keybinds = kbd.as_table();

    let mut rng = rand::thread_rng();

    // termbox display
    let display = Display::new(DisplayMode::Console, &materials);

    display.draw(&st);
    display.present();

    // main loop
    loop {
        let mut raw_ev = RawEvent::new();
        let t = unsafe { tb_poll_event(&mut raw_ev) };

        if t == -1 {
            display.close();
            eprintln!("error: fatal termbox error");
            // TODO: save
            std::process::exit(1);
        }

        if t == (TB_EVENT_KEY as i32) {
            let ev = EventType::from_rawevent(&raw_ev)
                .unwrap();
            match ev {
                EventType::Character(_)
                | EventType::Key(_) => {
                    if !keybinds.contains_key(&ev) {
                        continue;
                    }

                    let action = keybinds[&ev];
                    match action {
                        KeybindingAction::Save => {
                            st.save_to_file().unwrap();
                        },
                        KeybindingAction::Quit => {
                            st.save_to_file().unwrap();
                            break; // close display and exit
                        },
                        _ => st.handle_action(action),
                    }
                },

                // if the event type is EventType::Resize,
                // then just ignore it as the screen will be
                // redrawn later anyway
                _ => (),
            }

            tick::mobs_tick(&mut st, &mobs, &mut rng);
            tick::player_tick(&mut st);
            display.draw(&st);
            display.present();
        } else if t == (TB_EVENT_RESIZE as i32) {
            display.draw(&st);
            display.present();
        }
    }

    #[allow(unreachable_code)]
    display.close();
}
