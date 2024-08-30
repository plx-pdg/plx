pub mod core;
pub mod models;
pub mod ui;
use core::app::App;

fn main() {
    let app = App::new();
    if app.is_none() {
        eprintln!("Couldn't init App");
    }
}
