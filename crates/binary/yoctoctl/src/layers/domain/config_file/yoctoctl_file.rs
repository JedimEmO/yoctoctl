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
    Submodule { name: String, git_url: String, revision: Option<GitRevisionSpecifier> },
    InRepo { name: String },
}

/// Each project entry will correspond to one top level
/// init-env entry for build configuration
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub name: String,
    pub poky_revision: GitRevisionSpecifier,
    pub layers: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct YoctoctlFile {
    pub projects: Vec<Project>,
}


#[cfg(test)]
mod test {
    use crate::layers::domain::config_file::YoctoctlFile;

    #[test]
    fn parses_file_correctly() {
        let example_toml = r#"[[projects]]
name="test 1"
poky_revision = { branch = "hardknott" }
[[projects.layers]]
name = "meta-my-internal"

[[projects.layers]]
name = "meta-oe/meta-python"
git_url = "git://git.openembedded.org/meta-openembedded"
revision = { hash = "123"}
"#;
        let parsed: YoctoctlFile = toml::from_str(example_toml).unwrap();

        println!("parsed file: {:?}", parsed);
    }
}
