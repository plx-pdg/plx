use strum::{EnumIter, IntoStaticStr};

#[derive(Debug, PartialEq, Eq, EnumIter, IntoStaticStr)]
pub enum Key {
    Q,
    R,
    H,
    J,
    K,
    L,
    N,
    P,
    E,
    Enter,
    Esc,
}

impl Key {
    /// Get the human description of the key
    pub fn describe(&self) -> &str {
        match self {
            Key::Q => "Quit the TUI",
            Key::R => "Resume progress to latest or next exo",
            Key::H => "", //TODO: fill those
            Key::J => "",
            Key::K => "",
            Key::L => "",
            Key::N => "",
            Key::P => "",
            Key::E => "",
            Key::Enter => "",
            Key::Esc => "",
        }
    }
    /// An alternative key, can be empty
    pub fn alt(&self) -> &str {
        match self {
            Key::Q => "ctrl+c",
            Key::H => "left",
            Key::J => "down",
            Key::K => "up",
            Key::L => "right",
            _ => "",
        }
    }
}
