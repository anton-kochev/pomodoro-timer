mod app;
mod commands;
mod config;
mod screens;
mod timer;

use app::PomodoroApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    PomodoroApp::new().run()
}
