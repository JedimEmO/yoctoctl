use crate::layers::domain::generator::generator_structure::GeneratorStructure;
use crate::layers::domain::artifacts::project_folders::Folder;
use std::error::Error;

pub trait YoctoctlProjectWriter {
    fn write_folder(&self, folder: Folder) -> Result<(), Box<dyn Error>>;
}
