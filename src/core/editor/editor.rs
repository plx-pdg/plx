use std::env;

// Collects the EDITOR set using the env variable
pub fn get_default_editor() -> Option<String> {
    match env::var("EDITOR") {
        Ok(editor) => Some(editor.into()),
        Err(_) => None,
    }
}
