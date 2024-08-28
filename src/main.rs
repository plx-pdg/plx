pub mod core;
pub mod models;
pub mod ui;
use core::core::PlxCore;
use std::sync::{Arc, Mutex};
use ui::ui::Ui;
fn main() {
    let core = PlxCore::new();
    match core {
        Some(core) => {
            let mut ui = Ui::new(Arc::new(Mutex::new(core)));
            match ui.loop_forever() {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err),
            }
        }
        None => {
            eprintln!("Couldn't init Core");
        }
    }
}
