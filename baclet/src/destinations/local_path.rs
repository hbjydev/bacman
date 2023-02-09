use std::{io, fs, path};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LocalPathEnsureError {
    #[error("generic I/O failure")]
    GenericIOFail(#[from] io::Error),

    #[error("path exists but is not a directory")]
    NotADir,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocalPathDestinationSpec {
    pub path: String,
}

pub struct LocalPath {
    pub spec: LocalPathDestinationSpec,
}

impl LocalPath {
    pub fn from_spec(spec: LocalPathDestinationSpec) -> Self {
        LocalPath { spec }
    }

    /// Make sure path in spec exists and create it if possible.
    pub fn ensure(&self) -> Result<(), LocalPathEnsureError> {
        let exists = path::Path::new(&self.spec.path.clone()).exists();
        if !exists {
            fs::create_dir(self.spec.path.clone())?;
        } else {
            let stat = fs::metadata(self.spec.path.clone())?;
            if !stat.is_dir() {
                return Err(LocalPathEnsureError::NotADir);
            }
        }

        Ok(())
    }
}
