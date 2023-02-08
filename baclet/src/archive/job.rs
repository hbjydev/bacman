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
    TempDirCreateError(io::Error),
    FileMetadataError(io::Error),
    CreateFileError(io::Error),
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
        if let None = maybe_dest {
            return Err(JobRunError {
                error: ArchiveJobRunError::MissingDestinationError,
            });
        } else {
            let dest = maybe_dest.unwrap();
            log::debug!("Shipping backup to destination \"{}\"", dest.name);
            let JobType::ArchiveJob(spec) = &self.spec.job_spec;

            // Generate temporary directory
            // TODO Make this base path configurable
            let temp_file = Temp::new_dir_in("/tmp/baclet/backup").map_err(|e| JobRunError {
                error: ArchiveJobRunError::TempDirCreateError(e),
            })?;

            // Check if the destination is a directory (and if it is accessible)
            let is_dir = self.is_dir(spec.clone()).map_err(|e| JobRunError {
                error: ArchiveJobRunError::FileMetadataError(e),
            })?;

            // If it's a directory, create an archive of the entire directory and store it in the
            // temporary directory we just created.
            if is_dir {
                log::debug!("creating tar.gz file");
                let tar_gz =
                    File::create(format!("{}/{}.tgz", temp_file.display(), &self.spec.name))
                        .map_err(|e| JobRunError {
                            error: ArchiveJobRunError::CreateFileError(e),
                        })?;

                log::debug!("creating gz encoder");
                let enc = GzEncoder::new(tar_gz, Compression::default());

                log::debug!("filling gzipped tarball with directory contents");
                let mut tar = tar::Builder::new(enc);
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
