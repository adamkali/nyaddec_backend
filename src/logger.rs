use std::env;
use std::fs;
use log::{debug, warn, error, info, trace};

pub fn init() -> Result<(), fern::InitError> {
    // pull log level from env
    let log_level = env::var("LOG_LEVEL")
        .unwrap_or("DEBUG".into());
    let log_level = log_level
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);

    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!("[{}][{}][{}] {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.target(),
                record.level(),
                message    
            ))
        })
        .level(log_level)
        .chain(std::io::stderr());

    if let Ok(log_file) = env::var("LOG_LEVEL") {
        let log_file = fs::File::create(log_file)?;
        builder = builder.chain(log_file);
    }

    // globaly apply logger

    builder.apply()?;

    trace!("TRACE output enabled");
    debug!("DEBUG output enabled");
    info!("INFO output enabled");
    warn!("WARN output enabled");
    error!("ERROR output enabled");

    Ok(())
}