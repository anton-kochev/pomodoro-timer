use std::rc::Rc;

use crate::commands::Command;
use crate::config::Config;

pub struct ConfigurationScreen {
    config: Rc<Config>,
    current_option: usize,
}

impl ConfigurationScreen {
    pub fn new(config: Rc<Config>) -> Self {
        ConfigurationScreen {
            config,
            current_option: 0,
        }
    }

    pub fn render(&self) {
        ncurses::clear();
        ncurses::addstr("POMODO.RS").unwrap();
        for (i, option) in self.get_options().iter().enumerate() {
            if i == self.current_option {
                ncurses::attron(ncurses::COLOR_PAIR(1));
            }
            ncurses::mvprintw(i as i32 + 1, 0, option.as_str()).unwrap();
            if i == self.current_option {
                ncurses::attroff(ncurses::COLOR_PAIR(1));
            }
        }
        ncurses::refresh();
    }

    fn get_options(&self) -> Vec<String> {
        vec![
            format!("Pomodoro: {}:00", self.config.pomodoro_length),
            format!("Short break: {}:00", self.config.short_break_length),
            format!("Long break: {}:00", self.config.long_break_length),
        ]
    }

    pub fn execute(&self) -> Option<Command> {
        match self.current_option {
            _ => Some(Command::Home),
        }
    }

    pub fn next_option(&mut self) {
        let num_of_options = self.get_options().len();

        self.current_option = (self.current_option + 1) % num_of_options;
    }

    pub fn previous_option(&mut self) {
        let num_of_options = self.get_options().len();

        self.current_option = (self.current_option + num_of_options - 1) % num_of_options;
    }
}
