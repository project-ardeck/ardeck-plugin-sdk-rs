use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub id: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub main: String,
}