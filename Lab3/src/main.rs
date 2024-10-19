// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::AppConfig;

fn main() {
    let app_config = AppConfig::from_env().unwrap_or_else(|err| {
        println!("Error: {err}");
        std::process::exit(1);
    });

    logger::init(
        app_config.log_level,
        &logger::generate_log_name(&app_config.name),
    )
    .unwrap_or_else(|err| {
        println!("Logger initialization failed. Error: {}", err);
        std::process::exit(1);
    });

    ui::core::start(app_config.name, app_config.theme).unwrap_or_else(|err| {
        log::error!("{}", err);
        std::process::exit(1);
    });
}

pub mod config;
pub mod context;
pub mod errors {
    pub mod env;
    pub mod log;
}
pub mod geometry {
    pub mod line;
    pub mod moveable_point;
    pub mod point;
}
pub mod graphics {
    pub mod grid;
    pub mod screen;
}
pub mod logger;
pub mod math {
    pub mod angle;
}
pub mod models {
    pub mod bezier_curve;
    pub mod bezier_point;
    pub mod model;
}
pub mod operations {
    pub mod animation;
}
pub mod traits {
    pub mod positionable;
}
pub mod ui {
    pub mod app;
    pub mod core;
    pub mod components {
        pub mod canvas;
        pub mod settings;
    }
    pub mod styles {
        pub mod colors;
        pub mod strokes;
    }
    pub mod windows {
        pub mod main;
    }
}
