use crate::config::AppConfig;

pub mod config;
pub mod context;
pub mod error;
pub mod logger;
pub mod math {
    pub mod angle;
}
pub mod models;

pub mod transformations {
    pub mod affine;
    pub mod euclidean;
    pub mod projective;
    pub mod resize;
}
pub mod ui {
    pub mod app_model;
    pub mod core;
    pub mod windows;

    pub mod components;
}
pub mod utils {
    pub mod egui;
}

fn main() {
    let app_config = AppConfig::from_env().unwrap_or_else(|err| {
        println!("Error: {err}");
        std::process::exit(1);
    });

    let log_file_name = logger::generate_log_name(app_config.name.clone());
    logger::init(app_config.log_level, &log_file_name).unwrap_or_else(|err| {
        println!("Logger initialization failed. Error: {}", err);
        std::process::exit(1);
    });

    ui::core::start(app_config.name, app_config.theme).unwrap_or_else(|err| {
        log::error!("{}", err);
        std::process::exit(1);
    });
}
