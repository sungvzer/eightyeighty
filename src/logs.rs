use fern::colors::{Color, ColoredLevelConfig};
use log::Level;

pub fn log_init(log_level: Level) -> Result<(), log::SetLoggerError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .debug(Color::Blue)
        .error(Color::Red);

    let today = chrono::Local::now();
    let today = today.format("%Y-%m-%d");
    let filename = format!("log-{today}.log");

    let stdout_dispatcher = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}:{}][{}] {}",
                chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout());

    let file_dispatcher = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}:{}][{}] {}",
                chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(filename).unwrap());

    fern::Dispatch::new()
        .level(log_level.to_level_filter())
        .chain(stdout_dispatcher)
        .chain(file_dispatcher)
        .apply()?;
    Ok(())
}
