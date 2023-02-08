use serde::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

#[derive(Debug)]
pub struct JobRunError<E> {
    pub error: E,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum JobType {
    #[serde(rename = "archiveJob")]
    ArchiveJob(crate::archive::schema::ArchiveJobSpec),
}

/// A specification for a backup job
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobSpec {
    /// The programmatic name of the job
    pub name: String,

    /// The programmatic name of the destination to send the backup to.
    pub destination: String,

    /// The archive job to run
    #[serde(flatten)]
    pub job_spec: JobType,

    /// The cron-syntax schedule to run the job on
    pub schedule: String,
}

