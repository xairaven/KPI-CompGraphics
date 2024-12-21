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
    pub mod file_loader;
    pub mod log;
    pub mod validation;
}
pub mod geometry {
    pub mod line2d;
    pub mod point2d;
}
pub mod graphics {
    pub mod grid;
    pub mod screen;
}
pub mod logger;
pub mod math {
    pub mod angle;
}
pub mod fractal {
    pub mod dot;
    pub mod examples;
    pub mod file_loader;
    pub mod model;
    pub mod state;
    pub mod system;
    pub mod validator;
}
pub mod ui {
    pub mod app;
    pub mod core;
    pub mod screenshot;
    pub mod components {
        pub mod canvas;
        pub mod settings;
    }
    pub mod styles {
        pub mod colors;
        pub mod strokes;
    }
    pub mod windows {
        pub mod ifs_settings;
        pub mod main;
        pub mod message;

        pub mod traits {
            pub mod window_ops;
        }
    }
}
