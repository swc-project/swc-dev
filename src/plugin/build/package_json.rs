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

    #[serde(skip)]
    pub description: String,

    pub keywords: Vec<String>,

    pub author: String,

    pub homepage: String,

    pub license: String,

    #[serde(skip_deserializing)]
    pub engines: AHashMap<String, String>,

    pub publish_config: AHashMap<String, String>,

    pub repository: AHashMap<String, String>,

    pub bugs: AHashMap<String, String>,
}
