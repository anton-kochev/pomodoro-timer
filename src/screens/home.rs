use super::screen::Screen;
use super::timer::Timer;

pub struct Home {
    current_option: usize,
    options: Vec<String>,
}

impl Home {
    pub fn new() -> Self {
        Home {
            current_option: 0,
            options: vec!["Run timer".to_string(), "Configure".to_string()],
        }
    }

    pub fn render(&self) {
        ncurses::clear();
        ncurses::addstr("POMODO.RS").unwrap();
        for (i, option) in self.options.iter().enumerate() {
            ncurses::mv((i as i32) + 1, 0);
            if i == self.current_option {
                ncurses::attron(ncurses::COLOR_PAIR(1));
            }
            ncurses::addstr(option.as_str()).unwrap();
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

    pub fn execute(&self) -> Option<Screen> {
        match self.current_option {
            // Run timer
            0 => Some(Screen::Timer(Timer::new())),
            // Configure
            1 => Some(Screen::Timer(Timer::new())),
            _ => None,
        }
    }
}
