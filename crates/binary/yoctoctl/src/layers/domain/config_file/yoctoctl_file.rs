use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum GitRevisionSpecifier {
    Hash { hash: String },
    Tag { tag: String },
    Branch { branch: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Layer {
    Submodule { submodule_name: String, git_url: String, relative_path: Option<String>, revision: Option<GitRevisionSpecifier> },
    InRepo { name: String },
}

/// Each project entry will correspond to one top level
/// init-env entry for build configuration
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub project_id: String,
    pub name: String,
    pub poky_revision: GitRevisionSpecifier,
    pub layers: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct YoctoctlFile {
    pub projects: Vec<Project>,
}

impl YoctoctlFile {
    pub fn new_from_str(input: &str) -> Result<YoctoctlFile, Box<dyn Error>> {
        toml::from_str(input)
            .map_err(|e| e.into())
    }
}


#[cfg(test)]
pub mod test {
    use crate::layers::domain::config_file::YoctoctlFile;

    pub const EXAMPLE_TOML_1: &str = r#"[[projects]]
project_id = "my-test"
name="test 1"
poky_revision = { branch = "hardknott" }

[[projects.layers]]
submodule_name = "meta-oe"
relative_path = "meta-python"
git_url = "git://git.openembedded.org/meta-openembedded"
revision = { hash = "123"}

[[projects.layers]]
submodule_name = "meta-oe"
relative_path = "meta-networking"
git_url = "git://git.openembedded.org/meta-openembedded"
revision = { hash = "123"}

[[projects.layers]]
name = "meta-my-internal"
"#;

    #[test]
    fn parses_file_correctly() {
        let file = YoctoctlFile::new_from_str(EXAMPLE_TOML_1).unwrap();
    }
}
