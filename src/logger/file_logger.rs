use super::LogLevel;
use async_std::{fs::OpenOptions, io::prelude::*, task};

pub(super) fn write(verbosity: u8, file_path: String, log_level: LogLevel, data: String) {
	task::spawn(async move {
		let myself = verbosity as u8;
		let other = log_level as u8;
		if myself <= other {
			// Do the actual logging here
			let result = OpenOptions::new().append(true).open(file_path).await;
			if result.is_err() {
				return;
			}
			let result = result
				.unwrap()
				.write_all(format!("[{}]: {}\n", log_level.to_string(), data).as_bytes())
				.await;
			if let Err(err) = result {
				println!("Unable to write to file: {}", err);
			}
		}
	});
}
