use serde::{Serialize, Deserialize};

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
