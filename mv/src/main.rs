use pancurses::*;

fn main() {
    let window = initscr();
    window.printw("sup\n");
    window.refresh();
    window.getch();
    endwin();
}
