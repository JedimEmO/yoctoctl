use crate::layers::domain::generator::generator_structure::Project;
use crate::layers::domain::generator::project_to_bblayers::project_to_bblayers;

#[derive(Debug)]
pub struct BBLayers {
    pub layers: Vec<String>,
}

impl BBLayers {
    pub fn new_from_project(project: &Project) -> BBLayers {
        project_to_bblayers(project)
    }

    pub fn to_string(&self) -> String {
        let layer_list: Vec<String> = self.layers.iter()
            .map(|layer|
                format!("  ##OEROOT##/../layers/{} \\", layer))
            .collect();

        format!(r#"LCONF_VERSION = 6
BBPATH = "${{TOPDIR}}"

BBLAYERS ?= " \
{}
"
"#, layer_list.join("\n"))
    }
}
