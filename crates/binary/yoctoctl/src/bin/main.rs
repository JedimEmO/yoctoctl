use clap::Clap;
use git2::Repository;

use yoctoctl::layers::application::cmdline_params::{AppCommand, YoctoctlCmdline};
use yoctoctl::layers::application::fs_writer::FsWriter;
use yoctoctl::layers::domain::generate_process::generate_yocto_projects;
use std::sync::RwLock;

fn main() {
    let cmdline_params = YoctoctlCmdline::parse();

    match cmdline_params.command {
        AppCommand::Generate(config) => {
            let dst_path = match config.output_directory {
                Some(p) => std::env::current_dir().unwrap().join(p),
                _ => std::env::current_dir().unwrap()
            };

            let repository = Repository::init(dst_path.clone()).unwrap();

            let writer = FsWriter { root_path: dst_path.into_boxed_path(), git_repo: RwLock::new(repository) };

            let src = std::fs::read_to_string(config.config_file).unwrap();

            generate_yocto_projects(src.as_str(), writer).unwrap();
        }
    }
}
