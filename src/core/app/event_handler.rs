use crate::models::key::Key;

use super::app::App;

impl App {
    pub(super) fn on_key_press(&mut self, key: Key) {
        match key {
            Key::Q => self.on_q(),
            Key::R => self.on_r(),
            Key::H => self.on_h(),
            Key::J => self.on_j(),
            Key::K => todo!(),
            Key::L => self.on_l(),
            Key::N => todo!(),
            Key::P => todo!(),
            Key::E => todo!(),
            Key::Enter => todo!(),
            Key::Esc => todo!(),
        }
    }
}
