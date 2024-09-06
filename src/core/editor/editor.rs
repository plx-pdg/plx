use std::env;

// Collects the EDITOR set using the env variable
pub fn get_default_editor() -> Option<String> {
    match env::var("EDITOR") {
        Ok(editor) => {
            let valid_editors: Vec<String> = vec![
                String::from("code"),
                String::from("codium"),
                String::from("clion"),
            ];
            if valid_editors.contains(&editor) {
                Some(editor)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
