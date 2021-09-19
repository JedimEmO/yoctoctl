#[derive(Debug)]
pub struct LayerConf {
    pub layer_name: String,
}


impl LayerConf {
    pub fn to_string(&self) -> String {
        format!(r#"BBPATH .= ":${{LAYERDIR}}"

# We have recipes-* directories, add to BBFILES
BBFILES += "${{LAYERDIR}}/recipes/**/*.bb \
            ${{LAYERDIR}}/recipes/**/*.bbappend"

BBFILE_COLLECTIONS += "{}"
BBFILE_PATTERN_meta-{} = "^${{LAYERDIR}}/"
BBFILE_PRIORITY_meta-{} = "6"

LAYERDEPENDS_meta-{} = "core"
LAYERSERIES_COMPAT_meta-{} = "hardknott""#,
                self.layer_name, self.layer_name, self.layer_name, self.layer_name, self.layer_name)
    }
}
