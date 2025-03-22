use ncurses;
use std::thread;
use std::time::Duration;

use crate::commands::command::Command;
use crate::screens::home_screen::HomeScreen;

use super::config::Config;
use super::screens::{screen::Screen, timer_screen::TimerScreen};

pub struct PomodoroApp {
    config: Config,
    current_screen: Screen,
    timeout: u64,
}

impl PomodoroApp {
    pub fn new() -> Self {
        PomodoroApp {
            current_screen: Screen::Home(HomeScreen::new()),
            config: Config::load("config.json"),
            timeout: 50,
        }
    }

    // pub fn run_long_break(&mut self) {
    //     self.run_timer(IntervalKind::LongBreak);
    // }

    // pub fn run_pomodoro(&mut self) {
    //     self.run_timer(IntervalKind::Pomodoro);
    // }

    // pub fn run_short_break(&mut self) {
    //     self.run_timer(IntervalKind::ShortBreak);
    // }

    // pub fn stop_timer(&mut self) {
    //     self.timer = None;
    // }

    // fn run_timer(&mut self, kind: IntervalKind) {
    //     let timeout = match kind {
    //         IntervalKind::LongBreak => self.config.long_break_length,
    //         IntervalKind::Pomodoro => self.config.pomodoro_length,
    //         IntervalKind::ShortBreak => self.config.short_break_length,
    //     };

    //     self.timer = Option::Some(Timer::run(timeout));
    // }

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
                // Escape
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
                        Command::Configure(()) => {}
                        Command::StartTimer(()) => {
                            self.current_screen = Screen::Timer(TimerScreen::new(&self.config));
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
