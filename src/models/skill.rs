use crate::core::{
    file_utils::file_parser::{ParseError, ParseWarning},
    parser::{self, from_dir::FromDir},
};

use super::{constants::SKILL_INFO_FILE, exo::Exo};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub struct Skill {
    pub name: String,
    pub path: std::path::PathBuf,
    pub exos: Vec<Exo>,
}
#[derive(Deserialize, Serialize)]
struct SkillInfo {
    name: String,
    #[serde(rename = "exos")]
    exo_folders: Vec<std::path::PathBuf>,
}
impl Skill {
    pub fn new(name: String, path: std::path::PathBuf, exos: Vec<Exo>) -> Self {
        Self { name, path, exos }
    }
}
impl FromDir for Skill {
    ///
    /// Tries to build a skill from dir
    /// Returns Ok if we were able to get the skill info and at least 1 exo
    /// else Error
    ///
    fn from_dir(
        dir: &std::path::PathBuf,
    ) -> Result<(Self, Vec<ParseWarning>), (ParseError, Vec<ParseWarning>)> {
        // Get skill info by searching for the skill.toml file
        let skill_info_file = dir.join(SKILL_INFO_FILE);
        let info = parser::object_creator::create_from_file::<SkillInfo>(&skill_info_file)
            .map_err(|err| (err, vec![]))?;

        // Using the exo folders found in the skill.toml file, parse every skill
        // /!\ Folders not found in the skill.toml file are ignored /!\
        // TODO maybe warn if there are folder that aren't included in skill.toml ?
        let mut warnings = Vec::new();
        let exos = info
            .exo_folders
            .iter()
            .filter_map(|exo_folder| match Exo::from_dir(&dir.join(exo_folder)) {
                Ok((exo, mut exo_warnings)) => {
                    warnings.append(&mut exo_warnings);
                    Some(exo)
                }
                Err(err) => {
                    warnings.push(ParseWarning::ParseExoFail(format!(
                        "Couldn't Parse Exo {:?}",
                        err
                    )));
                    None
                }
            })
            .collect::<Vec<Exo>>();

        if exos.is_empty() {
            Err((
                ParseError::ErrorParsingExos(format!("Couldn't parse any exo in {:?}", dir)),
                warnings,
            ))
        } else {
            Ok((
                Self {
                    name: info.name,
                    path: dir.to_path_buf(),
                    exos,
                },
                warnings,
            ))
        }
    }
}
