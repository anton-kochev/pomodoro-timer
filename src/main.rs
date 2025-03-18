use std::io::{self, Read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

mod app;
mod config;
mod screens;

use app::PomodoroApp;

fn main() -> io::Result<()> {
    let mut app = PomodoroApp::new();

    app.run();

    Ok(())
}
