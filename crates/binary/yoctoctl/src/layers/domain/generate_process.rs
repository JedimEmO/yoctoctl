use std::error::Error;

use crate::layers::domain::artifacts::project_folders::ProjectFolders;
use crate::layers::domain::config_file::YoctoctlFile;
use crate::layers::domain::generator::generator_structure::{GeneratorStructure, Project};
use crate::layers::domain::writer::yoctoctl_project_writer::YoctoctlProjectWriter;

pub fn generate_yocto_projects<Writer: YoctoctlProjectWriter>
(input: &str, writer: Writer) -> Result<(), Box<dyn Error>> {
    let config = YoctoctlFile::new_from_str(input)?;
    let structure = GeneratorStructure::from_config(config)?;

    structure.projects.iter()
        .map(|project| write_project(project, &writer))
        .collect::<Result<(), Box<dyn Error>>>()?;

    Ok(())
}

fn write_project<Writer: YoctoctlProjectWriter>(project: &Project, writer: &Writer) -> Result<(), Box<dyn Error>> {
    let folders = ProjectFolders::new(project);

    folders.folders.into_iter()
        .map(|f| writer.write_folder(f))
        .collect::<Result<(), Box<dyn Error>>>()?;

    Ok(())
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use std::rc::Rc;
    use std::sync::RwLock;

    use crate::layers::domain::artifacts::project_folders::Folder;
    use crate::layers::domain::config_file::test::EXAMPLE_TOML_1;
    use crate::layers::domain::generate_process::generate_yocto_projects;
    use crate::layers::domain::writer::yoctoctl_project_writer::YoctoctlProjectWriter;

    struct WriterStub {
        pub data: Rc<RwLock<Vec<String>>>,
    }

    impl YoctoctlProjectWriter for WriterStub {
        fn write_folder(&self, folder: Folder) -> Result<(), Box<dyn Error>> {
            self.data.write().unwrap().push(format!("{:?}", folder));
            Ok(())
        }
    }

    #[test]
    fn writes_correct_folders() {
        let data = Rc::new(RwLock::new(Vec::new()));

        let writer = WriterStub {
            data: data.clone()
        };

        generate_yocto_projects(EXAMPLE_TOML_1, writer).unwrap();

        assert_eq!(data.read().unwrap().len(), 3);
    }
}
