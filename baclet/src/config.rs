use std::{collections::HashMap, path::PathBuf};

use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct BacletConfigSpec {
    pub jobs: Vec<BacletJobSpec>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BacletConfig {
    #[serde(flatten)]
    pub version: ObjectVersion,

    pub spec: BacletConfigSpec,
}

/// A specification for a backup job
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BacletJobSpec {
    /// The programmatic name of the job
    pub name: String,

    /// The archive job to run
    #[serde(rename = "archiveJob")]
    pub archive_job: Option<crate::archive::schema::ArchiveJobSpec>,

    /// The cron-syntax schedule to run the job on
    pub schedule: String,
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
