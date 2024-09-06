use crate::app::app::App;
use simplelog::*;
use std::fs::File;
pub mod app;
pub mod core;
pub mod models;
pub mod ui;

fn main() {
    WriteLogger::init(
        LevelFilter::Trace,
        Config::default(),
        File::create("debug.log").expect("Failed to create log file"),
    )
    .expect("Failed to initialize WriteLogger");
    match App::new() {
        Ok(app) => app.run_forever(),
        Err(err) => {
            eprintln!("Error starting plx {err}");
        }
    }
}
