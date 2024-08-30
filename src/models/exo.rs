use serde::{Deserialize, Serialize};

use crate::core::{
    file_utils::{
        file_parser::{ParseError, ParseWarning},
        file_utils::list_dir_files,
    },
    parser::{self, from_dir::FromDir},
};

use super::{check::Check, exo_state::ExoState};

#[derive(Serialize, Deserialize, Debug)]
struct ExoInfo {
    name: String,
    instruction: Option<String>,
    #[serde(default)]
    checks: Vec<Check>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExoStateInfo {
    state: ExoState,
    favorite: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Exo {
    pub name: String,
    pub instruction: Option<String>,
    pub state: ExoState,
    pub files: Vec<std::path::PathBuf>,
    pub solutions: Vec<std::path::PathBuf>,
    pub checks: Vec<Check>,
    pub favorite: bool,
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
impl FromDir for Exo {
    ///
    /// Tries to build an exo from dir
    /// Returns Ok if we were able to get the exo info and at least 1 exo file
    /// else Error
    ///
    fn from_dir(
        dir: &std::path::PathBuf,
    ) -> Result<(Self, Vec<ParseWarning>), (ParseError, Vec<ParseWarning>)> {
        // Get the exo info and the state if it exists.
        let mut warnings = Vec::new();
        let exo_info_file = dir.join("exo.toml");
        let exo_state_file = dir.join(".exo-state.toml");
        let exo_info = parser::object_creator::create_from_file::<ExoInfo>(&exo_info_file)
            .map_err(|err| (err, vec![]))?;

        // If the exo hasn't been started, the state file won't exist
        let exo_state = parser::object_creator::create_from_file::<ExoStateInfo>(&exo_state_file)
            .or::<ExoStateInfo>(Ok(ExoStateInfo {
                favorite: false,
                state: ExoState::Todo,
            }))
            .unwrap(); // this is safe with or fn

        // Get all the dir files and find the exo and solution files
        let files = list_dir_files(&dir)
            .map_err(|err| (ParseError::FileDiscoveryFailed(err.to_string()), vec![]))?;
        let (exo_files, solution_file) = Exo::find_exo_files(files, &mut warnings);

        if exo_files.is_empty() {
            return Err((ParseError::NoExoFilesFound(dir.to_path_buf()), vec![]));
        }

        let solution = if let Some(solution_file) = solution_file {
            Some(Solution::new(solution_file))
        } else {
            warnings.push(ParseWarning::ExoSolutionNotFound(format!(
                "No solution found in {:?}",
                dir
            )));
            None
        };

        Ok((
            Self {
                name: exo_info.name,
                instruction: exo_info.instruction,
                checks: exo_info.checks,
                state: exo_state.state,
                files: exo_files,
                favorite: exo_state.favorite,
                solution,
            },
            warnings,
        ))
    }
}
impl Exo {
    /// Finds exo and solution from a bunch of folder files
    fn find_exo_files(
        dir_entries: Vec<std::path::PathBuf>,
        warnings: &mut Vec<ParseWarning>,
    ) -> (Vec<std::path::PathBuf>, Option<std::path::PathBuf>) {
        let mut exo_files = Vec::new();
        let mut solution_file = None;
        for file_path in dir_entries {
            let file_path_str = file_path.display().to_string();
            let file_extension = file_path
                .extension()
                .and_then(|extension| extension.to_str())
                .or(Some(""))
                .unwrap();

            if file_path_str.contains(".sol.") {
                if solution_file.is_some() {
                    warnings.push(ParseWarning::MultipleSolutionsFound(format!(
                        "Keeping {:?}",
                        solution_file
                    )));
                    continue;
                }
                solution_file = Some(file_path);
                continue;
            }
            // Ignore our files
            if file_extension == "toml" {
                continue;
            }
            // TODO maybe make sure we don't mix .c with .java files here ?
            // We need to be careful adding this because .c can be mixed with .cpp, .h,
            // .hpp etc...
            exo_files.push(file_path);
        }
        (exo_files, solution_file)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::models::check::CheckType;

    use super::*;

    #[test]
    fn test_parse_full_intro_basic_args() {
        let file_path = "examples/mock-plx-project/intro/basic-args";

        let expected = Exo {
        name: String::from("Basic arguments usage"),
        instruction: Some(
                String::from(
                    "The 2 first program arguments are the firstname and number of legs of a dog. Print a full sentence about the dog. Make sure there is at least 2 arguments, print an error if not.")
            ),
        state: ExoState::Todo,
        files: vec![
            std::path::PathBuf::from_str(file_path)
                .unwrap()
                .join("main.c")],

        solution: Some(
            Solution::new(
            std::path::PathBuf::from_str(file_path)
                .unwrap()
                .join("main.sol.c")
            ),
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
            Exo::from_dir(&(file_path.into()))
                .expect("Couldn't parse file")
                .0
        );
    }
    #[test]
    fn test_exo_done() {
        let file_path = "examples/mock-plx-project/mock-skill/exo-done";
        let (exo, _warnings) = Exo::from_dir(&(file_path.into())).unwrap();
        let expected = Exo {
            name: String::from("Exo Done"),
            instruction: None,
            checks: vec![],
            files: vec![std::path::PathBuf::from_str(file_path)
                .unwrap()
                .join("main.c")],
            favorite: false,
            state: ExoState::Done,
            solution: None,
        };
        assert_eq!(expected, exo);
        println!("{:#?}", exo);
    }
    #[test]
    fn test_exo_favorite() {
        let file_path = "examples/mock-plx-project/mock-skill/exo-favorite";
        let (exo, _warnings) = Exo::from_dir(&(file_path.into())).unwrap();
        let expected = Exo {
            name: String::from("Favorite Exercise"),
            instruction: None,
            checks: vec![],
            files: vec![std::path::PathBuf::from_str(file_path)
                .unwrap()
                .join("main.c")],
            favorite: true,
            state: ExoState::Todo,
            solution: None,
        };
        assert_eq!(expected, exo);
        println!("{:#?}", exo);
    }

    #[test]
    fn test_exo_in_progress() {
        let file_path = "examples/mock-plx-project/mock-skill/exo-in-progress";
        let (exo, _warnings) = Exo::from_dir(&(file_path.into())).unwrap();
        let expected = Exo {
            name: String::from("In Progress"),
            instruction: None,
            checks: vec![],
            files: vec![std::path::PathBuf::from_str(file_path)
                .unwrap()
                .join("main.c")],
            favorite: false,
            state: ExoState::InProgress,
            solution: None,
        };
        assert_eq!(expected, exo);
        println!("{:#?}", exo);
    }

    #[test]
    fn test_exo_multiple_sols() {
        let file_path = "examples/mock-plx-project/mock-skill/multiple-sols";
        let sol_file = std::path::PathBuf::from_str(file_path)
            .unwrap()
            .join("solution1.sol.c");
        let (exo, warnings) = Exo::from_dir(&(file_path.into())).unwrap();
        let expected = Exo {
            name: String::from("Multiple Sols"),
            instruction: None,
            checks: vec![],
            files: vec![std::path::PathBuf::from_str(file_path)
                .unwrap()
                .join("main.c")],
            favorite: false,
            state: ExoState::Todo,
            solution: Some(Solution {
                path: sol_file.clone(),
            }),
        };
        assert_eq!(expected, exo);
        assert_eq!(warnings.len(), 1);
        assert!(matches!(
            warnings[0],
            ParseWarning::MultipleSolutionsFound(_)
        ));
    }
    #[test]
    fn test_no_exo_info() {
        let file_path = "examples/mock-plx-project/mock-skill/no-exo-info";
        let ret = Exo::from_dir(&(file_path.into()));
        assert!(ret.is_err());

        let err = match ret {
            Ok(_) => panic!("Exo can't be constructed with no exo info"),
            Err((error, _warnings)) => error,
        };
        assert!(matches!(err, ParseError::ReadFileError(_)));
    }
    #[test]
    fn test_no_files() {
        let file_path = "examples/mock-plx-project/mock-skill/no-files";
        let ret = Exo::from_dir(&(file_path.into()));
        assert!(ret.is_err());

        let err = match ret {
            Ok(_) => panic!("Exo can't be constructed with no files"),
            Err((error, _warnings)) => error,
        };
        assert!(matches!(err, ParseError::NoExoFilesFound(_)));
    }
    #[test]
    fn test_no_solution() {
        let file_path = "examples/mock-plx-project/mock-skill/no-sol";
        let (exo, warnings) = Exo::from_dir(&(file_path.into())).unwrap();
        let expected = Exo {
            name: String::from("No Sol"),
            instruction: None,
            checks: vec![],
            files: vec![std::path::PathBuf::from_str(file_path)
                .unwrap()
                .join("main.c")],
            favorite: false,
            state: ExoState::Todo,
            solution: None,
        };
        assert_eq!(expected, exo);
        assert_eq!(warnings.len(), 1);
        assert!(matches!(warnings[0], ParseWarning::ExoSolutionNotFound(_)));
    }
}
