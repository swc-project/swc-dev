use crate::util::AHashMap;
use serde::{Deserialize, Serialize};
use swc_node_arch::{NodeArch, NodePlatform};

/// A `package.json` file for a binary package.
///
/// (binary package means a platfomr-dependant package)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJsonForBin {
    pub name: String,
    pub version: String,

    #[serde(skip_deserializing, skip_serializing_if = "Vec::is_empty")]
    pub os: Vec<NodePlatform>,

    #[serde(skip_deserializing, skip_serializing_if = "Vec::is_empty")]
    pub cpu: Vec<NodeArch>,

    #[serde(skip_deserializing, skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,

    #[serde(skip_deserializing, skip_serializing_if = "String::is_empty")]
    pub description: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub author: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub homepage: String,

    pub license: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub main: String,

    #[serde(skip_deserializing, skip_serializing_if = "AHashMap::is_empty")]
    pub engines: AHashMap<String, String>,

    #[serde(default, skip_serializing_if = "AHashMap::is_empty")]
    pub publish_config: AHashMap<String, String>,

    #[serde(default, skip_serializing_if = "AHashMap::is_empty")]
    pub repository: AHashMap<String, String>,

    #[serde(default, skip_serializing_if = "AHashMap::is_empty")]
    pub bugs: AHashMap<String, String>,
}
