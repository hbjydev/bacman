use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectVersion {
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    pub kind: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectMetadata {
    pub name: String,
    pub labels: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
}
