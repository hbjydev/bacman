use std::path::PathBuf;

use clap::{arg, value_parser, Command};

pub mod archive;
pub mod config;

fn main() {
    let matches = Command::new("baclet")
        .version("0.1.0-dev")
        .author("Hayden Young <hayden@hbjy.dev>")
        .about("Runs the bacman agent, running backups locally on this system")
        .arg(
            arg!(-c --config <path> "Path to the config file (default: /etc/baclet/config.yaml)")
                .value_parser(value_parser!(PathBuf))
                .default_value("/etc/baclet/config.yaml")
        )
        .arg(
            arg!(-v --verbose "Enable verbose logging (default: false)")
                .default_value("false")
        )
        .get_matches();

    let config_path = matches.get_one::<PathBuf>("config").expect("required");
    let verbose = matches.get_one::<bool>("verbose").expect("required");

    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(match verbose {
            true => "debug",
            false => "info"
        })
    ).init();

    log::info!("baclet v0.1.0-dev");
    log::info!("config file: {}", config_path.display());
    log::info!("verbose: {}", verbose);

    log::debug!("loading config file");

    let config = match config::BacletConfig::from_file(config_path) {
        Ok(c) => c,
        Err(e) => {
            match e {
                config::BacletConfigInitError::FileError(e) => {
                    log::error!("failed to read config file: {}", e.to_string());
                    std::process::exit(1);
                },

                config::BacletConfigInitError::DeserializeError(e) => {
                    log::error!("failed to parse config file: {}", e.to_string());
                    std::process::exit(1);
                },
            }
        },
    };

    log::debug!("config: {:?}", config);

    if let Some(job_spec) = config.spec.jobs.get(0) {
        let job = archive::job::ArchiveJob::init(
            job_spec.archive_job.clone().unwrap()
        );

        match job.run() {
            Ok(v) => log::info!("archive job finished?: {}", v),
            Err(e) => {
                match e {
                    archive::job::ArchiveJobRunError::CreateFileError(e) => {
                        log::error!("failed to create archive placeholder: {}", e);
                        std::process::exit(1);
                    },

                    archive::job::ArchiveJobRunError::FileMetadataError(e) => {
                        log::error!("failed to retrieve metadata for source: {}", e);
                        std::process::exit(1);
                    },

                    archive::job::ArchiveJobRunError::AppendToTarError(e) => {
                        log::error!("failed to fill the archive: {}", e);
                        std::process::exit(1);
                    },
                }
            }
        };
    }
}
