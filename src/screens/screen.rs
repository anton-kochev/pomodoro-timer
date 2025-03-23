use crate::commands::Command;

use super::{
    configuration_screen::ConfigurationScreen, home_screen::HomeScreen, timer_screen::TimerScreen,
};

pub enum Screen {
    Configuration(ConfigurationScreen),
    Home(HomeScreen),
    Timer(TimerScreen),
}

impl Screen {
    pub fn render(&self) {
        match self {
            Screen::Configuration(screen) => {
                screen.render();
            }
            Screen::Home(screen) => {
                screen.render();
            }
            Screen::Timer(screen) => {
                screen.render();
            }
        }
    }

    pub fn next_option(&mut self) {
        match self {
            Screen::Configuration(screen) => screen.next_option(),
            Screen::Home(screen) => screen.next_option(),
            Screen::Timer(..) => {}
        }
    }

    pub fn previous_option(&mut self) {
        match self {
            Screen::Configuration(screen) => screen.previous_option(),
            Screen::Home(screen) => screen.previous_option(),
            Screen::Timer(..) => {}
        }
    }

    pub fn execute(&self) -> Option<Command> {
        match self {
            Screen::Configuration(screen) => screen.execute(),
            Screen::Home(screen) => screen.execute(),
            Screen::Timer(screen) => screen.execute(),
        }
    }
}
