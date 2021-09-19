use crate::layers::domain::config_file::Project as ConfigProject;
use crate::layers::domain::generator::generator_structure::Project;
use crate::layers::domain::generator::layer_config_to_entries::layer_config_to_entries;

pub(crate) fn project_config_to_generator(project: ConfigProject) -> Project {
    Project {
        name: project.name,
        poky_revision: project.poky_revision,
        layer_entries: layer_config_to_entries(project.layers)
    }
}
