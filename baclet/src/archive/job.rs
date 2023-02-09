use crate::archive::schema::ArchiveJobSpec;
use crate::destinations::schema::DestinationSpec;
use crate::job::JobTypeImpl;
use crate::schema::job::{JobSpec, JobType};
use flate2::write::GzEncoder;
use flate2::Compression;
use mktemp::Temp;
use std::collections::HashMap;
use std::fs::{metadata, File};
use std::io;

use crate::schema::job::JobRunError;

#[derive(Debug)]
pub enum ArchiveJobRunError {
    MissingDestinationError,
    TempFileCreateError(io::Error),
    FileMetadataError(io::Error),
    OpenTempFileError(io::Error),
    AppendToTarError(io::Error),
}

pub struct ArchiveJob {
    pub spec: JobSpec,

    dests: HashMap<String, DestinationSpec>,
}

impl ArchiveJob {
    pub fn init(spec: JobSpec, dests: HashMap<String, DestinationSpec>) -> ArchiveJob {
        ArchiveJob { spec, dests }
    }

    fn is_dir(&self, spec: ArchiveJobSpec) -> Result<bool, std::io::Error> {
        Ok(metadata(spec.src)?.is_dir())
    }
}

impl JobTypeImpl<ArchiveJob, ArchiveJobRunError> for ArchiveJob {
    fn run(&self) -> Result<bool, JobRunError<ArchiveJobRunError>> {
        let maybe_dest = self.dests.get(&self.spec.destination);
        if maybe_dest.is_none() {
            return Err(JobRunError {
                error: ArchiveJobRunError::MissingDestinationError,
            });
        } else {
            let dest = maybe_dest.unwrap();
            log::debug!("Shipping backup to destination \"{}\"", dest.name);
            let JobType::ArchiveJob(spec) = &self.spec.job_spec;

            // Generate temporary directory
            // TODO Make this base path configurable
            let temp_file = Temp::new_file().map_err(|e| JobRunError {
                error: ArchiveJobRunError::TempFileCreateError(e),
            })?;

            // Check if the destination is a directory (and if it is accessible)
            let is_dir = self.is_dir(spec.clone()).map_err(|e| JobRunError {
                error: ArchiveJobRunError::FileMetadataError(e),
            })?;

            // If it's a directory, create an archive of the entire directory and store it in the
            // temporary directory we just created.
            if is_dir {
                log::debug!("creating tar.gz file");
                let tar_gz = File::create(temp_file.as_path()).map_err(|e| JobRunError {
                    error: ArchiveJobRunError::OpenTempFileError(e),
                })?;

                log::debug!("creating gz encoder");
                let enc = GzEncoder::new(tar_gz, Compression::default());

                log::debug!("filling gzipped tarball with directory contents");
                let mut tar = tar::Builder::new(enc);

                tar.follow_symlinks(true);

                tar.append_dir_all(".", spec.src.clone())
                    .map_err(|e| JobRunError {
                        error: ArchiveJobRunError::AppendToTarError(e),
                    })?;
            } else {
                log::warn!("non-directory backups aren't supported yet.");
                return Ok(false);
            }
        }

        Ok(true)
    }
}
