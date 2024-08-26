use super::ui_key::UiKey;

#[derive(Debug, PartialEq, Eq)]
pub enum Event {
    KeyPressed(UiKey),
    EditorOpened,
    CouldNotOpenEditor,
}
