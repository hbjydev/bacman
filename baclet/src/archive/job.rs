use std::fs::{File, metadata};
use crate::archive::schema::ArchiveJobSpec;
use flate2::Compression;
use flate2::write::GzEncoder;

pub enum ArchiveJobRunError {
    FileMetadataError(std::io::Error),
    CreateFileError(std::io::Error),
    AppendToTarError(std::io::Error),
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
            let is_dir = match self.is_dir(spec.clone()) {
                Ok(v) => v,
                Err(e) => return Err(ArchiveJobRunError::FileMetadataError(e)),
            };

            if is_dir {
                log::debug!("creating tar.gz file");
                let tar_gz = match File::create(spec.dest.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(ArchiveJobRunError::CreateFileError(e)),
                };

                log::debug!("creating gz encoder");
                let enc = GzEncoder::new(tar_gz, Compression::default());
                
                log::debug!("filling gzipped tarball with directory contents");
                let mut tar = tar::Builder::new(enc);
                match tar.append_dir_all(".", spec.src.clone()) {
                    Ok(v) => v,
                    Err(e) => return Err(ArchiveJobRunError::AppendToTarError(e)),
                };
            } else {
                log::warn!("non-directory backups aren't supported yet.");
                return Ok(false);
            }
            return Ok(true)
        }

        Ok(false)
    }
}
