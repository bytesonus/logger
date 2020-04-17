use colored::*;

use super::LogLevel;

pub(super) fn write(verbosity: LogLevel, log_level: LogLevel, data: &str) {
	let myself = verbosity as u8;
	let other = log_level as u8;
	if myself <= other {
		let log_level = match log_level {
			LogLevel::Verbose => log_level.to_string().green(),
			LogLevel::Info => log_level.to_string().blue(),
			LogLevel::Debug => log_level.to_string().yellow(),
			LogLevel::Warn => log_level.to_string().on_yellow().black(),
			LogLevel::Error => log_level.to_string().on_red().white(),
		};
		println!("[{}]: {}", log_level, data);
	}
}
