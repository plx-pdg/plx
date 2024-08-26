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
        match state {
            UiState::StartMenu => println!("Starting..."),
            UiState::Quit => return false,
            UiState::ChoosingSubject { subject_index } => todo!(),
            UiState::ChoosingExo {
                subject_index,
                exo_index,
            } => todo!(),
            UiState::ExoPromp {
                subject_index,
                exo_index,
                exo,
            } => todo!(),
            UiState::LoadingExo { exo } => todo!(),
            UiState::ExoLoaded { exo } => todo!(),
            UiState::Compiling { exo } => todo!(),
            UiState::CompileError { exo, error } => todo!(),
            UiState::DoingExo { exo, checks } => todo!(),
            UiState::ExoComplete { exo } => todo!(),
            UiState::ShowSolution { exo } => todo!(),
        }
        return true;
    }
}
