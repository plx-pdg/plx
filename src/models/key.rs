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
            Key::H => "Move left", 
            Key::J => "Move down",
            Key::K => "Move up",
            Key::L => "Move right",
            Key::N => "Next test",
            Key::P => "Previous test",
            Key::E => "Edit exercise",
            Key::Enter => "Enter to continue",
            Key::Esc => "Go back",
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
