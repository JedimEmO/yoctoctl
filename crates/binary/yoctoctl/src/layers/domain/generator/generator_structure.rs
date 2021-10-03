use std::error::Error;

use crate::layers::domain::config_file::{GitRevisionSpecifier, YoctoctlFile};
use crate::layers::domain::generator::project_config_to_generator::project_config_to_generator;

pub enum LayerEntry {
    GitSubmodule {
        git_url: String,
        submodule_name: String,
        git_revision: Option<GitRevisionSpecifier>,
        layer_entries: Vec<String>,
    },
    Local(String),
}

pub struct Project {
    pub project_id: String,
    pub name: String,
    pub poky_url: String,
    pub poky_revision: GitRevisionSpecifier,
    pub layer_entries: Vec<LayerEntry>,
}

/// Represents the output structure to generate
pub struct GeneratorStructure {
    pub projects: Vec<Project>,
}

impl GeneratorStructure {
    pub fn from_config(config: YoctoctlFile) -> Result<Self, Box<dyn Error>> {
        let projects = config.projects
            .into_iter()
            .map(project_config_to_generator)
            .collect();

        Ok(GeneratorStructure {
            projects
        })
    }
}

#[cfg(test)]
mod test {
    use crate::layers::domain::config_file::test::EXAMPLE_TOML_1;
    use crate::layers::domain::config_file::YoctoctlFile;
    use crate::layers::domain::generator::generator_structure::{LayerEntry, Project};
    use crate::layers::domain::generator::project_config_to_generator::project_config_to_generator;
    use crate::layers::domain::artifacts::project_folders::ProjectFolders;

    #[test]
    fn converts_basic_config_to_generator() {
        let config = YoctoctlFile::new_from_str(EXAMPLE_TOML_1).unwrap();

        let projects: Vec<Project> = config.projects.into_iter()
            .map(project_config_to_generator)
            .collect();

        let project_1 = projects.first().unwrap();
        let meta_oe = project_1.layer_entries.iter().find(|item| match item {
            LayerEntry::GitSubmodule { submodule_name, .. } => submodule_name == "meta-oe",
            _ => false
        }).unwrap();

        match meta_oe {
            LayerEntry::GitSubmodule { git_url, submodule_name, git_revision, layer_entries } => {
                assert_eq!(layer_entries.len(), 2)
            }
            LayerEntry::Local(_) => {
                panic!("epojig")
            }
        }
    }

    #[test]
    fn creates_correct_output_artifacts() {
        let config = YoctoctlFile::new_from_str(EXAMPLE_TOML_1).unwrap();

        let projects: Vec<Project> = config.projects.into_iter()
            .map(project_config_to_generator)
            .collect();

        let project_1 = projects.first().unwrap();

        let folders = ProjectFolders::new(project_1);
    }
}
