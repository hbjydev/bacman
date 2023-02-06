use std::io;
use std::fs::{File, metadata};
use crate::archive::schema::ArchiveJobSpec;
use flate2::Compression;
use flate2::write::GzEncoder;

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

    pub fn run(&self) -> Result<bool, ArchiveJobRunError> {
        if let Some(spec) = &self.spec.archive_job {
            let is_dir = self.is_dir(spec.clone())
                .map_err(ArchiveJobRunError::FileMetadataError)?;

            if is_dir {
                log::debug!("creating tar.gz file");
                let tar_gz = File::create(spec.dest.clone())
                    .map_err(ArchiveJobRunError::CreateFileError)?;

                log::debug!("creating gz encoder");
                let enc = GzEncoder::new(tar_gz, Compression::default());
                
                log::debug!("filling gzipped tarball with directory contents");
                let mut tar = tar::Builder::new(enc);
                tar.append_dir_all(".", spec.src.clone())
                    .map_err(ArchiveJobRunError::AppendToTarError)?;
            } else {
                log::warn!("non-directory backups aren't supported yet.");
                return Ok(false);
            }
            return Ok(true)
        }

        Ok(false)
    }
}
