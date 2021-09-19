use std::collections::HashMap;

use itertools::Itertools;

use crate::layers::domain::config_file::{GitRevisionSpecifier, Layer};
use crate::layers::domain::generator::generator_structure::LayerEntry;

/// Takes a collection of configured layer definitions,
/// and transforms it into a list of entries to generate
/// in the layers directory.
///
/// Note that this will combine layers from the same submodule
/// into one entry.
pub(crate) fn layer_config_to_entries(config: Vec<Layer>) -> Vec<LayerEntry> {
    config.into_iter()
        // Convert to fully formed entries
        .map(|cfg| {
            (layer_to_folder_name(&cfg), cfg)
        })
        .group_by(|entry| entry.0.clone())
        .into_iter()
        .map(|(key, value)| {
            (key, value.map(|e| e.1).collect::<Vec<Layer>>())
        })
        .map(map_layer_group_to_layer_entry)
        .filter_map(|v| v)
        .collect()
}

fn map_layer_group_to_layer_entry(entry: (String, Vec<Layer>)) -> Option<LayerEntry> {
    entry.1
        .into_iter()
        .fold(None, |current, item| {
            match current {
                Some(LayerEntry::GitSubmodule { git_url, submodule_name, git_revision, mut layer_entries }) => {
                    Some(LayerEntry::GitSubmodule {
                        git_url,
                        submodule_name,
                        git_revision,
                        layer_entries: add_layer_to_entries_vec(layer_entries, item),
                    })
                }
                Some(LayerEntry::Local(path)) => {
                    Some(LayerEntry::Local(path))
                }
                None => {
                    match item {
                        Layer::InRepo { name } => Some(LayerEntry::Local(name)),
                        Layer::Submodule { submodule_name, git_url, relative_path, revision } => {
                            Some(LayerEntry::GitSubmodule {
                                git_url,
                                submodule_name,
                                git_revision: revision,
                                layer_entries: vec![
                                    match relative_path {
                                        Some(rel) => rel,
                                        _ => "".to_string()
                                    }
                                ],
                            })
                        }
                    }
                }
            }
        })
}

fn add_layer_to_entries_vec(mut layer_entries: Vec<String>, item: Layer) -> Vec<String> {
    layer_entries.push(match item {
        Layer::Submodule { relative_path, .. } => {
            match relative_path {
                Some(v) => v,
                _ => "".into()
            }
        }
        Layer::InRepo { name } => name
    });

    layer_entries
}

fn layer_to_folder_name(cfg: &Layer) -> String {
    match &cfg {
        Layer::InRepo { name } => name.clone(),
        Layer::Submodule { git_url, revision, submodule_name, .. } => {
            let rev = opt_git_rev_to_string(revision);

            format!("{}+{}+{}", git_url, rev, submodule_name, )
        }
    }
}
fn opt_git_rev_to_string(revision: &Option<GitRevisionSpecifier>) -> String {
    match revision {
        Some(r) => {
            match r {
                GitRevisionSpecifier::Hash { hash } => hash.clone(),
                GitRevisionSpecifier::Branch { branch } => branch.clone(),
                GitRevisionSpecifier::Tag { tag } => tag.clone()
            }
        }
        _ => "any".to_string()
    }
}
