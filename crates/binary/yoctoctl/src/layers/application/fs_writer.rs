use std::error::Error;
use std::path::Path;
use std::sync::RwLock;

use git2::{Repository, SubmoduleUpdateOptions};

use crate::layers::domain::artifacts::project_folders::Folder;
use crate::layers::domain::config_file::GitRevisionSpecifier;
use crate::layers::domain::writer::yoctoctl_project_writer::YoctoctlProjectWriter;

pub struct FsWriter {
    pub git_repo: RwLock<Repository>,
    pub root_path: Box<Path>,
}

impl YoctoctlProjectWriter for FsWriter {
    fn write_folder(&self, folder: Folder) -> Result<(), Box<dyn Error>> {
        match folder {
            Folder::Submodule { name, project_id, git_url, git_revision } => {
                // Add a submodule
                let submodule_path = format!("{}/layers/{}", project_id, name);

                if self.git_repo.read().unwrap().find_submodule(submodule_path.as_str()).is_err() {
                    {
                        let repo_read = self.git_repo.read().unwrap();


                        let mut submodule = repo_read.submodule(git_url.as_str(), Path::new(submodule_path.as_str()), true)
                            .unwrap();
                        submodule.init(false).unwrap();
                    };

                    {
                        let mut repo = self.git_repo.write().unwrap();

                        repo.submodule_set_branch(submodule_path.as_str(), match git_revision {
                            Some(GitRevisionSpecifier::Branch { branch }) => Some(branch),
                            _ => None
                        }.unwrap().as_str()).unwrap();
                    }

                    {
                        let repo_read = self.git_repo.read().unwrap();

                        let mut submodule = repo_read.find_submodule(submodule_path.as_str()).unwrap();
                        submodule.sync().unwrap();
                        println!("Cloning {} into {}...", git_url, submodule_path);
                        submodule.clone(None).unwrap();
                        submodule.add_to_index(true).unwrap();
                        ;
                        submodule.add_finalize().unwrap();
                    }
                }
            }
            Folder::Conf { name, project_id, layer_conf, bblayers } => {
                let rel_path = format!("{}/layers/{}", project_id, name);
                let out_path = self.root_path.join(rel_path);

                let bblayers_file = bblayers.to_string();
                let bblayers_path = out_path.join("bblayers.conf.sample");

                let layer_conf_file = layer_conf.to_string();
                let layer_conf_path = out_path.join("layer.conf");

                println!("Writing bblayers config to {}", bblayers_path.as_path().to_str().unwrap());
                std::fs::create_dir_all(out_path).unwrap();
                std::fs::write(bblayers_path, bblayers_file).unwrap();
                std::fs::write(layer_conf_path, layer_conf_file).unwrap();
            }
        };

        Ok(())
    }
}
