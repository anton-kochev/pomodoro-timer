use std::io::{stdin, Read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use std::{str, thread};

use ncurses;

use super::config::Config;
use super::screens::home::Home;
use super::screens::screen::Screen;

enum IntervalKind {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

struct TimerResult {
    done: bool,
    minutes: u64,
    seconds: u64,
}

struct Timer {
    started_at: Instant,
    timeout: u64,
}

impl Timer {
    fn run(timeout: u64) -> Self {
        Timer {
            started_at: Instant::now(),
            timeout,
        }
    }

    fn elapsed(&self) -> TimerResult {
        let elapsed_seconds = self.started_at.elapsed().as_secs();
        let done = elapsed_seconds >= self.timeout;
        let (minutes, seconds) = match done {
            false => (
                (self.timeout - elapsed_seconds) / 60,
                (self.timeout - elapsed_seconds) % 60,
            ),
            _ => (0, 0),
        };

        TimerResult {
            done,
            minutes,
            seconds,
        }
    }
}

pub struct PomodoroApp {
    current_page: Screen,
    timer: Option<Timer>,
    config: Config,
}

impl PomodoroApp {
    pub fn new() -> Self {
        PomodoroApp {
            current_page: Screen::Home(Home::new()),
            config: Config::load("config.json"),
            timer: None,
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

    pub fn run(&mut self) {
        ncurses::initscr();

        ncurses::start_color();
        ncurses::init_pair(0, ncurses::COLOR_BLUE, ncurses::COLOR_YELLOW);
        ncurses::init_pair(1, ncurses::COLOR_YELLOW, ncurses::COLOR_BLUE);
        ncurses::keypad(ncurses::stdscr(), true);
        ncurses::noecho();

        loop {
            self.render();

            match ncurses::getch() {
                // Escape
                27 => break,
                // Arrow up
                259 => {
                    self.current_page.previous_option();
                }
                // Arrow down
                258 => {
                    self.current_page.next_option();
                }
                // Enter
                10 => match self.current_page.execute() {
                    Some(page) => {
                        self.current_page = page;
                    }
                    None => {}
                },
                _ => {}
            }
        }

        ncurses::endwin();
    }

    fn render(&self) {
        self.current_page.render();
    }

    // TODO:
    // This is not a responsibility of PomodoroApp
    // Should be moved out to another place ?
    // fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
    //     initscr();

    //     start_color();
    //     init_pair(0, COLOR_BLUE, COLOR_YELLOW);
    //     init_pair(1, COLOR_YELLOW, COLOR_BLUE);

    //     let mut quit = false;

    //     while !quit {
    //         // render TUI
    //         clear();
    //         mv(0, 0);
    //         addstr("POMODORO TIMER").unwrap();
    //         mv(1, 0);
    //         attron(COLOR_PAIR(1));
    //         addstr("Today: 0").unwrap();
    //         attroff(COLOR_PAIR(1));
    //         mv(2, 0);

    //         let TimerResult {
    //             done,
    //             minutes,
    //             seconds,
    //         } = self.timer.as_ref().unwrap().elapsed();
    //         if done {
    //             addstr("Timer completed").unwrap();
    //         } else {
    //             addstr(format!("Timer {}:{}", minutes, seconds).as_str()).unwrap();
    //         }

    //         refresh();

    //         // Handle the input
    //         let key = getch();
    //         match key as u8 as char {
    //             'q' => quit = true,
    //             ' ' => {
    //                 // start-stop
    //             }
    //             _ => {}
    //         }
    //     }

    //     /* Terminate ncurses. */
    //     endwin();

    //     Ok(())
    // }
}
