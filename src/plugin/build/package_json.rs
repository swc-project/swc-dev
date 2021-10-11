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

    #[serde(skip_deserializing)]
    pub os: Vec<NodePlatform>,

    #[serde(skip_deserializing)]
    pub cpu: Vec<NodeArch>,

    #[serde(skip_deserializing)]
    pub files: Vec<String>,

    #[serde(skip_deserializing)]
    pub description: String,

    #[serde(default)]
    pub keywords: Vec<String>,

    #[serde(default)]
    pub author: String,

    #[serde(default)]
    pub homepage: String,

    pub license: String,

    #[serde(skip_deserializing)]
    pub engines: AHashMap<String, String>,

    #[serde(default)]
    pub publish_config: AHashMap<String, String>,

    #[serde(default)]
    pub repository: AHashMap<String, String>,

    #[serde(default)]
    pub bugs: AHashMap<String, String>,
}
