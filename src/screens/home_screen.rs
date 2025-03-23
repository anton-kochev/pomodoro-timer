use crate::commands::Command;

pub struct HomeScreen {
    current_option: usize,
    options: Vec<String>,
}

impl HomeScreen {
    pub fn new() -> Self {
        HomeScreen {
            current_option: 0,
            options: vec!["Run timer".to_string(), "Configure".to_string()],
        }
    }

    pub fn render(&self) {
        ncurses::clear();
        ncurses::addstr("POMODO.RS").unwrap();
        for (i, option) in self.options.iter().enumerate() {
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

    pub fn next_option(&mut self) {
        self.current_option = (self.current_option + 1) % self.options.len();
    }

    pub fn previous_option(&mut self) {
        self.current_option = (self.current_option + self.options.len() - 1) % self.options.len();
    }

    pub fn execute(&self) -> Option<Command> {
        match self.current_option {
            // Run timer
            0 => Some(Command::StartTimer),
            // Configure
            1 => Some(Command::Configure),
            _ => None,
        }
    }
}
