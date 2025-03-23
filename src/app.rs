use ncurses;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use crate::commands::Command;
use crate::config::Config;
use crate::screens::{
    configuration_screen::ConfigurationScreen, home_screen::HomeScreen, screen::Screen,
    timer_screen::TimerScreen,
};

pub struct PomodoroApp {
    config: Rc<Config>, // Shared ownership
    current_screen: Screen,
    timeout: u64,
}

impl PomodoroApp {
    pub fn new() -> Self {
        PomodoroApp {
            config: Rc::new(Config::initialize()),
            current_screen: Screen::Home(HomeScreen::new()),
            timeout: 50,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        ncurses::initscr();
        ncurses::noecho();
        ncurses::timeout(self.timeout as i32);
        ncurses::keypad(ncurses::stdscr(), true);
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        ncurses::start_color();
        ncurses::init_pair(0, ncurses::COLOR_BLUE, ncurses::COLOR_YELLOW);
        ncurses::init_pair(1, ncurses::COLOR_YELLOW, ncurses::COLOR_BLUE);

        loop {
            self.current_screen.render();

            match ncurses::getch() {
                // Space
                32 => break,
                // Arrow up
                259 => {
                    self.current_screen.previous_option();
                }
                // Arrow down
                258 => {
                    self.current_screen.next_option();
                }
                // Enter
                10 => match self.current_screen.execute() {
                    Some(command) => match command {
                        Command::Configure => {
                            self.current_screen = Screen::Configuration(ConfigurationScreen::new(
                                self.config.clone(),
                            ));
                        }
                        Command::Home => {
                            self.current_screen = Screen::Home(HomeScreen::new());
                        }
                        Command::StartTimer => {
                            self.current_screen =
                                Screen::Timer(TimerScreen::new(self.config.clone()));
                        }
                        Command::StopTimer => {
                            self.current_screen = Screen::Home(HomeScreen::new());
                        }
                    },
                    None => {}
                },
                _ => {}
            }

            thread::sleep(Duration::from_millis(self.timeout));
        }

        ncurses::endwin();

        Ok(())
    }
}
