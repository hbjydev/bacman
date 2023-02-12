use serde::{Serialize, Deserialize};
use anyhow::Result;

use super::local_path::LocalPathDestinationSpec;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DestinationType {
    #[serde(rename = "localPathDestination")]
    LocalPath(LocalPathDestinationSpec),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DestinationSpec {
    pub name: String,

    #[serde(flatten)]
    pub destination_spec: DestinationType,
}

pub trait DestinationTypeImpl {
    fn send(&self, path: &str) -> Result<bool>;
}
