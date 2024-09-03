use serde::Deserialize;

use crate::core::{
    file_utils::file_parser::{ParseError, ParseWarning},
    parser::{from_dir::FromDir, object_creator},
};

use super::{constants::COURSE_INFO_FILE, skill::Skill};

#[derive(Debug, PartialEq, Eq)]
pub struct Project {
    name: String,
    skills: Vec<Skill>,
}

#[derive(Deserialize)]
struct ProjectInfo {
    name: String,
    #[serde(rename = "skills")]
    skill_folders: Vec<std::path::PathBuf>,
}

impl FromDir for Project {
    ///
    /// Tries to build a project from dir
    /// Returns Ok if we were able to parse the project info and at least one skill
    /// else Error
    ///
    fn from_dir(
        dir: &std::path::PathBuf,
    ) -> Result<(Self, Vec<ParseWarning>), (ParseError, Vec<ParseWarning>)> {
        // Get course info by searching for the course.toml file
        // TODO magic value maybe change this
        let course_info_file = dir.join(COURSE_INFO_FILE);
        let course_info = object_creator::create_from_file::<ProjectInfo>(&course_info_file)
            .map_err(|err| (err, vec![]))?;

        // Using the skill folders found in the course.toml file, parse every skill
        // /!\ Folders not found in the course.toml file are ignored /!\
        // TODO maybe warn if there are folder that aren't included in course.toml ?
        let mut warnings = Vec::new();
        let skills = course_info
            .skill_folders
            .iter()
            .filter_map(
                |skill_folder| match Skill::from_dir(&dir.join(skill_folder)) {
                    Ok((skill, mut skill_warnings)) => {
                        warnings.append(&mut skill_warnings);
                        Some(skill)
                    }
                    Err(error) => {
                        warnings.push(ParseWarning::ParseSkillFail(format!(
                            "Couldn't parse skill in {:?}: {:?}",
                            skill_folder, error
                        )));
                        None
                    }
                },
            )
            .collect::<Vec<Skill>>();

        if skills.is_empty() {
            Err((
                ParseError::ErrorParsingSkills(format!(
                    "Couldn't parse any skill folders in {:?}",
                    dir
                )),
                warnings,
            ))
        } else {
            Ok((
                Self {
                    name: course_info.name,
                    skills,
                },
                warnings,
            ))
        }
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use crate::models::{
        check::{Check, CheckTest},
        exo::Exo,
        exo_state::ExoState,
    };

    use super::*;

    #[test]
    fn test_example_full() {
        let project_path = std::path::PathBuf::from_str("examples/full").unwrap();
        let ret = Project::from_dir(&project_path);

        println!("{:#?}", ret);
        assert!(ret.is_ok());
        let (_project, warnings) = ret.unwrap();
        assert!(warnings.len() < 2);
    }
    #[test]
    fn test_full_hierarchy() {
        let project_path = std::path::PathBuf::from_str("examples/mock-plx-project").unwrap();
        let project = Project::from_dir(&project_path);
        let expected  = Project {
            name: String::from("Full fictive course"),
            skills: vec![
                Skill {
                    name: String::from("Introduction"),
                    path: project_path.join("intro"),
                    exos: vec![
                        Exo {
                            name: String::from("Basic arguments usage"),
                            instruction: Some(
                                String::from("The 2 first program arguments are the firstname and number of legs of a dog. Print a full sentence about the dog. Make sure there is at least 2 arguments, print an error if not."),
                            ),
                            state: ExoState::Todo,
                            files: vec![
                               project_path.join("intro").join("basic-args").join("main.c"),
                            ],
                            solutions: vec![
                                project_path.join("intro").join("basic-args").join("main.sol.c").into(),
                            ],
                            checks: vec![
                                Check {
                                    name: String::from("Joe + 5 legs"),
                                    args: vec![
                                        String::from("Joe"),
                                        String::from("5"),
                                    ],
                                    test: CheckTest::Output{expected: String::from("The dog is Joe and has 5 legs")},
                                },
                                Check {
                                    name: String::from("No arg -> error"),
                                    args: vec![],
                                    test: CheckTest::Output{ expected : String::from("Error: missing argument firstname and legs number")},
                                },
                                Check {
                                    name: String::from("One arg -> error"),
                                    args: vec![
                                        String::from("Joe"),
                                    ],
                                    test: CheckTest::Output {expected : String::from("Error: missing argument firstname and legs number")},
                                },
                            ],
                            favorite: false,
                        },
                        Exo {
                            name: String::from("Basic output printing"),
                            instruction: Some(
                                String::from("Just print 2 lines"),
                            ),
                            state: ExoState::Todo,
                            files: vec![
                               project_path.join("intro").join("basic-output").join("main.c"),
                            ],
                            solutions: vec![
                               project_path.join("intro").join("basic-output").join("main.sol.c"),
                            ],
                            checks: vec![
                                Check {
                                    name: String::from("Lines are correct"),
                                    args: vec![],
                                    test: CheckTest::Output{ expected: String::from("PLX is amazing !\nThis is a neutral opinion...\n")},
                                },
                            ],
                            favorite: false,
                        },
                    ],
                },
            ],
        };
        let (actual, warnings) = project.unwrap();
        assert_eq!(expected, actual);
        assert!(matches!(warnings[0], ParseWarning::ParseSkillFail(_)));
    }
}
