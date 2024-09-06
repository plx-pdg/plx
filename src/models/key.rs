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
    Interrogation,
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
            Key::N => "Next block",
            Key::P => "Previous block",
            Key::E => "Edit exercise",
            Key::Enter => "Enter to continue",
            Key::Esc => "Go back",
            Key::Interrogation => "View help",
        }
    }

    /// Define a key name, by default the lowercase version of the enum case
    /// but it can also be used to rename special keys like Interrogation -> ?
    pub fn name(&self) -> String {
        match self {
            Key::Interrogation => "?".to_string(),
            _ => format!("{:?}", self).to_lowercase(),
        }
    }

    /// An alternative key, can be empty
    pub fn alt(&self) -> &str {
        match self {
            // Key::Q => "ctrl+c", TODO: use sigint / sigterm signals to handle shortcut
            Key::H => "left",
            Key::J => "down",
            Key::K => "up",
            Key::L => "right",
            _ => "",
        }
    }
}
