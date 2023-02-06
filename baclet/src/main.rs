use std::path::PathBuf;

use clap::{arg, value_parser, Command};

pub mod job;
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

    config.spec.jobs.iter().for_each(|js| {
        log::info!("starting job \"{}\"", js.name);
        let job = match js.job_spec {
            config::BacletJobType::ArchiveJob(_) =>
                archive::job::ArchiveJob::init(js.clone()),
        };

        match job::JobType::run(&job) {
            Ok(_) => log::info!("backup job \"{}\" finished", js.name),
            Err(e) => {
                log::error!("failed to run backup \"{}\": {:?}", js.name, e);
                std::process::exit(1);
            }
        };
    });
}
