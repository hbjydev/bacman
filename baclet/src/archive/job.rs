use std::io;
use std::fs::{File, metadata};
use crate::archive::schema::ArchiveJobSpec;
use flate2::Compression;
use flate2::write::GzEncoder;
use crate::config::BacletJobType;

use crate::job::{JobType, JobRunError};

#[derive(Debug)]
pub enum ArchiveJobRunError {
    FileMetadataError(io::Error),
    CreateFileError(io::Error),
    AppendToTarError(io::Error),
}

pub struct ArchiveJob {
    pub spec: crate::config::BacletJobSpec,
}

impl ArchiveJob {
    pub fn init(spec: crate::config::BacletJobSpec) -> ArchiveJob {
        ArchiveJob { spec }
    }

    fn is_dir(&self, spec: ArchiveJobSpec) -> Result<bool, std::io::Error> {
        Ok(metadata(spec.src)?.is_dir())
    }
}

impl JobType<ArchiveJob, ArchiveJobRunError> for ArchiveJob {
    fn run(&self) -> Result<bool, JobRunError<ArchiveJobRunError>> {
        let BacletJobType::ArchiveJob(spec) = &self.spec.job_spec;

        let is_dir = self.is_dir(spec.clone())
            .map_err(|e| JobRunError { error: ArchiveJobRunError::FileMetadataError(e) })?;

        if is_dir {
            log::debug!("creating tar.gz file");
            let tar_gz = File::create(spec.dest.clone())
                .map_err(|e| JobRunError { error: ArchiveJobRunError::CreateFileError(e) })?;

            log::debug!("creating gz encoder");
            let enc = GzEncoder::new(tar_gz, Compression::default());
            
            log::debug!("filling gzipped tarball with directory contents");
            let mut tar = tar::Builder::new(enc);
            tar.append_dir_all(".", spec.src.clone())
                .map_err(|e| JobRunError { error: ArchiveJobRunError::AppendToTarError(e) })?;
        } else {
            log::warn!("non-directory backups aren't supported yet.");
            return Ok(false);
        }
        Ok(true)
    }
}
