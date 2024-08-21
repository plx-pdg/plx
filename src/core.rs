use crate::plx_core::{PlxExercise, PlxSubject};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Key {
    Q,
    J,
    K,
    L,
    H,
    Enter,
}
pub trait Core {
    fn on_click(&mut self, key: Key);
    fn get_subjects(&self) -> (Vec<PlxSubject>, usize);
    fn get_exercises(&self) -> (Option<&Vec<PlxExercise>>, usize);
    fn get_current_exercise(&self) -> Option<&PlxExercise>;
    fn quit(&self) -> bool;
}
