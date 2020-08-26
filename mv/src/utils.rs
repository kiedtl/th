use termbox_sys::*;

#[inline]
pub fn setup_tb() {
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
