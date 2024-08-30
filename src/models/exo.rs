use std::fs::ReadDir;

use serde::{Deserialize, Serialize};

use crate::core::{
    file_utils::{file_parser::ParseError, file_utils::list_dir},
    parser::{self},
};

use super::{check::Check, exo_state::ExoState, solution::Solution};

#[derive(Serialize, Deserialize, Debug)]
struct ExoInfo {
    name: String,
    instruction: Option<String>,
    checks: Vec<Check>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExoStateInfo {
    state: ExoState,
    favorite: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Exo {
    name: String,
    instruction: Option<String>,
    state: ExoState,
    files: Vec<std::path::PathBuf>,
    solution: Option<Solution>,
    checks: Vec<Check>,
    favorite: bool,
}
impl Exo {
    pub fn get_main_file(&self) -> Option<&std::path::PathBuf> {
        match self.files.iter().find(|file| {
            if let Some(file_name) = file.file_stem() {
                return file_name == "main";
            }
            return false;
        }) {
            Some(file) => Some(file),
            None => self.files.first(),
        }
    }
}
impl Exo {
    pub fn from_dir(directory: std::path::PathBuf) -> Result<Self, ParseError>
    where
        Self: serde::de::DeserializeOwned,
    {
        let exo_info_file = directory.join("exo.toml");
        let exo_state_file = directory.join(".exo.state");
        if !exo_info_file.exists() {
            return Err(ParseError::FileNotFound(
                exo_info_file.display().to_string(),
            ));
        }
        let exo_info = parser::object_creator::create_from_file::<ExoInfo>(&exo_info_file)?;
        let exo_state = parser::object_creator::create_from_file::<ExoStateInfo>(&exo_state_file)
            .or::<ExoStateInfo>(Ok(ExoStateInfo {
                favorite: false,
                state: ExoState::Todo,
            }))
            .unwrap();
        let files =
            list_dir(&directory).map_err(|err| ParseError::FileDiscoveryFailed(err.to_string()))?;
        let (exo_files, solution_file) = Exo::find_exo_files(files);
        if exo_files.is_empty() {
            return Err(ParseError::NoExoFilesFound(directory));
        }
        let state = exo_state.state;
        let favorite = exo_state.favorite;

        let solution = if let Some(solution_file) = solution_file {
            Some(Solution::new(solution_file))
        } else {
            None
        };

        Ok(Self {
            name: exo_info.name,
            instruction: exo_info.instruction,
            checks: exo_info.checks,
            state,
            files: exo_files,
            favorite,
            solution,
        })
    }

    fn find_exo_files(
        dir_entries: ReadDir,
    ) -> (Vec<std::path::PathBuf>, Option<std::path::PathBuf>) {
        let mut exo_files = Vec::new();
        let mut solution_file = None;
        for entry in dir_entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();
                let file_path_str = file_path.display().to_string();
                let file_extension = file_path
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .or(Some(""))
                    .unwrap();

                if file_path_str.contains(".sol.") {
                    // TODO handle the case where we have multiple solutions ?
                    solution_file = Some(file_path);
                    continue;
                }
                if file_extension == "toml" {
                    continue;
                }
                // TODO maybe make sure we don't mix .c with .java files here ?
                // We need to be careful adding this because .c can be mixed with .cpp, .h,
                // .hpp etc...
                exo_files.push(file_path);
            }
        }
        (exo_files, solution_file)
    }
}

#[cfg(test)]
mod test {
    use crate::models::check::CheckType;

    use super::*;

    #[test]
    fn test_parse_full_intro_basic_args() {
        let file_path = "examples/full/intro/basic-args";

        let expected = Exo {
        name: String::from("Basic arguments usage"),
        instruction: Some(
                String::from(
                    "The 2 first program arguments are the firstname and number of legs of a dog. Print a full sentence about the dog. Make sure there is at least 2 arguments, print an error if not.")
            ),
        state: ExoState::Todo,
        files: vec!["examples/full/intro/basic-args/basic-args.c".into()],
        solution: Some(
            Solution::new(
            "examples/full/intro/basic-args/basic-args.sol.c".into()),
        ),
        checks: vec![
            Check {
                name: String::from("Joe + 5 legs"),
                args: vec![
                    String::from("Joe"),
                    String::from("5"),
                ],
                check_type: CheckType::Output,
            },
            Check {
                name: String::from("No arg -> error"),
                args: vec![],
                check_type: CheckType::Output,
            },
            Check {
                name: String::from("One arg -> error"),
                args: vec![
                    String::from("Joe"),
                ],
                check_type: CheckType::Output,
            },
        ],
        favorite: false,
    };
        assert_eq!(
            expected,
            Exo::from_dir(file_path.into()).expect("Couldn't parse file")
        );
    }
}
