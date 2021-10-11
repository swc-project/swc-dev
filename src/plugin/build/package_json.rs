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

    pub os: Vec<NodePlatform>,

    pub cpu: Vec<NodeArch>,

    pub files: Vec<String>,

    pub description: String,

    pub keywords: Vec<String>,

    pub author: String,

    pub homepage: String,

    pub license: String,

    pub engines: AHashMap<String, String>,

    pub publish_config: AHashMap<String, String>,

    pub repository: AHashMap<String, String>,

    pub bugs: AHashMap<String, String>,
}
