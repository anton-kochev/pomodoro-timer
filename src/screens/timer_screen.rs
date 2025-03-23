use std::rc::Rc;

use crate::commands::Command;
use crate::config::Config;
use crate::timer::{Timer, TimerResult};

pub struct TimerScreen {
    config: Rc<Config>,
    timer: Timer,
}

impl TimerScreen {
    pub fn new(config: Rc<Config>) -> Self {
        let duration = config.pomodoro_length;

        TimerScreen {
            config,
            timer: Timer::new(duration * 60),
        }
    }

    pub fn execute(&self) -> Option<Command> {
        Some(Command::StopTimer)
    }

    pub fn render(&self) {
        ncurses::clear();
        ncurses::addstr("POMODORO TIMER").unwrap();
        let TimerResult {
            minutes, seconds, ..
        } = self.timer.elapsed();

        ncurses::mvprintw(1, 0, TimerScreen::format_timer(minutes, seconds).as_str()).unwrap();
        ncurses::mvprintw(2, 0, "Press ENTER to <stop>...").unwrap();
        ncurses::refresh();
    }

    fn format_timer(minutes: u64, seconds: u64) -> String {
        format!(
            "{}:{}",
            TimerScreen::format(minutes),
            TimerScreen::format(seconds)
        )
    }

    fn format(time: u64) -> String {
        match time < 10 {
            true => format!("0{}", time),
            false => format!("{}", time),
        }
    }
}
