pub struct BBLayers {
    pub layers: Vec<String>,
}

impl BBLayers {
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
