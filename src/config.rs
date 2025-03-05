use serde::Deserialize;
use serde_json;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub pomodoro_length: u16,
    pub short_break_length: u16,
    pub long_break_length: u16,
    pub intervals_before_long_break: u16,
}

impl Config {
    fn new() -> Self {
        Config {
            pomodoro_length: 25,
            short_break_length: 5,
            long_break_length: 15,
            intervals_before_long_break: 4,
        }
    }

    pub fn init(path: &str) -> Self {
        match fs::read(path) {
            Ok(contents) => serde_json::from_slice(&contents).unwrap_or_else(|_| Config::new()),
            Err(..) => Config::new(),
        }
    }
}
