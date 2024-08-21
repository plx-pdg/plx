use crate::core::{Core, Key, UIState};

struct PlxState {
    ui_state: UIState,
}

#[derive(Clone)]
pub struct PlxExercise {
    pub title: String,
    pub prompt: String,
}

#[derive(Clone)]
pub struct PlxSubject {
    pub title: String,
    path: std::path::PathBuf,
    exercises: Vec<PlxExercise>,
}
pub struct PlxCore {
    state: PlxState,
    subjects: Vec<PlxSubject>,
}
impl PlxSubject {
    fn new() -> Self {
        Self {
            title: "Subject".to_string(),
            path: "/home".to_string().into(),
            exercises: vec![PlxExercise::new(), PlxExercise::new()],
        }
    }
}
impl PlxExercise {
    fn new() -> Self {
        Self {
            title: "Titre".to_string(),
            prompt: "Prompt".to_string(),
        }
    }
}
impl PlxState {
    fn new() -> Self {
        Self {
            ui_state: UIState::Starting,
        }
    }
    fn new_type(&self, new_type: UIState) -> Self {
        Self {
            ui_state: new_type,
            ..*self
        }
    }
}

impl PlxCore {
    pub fn new() -> Self {
        Self {
            subjects: vec![PlxSubject::new(), PlxSubject::new(), PlxSubject::new()],
            state: PlxState::new(),
        }
    }
    fn update_state(&mut self, key: Key) {
        match key {
            Key::J => self.on_j(),
            Key::K => self.on_k(),
            Key::L => self.on_l(),
            Key::Q => self.on_q(),
            Key::H => self.on_h(),
            Key::Enter => self.on_enter(),
        }
    }
    fn on_enter(&mut self) {
        match self.state.ui_state {
            UIState::Starting => self.state = self.state.new_type(UIState::SelectingSubject(0)),
            UIState::Quit => (),
            UIState::SelectingSubject(index) => {
                self.state = self.state.new_type(UIState::SelectingExercise(index, 0))
            }
            UIState::SelectingExercise(_subject_index, _exercice_index) => (),
        }
    }
    fn on_h(&mut self) {
        match self.state.ui_state {
            UIState::Starting => (),
            UIState::Quit => (),
            UIState::SelectingSubject(_index) => {
                self.state = self.state.new_type(UIState::Starting)
            }
            UIState::SelectingExercise(subject_index, _exercice_index) => {
                self.state = self
                    .state
                    .new_type(UIState::SelectingSubject(subject_index))
            }
        }
    }
    fn on_l(&mut self) {
        match self.state.ui_state {
            UIState::Starting => self.state = self.state.new_type(UIState::SelectingSubject(0)),
            UIState::Quit => (),
            UIState::SelectingSubject(index) => {
                self.state = self.state.new_type(UIState::SelectingExercise(index, 0))
            }
            UIState::SelectingExercise(_subject_index, _exercice_index) => (),
        }
    }
    fn on_k(&mut self) {
        match self.state.ui_state {
            UIState::Starting => (),
            UIState::Quit => (),
            UIState::SelectingSubject(index) => {
                if index > 0 {
                    self.state = self.state.new_type(UIState::SelectingSubject(index - 1))
                }
            }
            UIState::SelectingExercise(subject_index, exercice_index) => {
                if exercice_index > 0 {
                    self.state = self.state.new_type(UIState::SelectingExercise(
                        subject_index,
                        exercice_index - 1,
                    ))
                }
            }
        }
    }
    fn on_q(&mut self) {
        self.state = self.state.new_type(UIState::Quit)
    }

    fn on_j(&mut self) {
        match self.state.ui_state {
            UIState::Starting => (),
            UIState::Quit => (),
            UIState::SelectingSubject(index) => {
                if index + 1 < self.subjects.len() {
                    self.state = self.state.new_type(UIState::SelectingSubject(index + 1))
                }
            }
            UIState::SelectingExercise(subject_index, exercice_index) => {
                if exercice_index + 1 < self.subjects[subject_index].exercises.len() {
                    self.state = self.state.new_type(UIState::SelectingExercise(
                        subject_index,
                        exercice_index + 1,
                    ))
                }
            }
        }
    }
}

impl Core for PlxCore {
    fn on_click(&mut self, key: Key) {
        self.update_state(key);
    }
    fn get_subjects(&self) -> (Vec<PlxSubject>, usize) {
        match self.state.ui_state {
            UIState::Starting => (vec![], 0),
            UIState::SelectingSubject(index) => (self.subjects.clone(), index),
            UIState::SelectingExercise(index, _) => (self.subjects.clone(), index),
            UIState::Quit => (vec![], 0),
        }
    }
    fn get_exercises(&self) -> (Option<&Vec<PlxExercise>>, usize) {
        match self.state.ui_state {
            UIState::Starting => (None, 0),
            UIState::SelectingSubject(index) => (Some(&self.subjects[index].exercises), 0),
            UIState::SelectingExercise(index, ex_index) => {
                (Some(&self.subjects[index].exercises), ex_index)
            }
            UIState::Quit => (None, 0),
        }
    }

    fn get_current_exercise(&self) -> Option<&PlxExercise> {
        match self.state.ui_state {
            UIState::Starting => None,
            UIState::SelectingSubject(index) => self.subjects[index].exercises.first(),
            UIState::SelectingExercise(index, ex_index) => {
                Some(&self.subjects[index].exercises[ex_index])
            }
            UIState::Quit => None,
        }
    }
    fn quit(&self) -> bool {
        self.state.ui_state == UIState::Quit
    }

    fn get_state(&self) -> &UIState {
        &self.state.ui_state
    }
}
