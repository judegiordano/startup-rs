use std::{env, process, str::FromStr};

pub struct Config {
    pub port: u16,
    pub log_level: tracing::Level,
}

impl Config {
    pub fn get_required(key_name: &str) -> String {
        match env::var(key_name) {
            Ok(found) => found,
            Err(e) => {
                eprintln!("error getting env value for {key_name}: {}", e);
                process::exit(1)
            }
        }
    }

    #[allow(dead_code)]
    fn get_int<T: FromStr>(key_name: &str) -> T {
        match env::var(key_name) {
            Ok(found) => match found.parse::<T>() {
                Ok(int) => int,
                Err(_) => {
                    eprintln!("unable to parse int for variable {}", key_name);
                    process::exit(1)
                }
            },
            Err(e) => {
                eprintln!("error getting env value for {key_name}: {}", e);
                process::exit(1)
            }
        }
    }

    fn get_log_level(key_name: &str) -> tracing::Level {
        match Self::get_required(key_name).to_uppercase().as_ref() {
            "INFO" => tracing::Level::INFO,
            "DEBUG" => tracing::Level::DEBUG,
            "ERROR" => tracing::Level::ERROR,
            "WARN" => tracing::Level::WARN,
            "TRACE" => tracing::Level::TRACE,
            _ => tracing::Level::ERROR,
        }
    }

    fn default() -> Self {
        Self {
            port: Self::get_int("PORT"),
            log_level: Self::get_log_level("LOG_LEVEL"),
        }
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}
