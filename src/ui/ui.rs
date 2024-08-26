use std::sync::{Mutex, Weak};

use crate::{core::core::PlxCore, models::ui_state::UiState};

pub struct Ui<'a> {
    core: Weak<Mutex<PlxCore<'a>>>,
}
impl Ui<'_> {
    pub fn new(core: Weak<Mutex<PlxCore>>) -> Ui<'_> {
        Ui { core }
    }
    fn setup(&mut self) {
        println!("Ui Setup...");
    }
    fn teardown(&mut self) {
        println!("Ui Teardown...");
    }
    pub fn loop_forever(&mut self) {
        self.setup();

        loop {
            match self.core.upgrade() {
                Some(core) => {
                    if let Ok(core) = core.lock() {
                        if !self.render(core.get_state()) {
                            break;
                        }
                    }
                }
                None => break,
            }
        }
        self.teardown();
    }
    fn render(&mut self, state: &UiState) -> bool {
        todo!();
        return true;
    }
}
