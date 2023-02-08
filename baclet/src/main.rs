use std::{thread, path::PathBuf, collections::HashMap};

use clap::{arg, value_parser, Command};

use crate::{schema::job::JobType, destinations::schema::DestinationSpec, job::JobTypeImpl, archive::job::ArchiveJob};

pub mod job;
pub mod archive;
pub mod config;
pub mod schema;
pub mod destinations;

fn main() {
    let matches = Command::new("baclet")
        .version("0.1.0-dev")
        .author("Hayden Young <hayden@hbjy.dev>")
        .about("Runs the bacman agent, running backups locally on this system")
        .arg(
            arg!(-c --config <path> "Path to the config file")
                .value_parser(value_parser!(PathBuf))
                .default_value("/etc/baclet/config.yaml")
        )
        .arg(
            arg!(-v --verbose "Enable verbose logging")
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

    log::debug!("loading destination list");
    let mut destinations: HashMap<String, DestinationSpec> = HashMap::new();
    config.spec.destinations.iter().for_each(|ds| {
        log::debug!("loading destination \"{}\"", ds.name.clone());
        destinations.insert(ds.name.clone(), ds.clone());
    });

    let mut threads: Vec<_> = Vec::new();

    for job in config.spec.jobs.iter() {
        let js = job.clone();

        log::info!("starting job \"{}\"", js.name);

        let job = match js.job_spec {
            JobType::ArchiveJob(_) =>
                ArchiveJob::init(js.clone(), destinations.clone()),
        };

        let a = thread::spawn(move || {
            match JobTypeImpl::run(&job) {
                Ok(_) => log::info!("backup job \"{}\" finished", js.name),
                Err(e) => {
                    log::error!("failed to run backup \"{}\": {:?}", js.name, e);
                    std::process::exit(1);
                }
            };
        });
        threads.push(a);
    }

    for thread in threads {
        thread.join().unwrap();
    }
}
