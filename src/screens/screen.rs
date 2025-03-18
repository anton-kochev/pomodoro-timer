use super::home::Home;
use super::timer::Timer;

pub enum Screen {
    Home(Home),
    Timer(Timer),
}

impl Screen {
    pub fn render(&self) {
        match self {
            Screen::Home(home) => home.render(),
            Screen::Timer(timer) => timer.render(),
        }
    }

    pub fn next_option(&mut self) {
        match self {
            Screen::Home(home) => home.next_option(),
            Screen::Timer(..) => {}
        }
    }

    pub fn previous_option(&mut self) {
        match self {
            Screen::Home(home) => home.previous_option(),
            Screen::Timer(..) => {}
        }
    }

    pub fn execute(&self) -> Option<Screen> {
        match self {
            Screen::Home(home) => home.execute(),
            Screen::Timer(..) => None,
        }
    }
}
