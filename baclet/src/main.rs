use std::{collections::HashMap, path::PathBuf, time::Duration, sync::mpsc};

use actix_web::{dev::ServerHandle, web, middleware, App, HttpServer, HttpResponse, http::header};
use clap::{arg, value_parser, Command};
use job_scheduler::{Job, JobScheduler};
use prometheus::{self, IntCounter, TextEncoder, Counter, Opts, register_int_counter};
use lazy_static::lazy_static;

lazy_static! {
    static ref BACLET_RUNS_TOTAL: IntCounter =
        register_int_counter!("baclet_runs_total", "the number of times the run loop has completed").unwrap();
}

use crate::{
    archive::job::ArchiveJob, destinations::schema::DestinationSpec, job::JobTypeImpl,
    schema::job::JobType,
};

pub mod archive;
pub mod config;
pub mod destinations;
pub mod job;
pub mod schema;

async fn metrics() -> HttpResponse {
    let encoder = TextEncoder::new();
    let mut buffer = String::new();
    encoder
        .encode_utf8(&prometheus::gather(), &mut buffer)
        .expect("failed to encode metrics");

    HttpResponse::Ok()
        .insert_header(header::ContentType::plaintext())
        .body(buffer)
}

async fn run_http(tx: mpsc::Sender<ServerHandle>) -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/metrics").to(metrics))
    })
    .bind(("0.0.0.0", 6969))?
    .workers(1)
    .run();

    let _ = tx.send(server.handle());

    server.await
}

fn main() {
    let matches = Command::new("baclet")
        .version("0.1.0-dev")
        .author("Hayden Young <hayden@hbjy.dev>")
        .about("Runs the bacman agent, running backups locally on this system")
        .arg(
            arg!(-c --config <path> "Path to the config file")
                .value_parser(value_parser!(PathBuf))
                .default_value("/etc/baclet/config.yaml"),
        )
        .arg(arg!(-v --verbose "Enable verbose logging").default_value("false"))
        .get_matches();

    let config_path = matches.get_one::<PathBuf>("config").expect("required");
    let verbose = matches.get_one::<bool>("verbose").expect("required");

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(match verbose {
        true => "debug",
        false => "info",
    }))
    .init();

    log::info!("baclet v0.1.0-dev");
    log::info!("config file: {}", config_path.display());
    log::info!("verbose: {}", verbose);

    log::debug!("loading config file");

    let config = match config::BacletConfig::from_file(config_path) {
        Ok(c) => c,
        Err(e) => match e {
            config::BacletConfigInitError::FileError(e) => {
                log::error!("failed to read config file: {}", e.to_string());
                std::process::exit(1);
            }

            config::BacletConfigInitError::DeserializeError(e) => {
                log::error!("failed to parse config file: {}", e.to_string());
                std::process::exit(1);
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

    let mut sched = JobScheduler::new();
    let (tx, rx) = mpsc::channel();
    let (stop_tx, stop_rx) = mpsc::channel();

    std::thread::spawn(move || {
        let server_future = run_http(tx);
        actix_web::rt::System::new().block_on(server_future)
    });

    let server_handle = rx.recv().unwrap();

    ctrlc::set_handler(move || {
        stop_tx.send(true).unwrap();
    })
    .expect("error setting ctrl+c handler");

    for job in config.spec.jobs.iter() {
        let js = job.clone();
        let job = match js.job_spec {
            JobType::ArchiveJob(_) => ArchiveJob::init(js.clone(), destinations.clone()),
        };

        sched.add(Job::new(js.schedule.parse().unwrap(), move || {
            log::info!("starting job \"{}\"", js.name);

            match JobTypeImpl::run(&job) {
                Ok(_) => log::info!("backup job \"{}\" finished", js.name),
                Err(e) => {
                    log::error!("failed to run backup \"{}\": {:?}", js.name, e);
                }
            };
        }));
    }

    loop {
        match stop_rx.try_recv() {
            Ok(_) => {
                actix_web::rt::System::new().block_on(server_handle.stop(true));
                break;
            },
            Err(e) => {
                match e {
                    mpsc::TryRecvError::Empty => (),
                    mpsc::TryRecvError::Disconnected => std::process::exit(1),
                }
            },
        };

        sched.tick();

        BACLET_RUNS_TOTAL.inc();

        std::thread::sleep(Duration::from_millis(500));
    }
}
