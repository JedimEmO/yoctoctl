use crate::layers::domain::artifacts::bblayers::BBLayers;
use crate::layers::domain::artifacts::layerconf::LayerConf;
use crate::layers::domain::config_file::GitRevisionSpecifier;
use crate::layers::domain::generator::generator_structure::{LayerEntry, Project};

#[derive(Debug)]
pub enum Folder {
    Conf { project_id: String, name: String, bblayers: BBLayers, layer_conf: LayerConf },
    Submodule { project_id: String, name: String, git_url: String, git_revision: Option<GitRevisionSpecifier> },
}

#[derive(Debug)]
pub struct ProjectFolders {
    pub project_id: String,
    pub folders: Vec<Folder>,
}

impl ProjectFolders {
    pub fn new(project: &Project) -> ProjectFolders {
        let mut folders: Vec<Folder> = project.layer_entries.iter()
            .map(|layer_entry| {
                match layer_entry {
                    LayerEntry::GitSubmodule { submodule_name, git_revision, git_url, .. } => {
                        Folder::Submodule {
                            project_id: project.project_id.clone(),
                            name: submodule_name.clone(),
                            git_revision: git_revision.clone(),
                            git_url: git_url.clone(),
                        }
                    }
                    // Local layers have a conf directory
                    LayerEntry::Local(layer) => Folder::Conf {
                        project_id: project.project_id.clone(),
                        name: format!("{}/conf", layer),
                        bblayers: BBLayers::new_from_project(project),
                        layer_conf: LayerConf {
                            layer_name: layer.clone()
                        },
                    }
                }
            })
            .collect();

        ProjectFolders {
            project_id: project.project_id.clone(),
            folders,
        }
    }
}
