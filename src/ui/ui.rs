use std::sync::Mutex;

use crate::{core::core::PlxCore, models::ui_state::UiState};

pub struct Ui<'a> {
    core: Mutex<PlxCore<'a>>,
}
impl Ui<'_> {
    pub fn new(core: Mutex<PlxCore>) -> Ui<'_> {
        Ui { core }
    }
    fn setup(&mut self) {
        println!("Ui Setup...");
    }
    pub fn loop_forever(&mut self) {
        self.setup();

        loop {
            if let Ok(core) = self.core.lock() {
                match core.get_state() {
                    UiState::StartMenu => println!("Starting..."),
                    UiState::Quit => return,
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
            }
        }
    }
}
