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
    pub name: String,
    pub poky_revision: GitRevisionSpecifier,
    pub layer_entries: Vec<LayerEntry>
}

/// Represents the output structure to generate
pub struct GeneratorStructure {
    projects: Vec<Project>,
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
    use crate::layers::domain::generator::generator_structure::{Project, LayerEntry};
    use crate::layers::domain::config_file::test::EXAMPLE_TOML_1;
    use crate::layers::domain::generator::project_config_to_generator::project_config_to_generator;
    use crate::layers::domain::config_file::YoctoctlFile;

    #[test]
    fn converts_basic_config_to_generator() {
        let config = YoctoctlFile::new_from_str(EXAMPLE_TOML_1).unwrap();

        let projects: Vec<Project> = config.projects.into_iter()
            .map(project_config_to_generator)
            .collect();

        let project_1 = projects.first().unwrap();
        let layer_2 = project_1.layer_entries.iter().nth(1).unwrap();

        match layer_2 {
            LayerEntry::GitSubmodule { git_url, submodule_name, git_revision, layer_entries } => {
                assert_eq!(layer_entries.len(), 2)
            }
            LayerEntry::Local(_) => {
                panic!("epojig")
            }
        }
    }
}
