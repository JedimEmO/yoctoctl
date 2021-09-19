use crate::layers::domain::artifacts::bblayers::BBLayers;
use crate::layers::domain::generator::generator_structure::{LayerEntry, Project};

pub fn project_to_bblayers(project: &Project) -> BBLayers {
    BBLayers {
        layers: project.layer_entries.iter()
            .flat_map(|l| match l {
                LayerEntry::Local(name) => vec![name.clone()],
                LayerEntry::GitSubmodule { submodule_name, layer_entries, .. } => {
                    layer_entries.into_iter()
                        .map(|lentry| format!("{}/{}", submodule_name, lentry))
                        .collect()
                }
            }).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::layers::domain::config_file::{YoctoctlFile};
    use crate::layers::domain::config_file::test::EXAMPLE_TOML_1;
    use crate::layers::domain::generator::project_config_to_generator::project_config_to_generator;
    use crate::layers::domain::generator::generator_structure::Project;
    use crate::layers::domain::generator::project_to_bblayers::project_to_bblayers;

    #[test]
    fn can_serialize_bblayers() {
        let config = YoctoctlFile::new_from_str(EXAMPLE_TOML_1).unwrap();

        let projects: Vec<Project> = config.projects.into_iter()
            .map(project_config_to_generator)
            .collect();

        let project_1 = projects.first().unwrap();

        let bblayers = project_to_bblayers(&project_1);

        println!("{}", bblayers.to_string());
    }
}
