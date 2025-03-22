use crate::commands::command::Command;

use super::{home_screen::HomeScreen, timer_screen::TimerScreen};

pub enum Screen {
    Home(HomeScreen),
    Timer(TimerScreen),
}

impl Screen {
    pub fn render(&self) {
        match self {
            Screen::Home(home) => {
                home.render();
            }
            Screen::Timer(timer) => {
                timer.render();
            }
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

    pub fn execute(&self) -> Option<Command> {
        match self {
            Screen::Home(home) => home.execute(),
            Screen::Timer(..) => None,
        }
    }
}
