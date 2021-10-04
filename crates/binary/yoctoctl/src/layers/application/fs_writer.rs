use std::error::Error;
use std::path::Path;
use std::sync::RwLock;

use git2::{BranchType, ObjectType, Repository, ResetType, SubmoduleUpdateOptions};

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
                generate_git_submodule(&self.git_repo, project_id, name, git_url, git_revision)
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


fn generate_git_submodule(git_repo: &RwLock<Repository>, project_id: String, name: String, git_url: String, git_revision: Option<GitRevisionSpecifier>) {
    // Add a submodule
    let submodule_path = format!("{}/layers/{}", project_id, name);
    let submodule_repo_path = git_repo.read().unwrap().path().join(format!("../{}", submodule_path.as_str()));

    if git_repo.read().unwrap().find_submodule(submodule_path.as_str()).is_err() {
        let repo_read = git_repo.read().unwrap();

        let mut submodule = repo_read.submodule(git_url.as_str(), Path::new(submodule_path.as_str()), true)
            .unwrap();

        std::fs::remove_dir_all(submodule_repo_path.clone()).unwrap();
        Repository::clone(git_url.as_str(), submodule_repo_path.clone()).unwrap();

        submodule.add_to_index(false).unwrap();
        submodule.add_finalize().unwrap();
    }


    let sm_repo = Repository::open(submodule_repo_path).unwrap();

    let mut sm_remote = sm_repo.find_remote("origin").unwrap();
    sm_remote.download(&[] as &[&str], None).unwrap();

    let revision = match git_revision {
        Some(GitRevisionSpecifier::Branch { branch }) => {
            println!("checking out: refs/remotes/origin/{}", branch);
            sm_repo.find_reference(format!("refs/remotes/origin/{}", branch).as_str()).unwrap()
        }
        Some(GitRevisionSpecifier::Tag { tag }) => {
            println!("checking out: refs/tags/{}", tag);
            sm_repo.find_reference(format!("refs/tags/{}", tag).as_str()).unwrap()
        }
        _ => unimplemented!()
    };

    sm_repo.set_head_detached(revision.target().unwrap()).unwrap();
    sm_repo.reset(&revision.peel(ObjectType::Any).unwrap(), ResetType::Hard, None).unwrap();
}
