use std::env;

pub fn get_default_editor() -> Option<String> {
    match env::var("EDITOR") {
        Ok(editor) => Some(editor.into()),
        Err(_) => None,
    }
}
