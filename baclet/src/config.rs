use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::destinations::schema::DestinationSpec;
use crate::schema::job::JobSpec;
use crate::schema::shared::ObjectVersion;

#[derive(Serialize, Deserialize, Debug)]
pub struct BacletConfigSpec {
    pub destinations: Vec<DestinationSpec>,
    pub jobs: Vec<JobSpec>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BacletConfig {
    #[serde(flatten)]
    pub version: ObjectVersion,
    pub spec: BacletConfigSpec,
}

#[derive(Debug)]
pub enum BacletConfigInitError {
    FileError(std::io::Error),
    DeserializeError(serde_yaml::Error),
}

impl BacletConfig {
    pub fn from_file(path: &PathBuf) -> Result<BacletConfig, BacletConfigInitError> {
        let config_content = std::fs::read_to_string(path)
            .map_err(BacletConfigInitError::FileError)?;

        let config = serde_yaml::from_str::<BacletConfig>(&config_content)
            .map_err(BacletConfigInitError::DeserializeError)?;

        Ok(config)
    }
}
