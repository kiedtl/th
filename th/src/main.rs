mod coord;
mod display;
mod kbd;
mod message;
mod priority;
mod state;
mod tb;
mod tick;

use crate::display::*;
use crate::kbd::*;
use crate::state::*;
use crate::tb::*;
use lib::dungeon::*;
use lib::info_files::*;
use lib::material::*;
use ron::de::from_reader;
use std::collections::HashMap;
use std::fs::File;
use termbox_sys::*;

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

    // try to load map
    let input_path = &args[1];
    let fmap = match File::open(input_path) {
        Ok(d) => d,
        Err(e) => {
            println!("{}: \"{}\": {}", args[0], input_path, e);
            std::process::exit(1);
        },
    };

    // parse map
    let map: Dungeon = match from_reader(fmap) {
        Ok(x) => x,
        Err(e) => {
            println!("{}: failed to load map: {}", args[0], e);
            std::process::exit(1);
        },
    };

    let materials: HashMap<String, MaterialInfo> =
        load_info_files("../dat/mats/").unwrap();

    // game state
    let mut st: State = State::new(map);
    tick::player_tick(&mut st);

    // keybindings
    let kbd = Keybindings::new();
    let keybinds = kbd.as_table();

    // termbox display
    let mut display = Display::new(DisplayMode::Console, &materials);

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
                        KeybindingAction::Quit => {
                            // TODO: saves
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

            tick::player_tick(&mut st);
            display.draw(&st);
            display.present();
        }
    }

    #[allow(unreachable_code)]
    display.close();
}
