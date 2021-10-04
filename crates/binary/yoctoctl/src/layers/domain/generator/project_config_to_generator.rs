use crate::layers::domain::config_file::{Project as ConfigProject, GitRevisionSpecifier, GitModule};
use crate::layers::domain::generator::generator_structure::Project;
use crate::layers::domain::generator::layer_config_to_entries::layer_config_to_entries;

pub(crate) fn project_config_to_generator(project: ConfigProject) -> Project {
    Project {
        project_id: project.project_id,
        name: project.name,
        bitbake: project.bitbake,
        layer_entries: layer_config_to_entries(project.layers),
        openembedded_core: project.openembedded_core
    }
}
