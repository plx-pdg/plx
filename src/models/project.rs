use serde::Deserialize;

use crate::core::{
    file_utils::file_parser::{ParseError, ParseWarning},
    parser::{from_dir::FromDir, object_creator},
};

use super::skill::Skill;

#[derive(Debug, PartialEq, Eq)]
pub struct Project {
    name: String,
    skills: Vec<Skill>,
}

#[derive(Deserialize)]
struct ProjectInfo {
    name: String,
    #[serde(alias = "skills")]
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
        let course_info_file = dir.join("course.toml");
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

    use crate::models::{
        check::{Check, CheckType},
        exo::Exo,
        exo_state::ExoState,
        solution::Solution,
    };

    use super::*;

    #[test]
    fn test_full_hierarchy() {
        let project = Project::from_dir(&("examples/full".into()));
        let expected : Result<(Project, Vec<ParseWarning>), (ParseError, Vec<ParseWarning>)> = Ok((
            Project {
            name: String::from("Full fictive course"),
            skills: vec![
                Skill {
                    name: String::from("Introduction"),
                    path: "examples/full/intro".into(),
                    exos: vec![
                        Exo {
                            name: String::from("Basic arguments usage"),
                            instruction: Some(
                                String::from("The 2 first program arguments are the firstname and number of legs of a dog. Print a full sentence about the dog. Make sure there is at least 2 arguments, print an error if not."),
                            ),
                            state: ExoState::Todo,
                            files: vec![
                                "examples/full/intro/basic-args/main.c".into(),
                            ],
                            solution: Some(
                                Solution {
                                    path: "examples/full/intro/basic-args/main.sol.c".into(),
                                },
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
                        },
                        Exo {
                            name: String::from("Basic output printing"),
                            instruction: Some(
                                String::from("Just print 2 lines"),
                            ),
                            state: ExoState::Todo,
                            files: vec![
                                "examples/full/intro/basic-output/main.c".into(),
                            ],
                            solution: Some(
                                Solution {
                                    path: "examples/full/intro/basic-output/main.sol.c".into(),
                                },
                            ),
                            checks: vec![
                                Check {
                                    name: String::from("Lines are correct"),
                                    args: vec![],
                                    check_type: CheckType::Output,
                                },
                            ],
                            favorite: false,
                        },
                    ],
                },
            ],
        },
        vec![
            ParseWarning::ParseSkillFail(
                String::from("Couldn't parse skill in \"pointers\": (ReadFileError(\"No such file or directory (os error 2)\"), [])"),
            ),
        ],
    ),
    );
        let (expected_project, expected_warnings) = expected.unwrap();
        let (project, warnings) = project.unwrap();
        assert_eq!(expected_project, project);
        assert_eq!(expected_warnings, warnings);
        println!("{:#?}", project);
    }
}
