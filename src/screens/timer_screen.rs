use crate::{
    config::Config,
    timer::{Timer, TimerResult},
};

pub struct TimerScreen {
    timer: Timer,
}

impl TimerScreen {
    pub fn new(config: &Config) -> Self {
        TimerScreen {
            timer: Timer::new(config.pomodoro_length * 60),
        }
    }

    pub fn render(&self) {
        ncurses::clear();
        ncurses::addstr("POMODORO TIMER").unwrap();
        let TimerResult {
            minutes, seconds, ..
        } = self.timer.elapsed();

        ncurses::mvprintw(1, 0, TimerScreen::format_timer(minutes, seconds).as_str()).unwrap();
        ncurses::mvprintw(2, 0, "Press SPACE to <stop>...").unwrap();
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
