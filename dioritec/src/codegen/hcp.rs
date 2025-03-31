//! Hypercube Project (unofficial format) serializer
//! 

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HypercubeProject {
    pub hypercube_project: String,
    pub metadata: ProjectMeta,
    pub lines: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub name: String,
    pub version: String,
    pub mc_version: String,
    pub description: Option<String>,
    pub license: String,
    pub authors: Vec<String>
}
