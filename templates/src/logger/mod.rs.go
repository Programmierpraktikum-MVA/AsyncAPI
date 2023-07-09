use std::str::FromStr;
use log::LevelFilter;

#[derive(Debug)]
pub enum LogLevelParseError {
    InvalidLogLevel,
}

pub struct LogLevel(LevelFilter);

impl FromStr for LogLevel {
    type Err = LogLevelParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let level = match s {
            "OFF" => Ok(LevelFilter::Off),
            "ERROR" => Ok(LevelFilter::Error),
            "WARN" => Ok(LevelFilter::Warn),
            "INFO" => Ok(LevelFilter::Info),
            "DEBUG" => Ok(LevelFilter::Debug),
            "TRACE" => Ok(LevelFilter::Trace),
            _ => Err(LogLevelParseError::InvalidLogLevel),
        }?;
        Ok(LogLevel(level))
    }
}

pub fn init_logger(log_level: &str) {
    let log_level: LevelFilter = match log_level.parse::<LogLevel>() {
        Ok(LogLevel(level)) => level,
        Err(_) => panic!("Invalid log level"),
    };
    env_logger::Builder::new().filter(None, log_level).init();
}