use log::LevelFilter;

pub mod error;
pub mod logger;
pub mod ui;

pub const LOG_FILE_NAME: &str = "log.txt";
pub const LOG_LEVEL: LevelFilter = LevelFilter::Info;

fn main() {
    logger::init(LOG_LEVEL, LOG_FILE_NAME).unwrap_or_else(|err| {
        println!("Logger initialization failed. Error: {}", err);
        std::process::exit(1);
    });

    ui::core::start().unwrap_or_else(|err| {
        log::error!("{}", err);
        std::process::exit(1);
    });
}
