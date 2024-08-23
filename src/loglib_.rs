use rustypath::RPath;
use chrono::Local;
use std::sync::{Arc, Mutex};
use std::io::{self, Write};
use std::fs::{File, OpenOptions};

use colorized::*;

#[derive(Clone, Debug)]
struct Log {
    file: Option<Arc<Mutex<File>>>,
}

enum LogLevel {
    Info,
    Warn,
    Error,
    Debug,
}

impl Log {
    fn prep(filepath: Option<RPath>) -> io::Result<Self> {
        let file = if let Some(path) = filepath {
            Some(Arc::new(Mutex::new(OpenOptions::new().append(true).create(true).open(path.convert_to_pathbuf())?)))
        } else {
            None
        };

        Ok(Log{file})
    }

    fn log(&self, level: LogLevel, content: &str, log_to_console: bool) {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let formatted_message = format!("{} - {:?} - {}\n", now, level, content);

        if let Some(file) = &self.file {
            let mut file = file.lock().unwrap();
            let _ = file.write_all(formatted_message.as_bytes());
        }

        if log_to_console {
            print!("{}", formatted_message);
        }
    }

    fn info(&self, content: &str, log_to_console: bool) {
        self.log(LogLevel::Info, content, log_to_console)
    }

    fn warn(&self, content: &str, log_to_console: bool) {
        self.log(LogLevel::Warn, content, log_to_console)
    }

    fn err(&self, content: &str, log_to_console: bool) {
        self.log(LogLevel::Error, content, log_to_console)
    }

    fn debug(&self, content: &str, log_to_console: bool) {
        self.log(LogLevel::Debug, content, log_to_console)
    }
}

impl std::fmt::Debug for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let level_str = match self {
            LogLevel::Info => "INFO".color(Colors::BrightBlueFg),
            LogLevel::Warn => "WARN".color(Colors::YellowFg),
            LogLevel::Error => "ERROR".color(Colors::RedFg),
            LogLevel::Debug => "DEBUG".color(Colors::BrightBlackFg),
        };
        write!(f, "{}", level_str)
    }
}

use pyo3::prelude::*;

#[pyclass]
pub struct Logger {
    logger: Log,
}

#[pymethods]
impl Logger {
    #[new]
    pub fn new(logfile: &str) -> Logger {
        Logger{logger: Log::prep(Some(RPath::from(logfile))).unwrap_or_else(|e| {
            eprintln!("Failed to initialize logger: {}", e);
            std::process::exit(1);
        })}
    }

    pub fn log(&self, content: &str, log_to_console: bool) {
        self.logger.log(LogLevel::Info, content, log_to_console)
    }

    pub fn info(&self, content: &str, log_to_console: bool) {
        self.logger.info(content, log_to_console)
    }

    pub fn warn(&self, content: &str, log_to_console: bool) {
        self.logger.warn(content, log_to_console)
    }

    pub fn err(&self, content: &str, log_to_console: bool) {
        self.logger.err(content, log_to_console)
    }

    pub fn debug(&self, content: &str, log_to_console: bool) {
        self.logger.debug(content, log_to_console)
    }
}