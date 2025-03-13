use lazy_static::lazy_static;
use std::sync::Mutex;
use std::fs::OpenOptions;
use std::io::Write;

pub enum LogLevel {
    Info,
    Error,
}

struct Logger {
    file_path: String,
}

impl Logger {
    fn new(file_path: &str) -> Self {
        Logger {
            file_path: file_path.to_string(),
        }
    }

    fn log(&self, level: LogLevel, message: &str) {
        let level_str = match level {
            LogLevel::Info => "INFO",
            LogLevel::Error => "ERROR",
        };

        let log_entry = format!("[{}] {}\n", level_str, message);

        print!("{}", log_entry);

        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
        {
            let _ = file.write_all(log_entry.as_bytes());
        }
    }
}

lazy_static! {
    static ref LOGGER: Mutex<Logger> = Mutex::new(
        Logger::new("application.log")
    );
}

pub fn info(message: &str) {
    LOGGER.lock().unwrap().log(LogLevel::Info, message);
}

pub fn error(message: &str) {
    LOGGER.lock().unwrap().log(LogLevel::Error, message);
}
