use crate::error::logger::LoggerError;
use chrono::{Datelike, Local};
use log::LevelFilter;

pub fn init(log_level: LevelFilter, file_name: &str) -> Result<(), LoggerError> {
    let file = fern::log_file(file_name).map_err(|_| LoggerError::CannotAccessLogFile)?;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log_level)
        .chain(fern::Output::file(file, "\r\n"))
        .apply()
        .map_err(LoggerError::SetLoggerError)
}

pub fn generate_log_name(crate_name: String) -> String {
    let now = Local::now();
    let date = format!(
        "{year:04}-{month:02}-{day:02}",
        year = now.year(),
        month = now.month(),
        day = now.day(),
    );

    let crate_name_formatted = crate_name.replace(" ", "-");
    format!("{crate_name_formatted}_{date}.log")
}
