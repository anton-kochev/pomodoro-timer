pub struct Timer {}

impl Timer {
    pub fn new() -> Self {
        Timer {}
    }

    pub fn render(&self) {
        ncurses::clear();
        ncurses::addstr("POMODORO TIMER").unwrap();
        ncurses::mv(1, 0);
        ncurses::addstr("Timer is running...").unwrap();
        ncurses::refresh();
    }
}
