use std::sync::Mutex;

mod console_logger;
mod dir_logger;
mod file_logger;

lazy_static! {
	pub(crate) static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::default());
}

#[derive(Clone, Copy)]
pub(crate) enum LogLevel {
	Verbose = 1,
	Info = 2,
	Debug = 3,
	Warn = 4,
	Error = 5,
}

impl LogLevel {
	pub fn to_string(&self) -> &str {
		match &self {
			LogLevel::Verbose => "VERBOSE",
			LogLevel::Info => "INFO",
			LogLevel::Debug => "DEBUG",
			LogLevel::Warn => "WARN",
			LogLevel::Error => "ERROR",
		}
	}
}

pub(crate) enum Logger {
	None,
	Console {
		verbosity: LogLevel,
	},
	File {
		verbosity: LogLevel,
		file_path: String,
	},
	Dir {
		verbosity: LogLevel,
		dir_path: String,
	},
}

impl Logger {
	pub(crate) fn default() -> Self {
		Logger::Console {
			verbosity: LogLevel::Verbose,
		}
	}

	pub(crate) fn verbose(&self, data: &str) {
		self.write(LogLevel::Verbose, data);
	}

	pub(crate) fn info(&self, data: &str) {
		self.write(LogLevel::Info, data);
	}

	pub(crate) fn debug(&self, data: &str) {
		self.write(LogLevel::Debug, data);
	}

	pub(crate) fn warn(&self, data: &str) {
		self.write(LogLevel::Warn, data);
	}

	pub(crate) fn error(&self, data: &str) {
		self.write(LogLevel::Error, data);
	}

	pub(crate) fn set_verbosity(&mut self, log_level: LogLevel) {
		match self {
			Logger::None => {}
			Logger::Console { ref mut verbosity } => {
				*verbosity = log_level;
			}
			Logger::File {
				ref mut verbosity, ..
			} => {
				*verbosity = log_level;
			}
			Logger::Dir {
				ref mut verbosity, ..
			} => {
				*verbosity = log_level;
			}
		}
	}

	pub(crate) fn write(&self, log_level: LogLevel, data: &str) {
		match self {
			Logger::None => {}
			Logger::Console { verbosity } => {
				console_logger::write(*verbosity, log_level, data);
			}
			Logger::File {
				verbosity,
				file_path,
			} => {
				file_logger::write(
					*verbosity as u8,
					file_path.clone(),
					log_level,
					String::from(data),
				);
			}
			Logger::Dir {
				verbosity,
				dir_path,
			} => {
				dir_logger::write(
					*verbosity as u8,
					dir_path.clone(),
					log_level,
					String::from(data),
				);
			}
		}
	}
}
