use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    pub project: Project,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub scope: String,
    pub version: String,
}