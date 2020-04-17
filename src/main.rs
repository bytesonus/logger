#[macro_use]
extern crate lazy_static;
extern crate async_std;
extern crate clap;
extern crate futures;
extern crate juno;

mod cli_parser;
mod logger;

use logger::{LogLevel, Logger};

use std::{collections::HashMap, path::Path, time::Duration};

use async_std::task;
use clap::{crate_name, crate_version};
use juno::models::{Number, Value};

#[async_std::main]
async fn main() {
	let mut module = cli_parser::from_cli_args();
	module
		.initialize(crate_name!(), crate_version!(), HashMap::new())
		.await
		.unwrap();
	module
		.declare_function("setVerbosity", set_verbosity)
		.await
		.unwrap();
	module
		.declare_function("setLogDestination", set_log_destination)
		.await
		.unwrap();
	module
		.declare_function("verbose", verbose_log)
		.await
		.unwrap();
	module.declare_function("info", info_log).await.unwrap();
	module.declare_function("debug", debug_log).await.unwrap();
	module.declare_function("warn", warn_log).await.unwrap();
	module.declare_function("error", error_log).await.unwrap();
	loop {
		task::sleep(Duration::from_millis(1000)).await;
	}
}

fn set_verbosity(args: HashMap<String, Value>) -> Value {
	let verbosity = args.get("verbosity");
	if verbosity.is_none() {
		return Value::Null;
	}
	let verbosity = verbosity.unwrap();
	if !verbosity.is_number() {
		return Value::Null;
	}
	let verbosity = match verbosity.as_number().unwrap() {
		Number::Decimal(num) => *num as u8,
		Number::SignedInteger(num) => *num as u8,
		Number::UnsignedInteger(num) => *num as u8,
	};
	let mut mutex = logger::LOGGER.lock().unwrap();
	mutex.set_verbosity(match verbosity {
		5 => LogLevel::Error,
		4 => LogLevel::Warn,
		3 => LogLevel::Debug,
		2 => LogLevel::Info,
		_ => LogLevel::Verbose,
	});

	Value::Null
}

fn set_log_destination(args: HashMap<String, Value>) -> Value {
	let destination = args.get("destination");
	if destination.is_none() {
		return Value::Null;
	}
	let destination = destination.unwrap();
	if !destination.is_string() {
		return Value::Null;
	}
	let destination = destination.as_string().unwrap();
	let mut verbosity = args.get("verbosity");
	if verbosity.is_none() {
		verbosity = Some(&Value::Number(Number::UnsignedInteger(1)));
	}
	let mut verbosity = verbosity.unwrap();
	if !verbosity.is_number() {
		verbosity = &Value::Number(Number::UnsignedInteger(1));
	}
	let verbosity = match verbosity.as_number().unwrap() {
		Number::Decimal(num) => *num as u8,
		Number::SignedInteger(num) => *num as u8,
		Number::UnsignedInteger(num) => *num as u8,
	};
	let verbosity = match verbosity {
		5 => LogLevel::Error,
		4 => LogLevel::Warn,
		3 => LogLevel::Debug,
		2 => LogLevel::Info,
		_ => LogLevel::Verbose,
	};
	let path = Path::new(destination);

	let new_logger = if path.is_file() {
		Logger::File {
			verbosity,
			file_path: destination.clone(),
		}
	} else if path.is_dir() {
		Logger::Dir {
			verbosity,
			dir_path: destination.clone(),
		}
	} else if destination == "none" {
		Logger::None
	} else {
		Logger::Console { verbosity }
	};

	let mut logger = logger::LOGGER.lock().unwrap();
	*logger = new_logger;

	Value::Null
}

fn verbose_log(args: HashMap<String, Value>) -> Value {
	let data = args.get("data");
	if data.is_none() {
		return Value::Null;
	}
	let data = data.unwrap();
	if !data.is_string() {
		return Value::Null;
	}
	let data = data.as_string().unwrap();

	logger::LOGGER.lock().unwrap().verbose(data);

	Value::Null
}

fn info_log(args: HashMap<String, Value>) -> Value {
	let data = args.get("data");
	if data.is_none() {
		return Value::Null;
	}
	let data = data.unwrap();
	if !data.is_string() {
		return Value::Null;
	}
	let data = data.as_string().unwrap();

	logger::LOGGER.lock().unwrap().info(data);

	Value::Null
}

fn debug_log(args: HashMap<String, Value>) -> Value {
	let data = args.get("data");
	if data.is_none() {
		return Value::Null;
	}
	let data = data.unwrap();
	if !data.is_string() {
		return Value::Null;
	}
	let data = data.as_string().unwrap();

	logger::LOGGER.lock().unwrap().debug(data);

	Value::Null
}

fn warn_log(args: HashMap<String, Value>) -> Value {
	let data = args.get("data");
	if data.is_none() {
		return Value::Null;
	}
	let data = data.unwrap();
	if !data.is_string() {
		return Value::Null;
	}
	let data = data.as_string().unwrap();

	logger::LOGGER.lock().unwrap().warn(data);

	Value::Null
}

fn error_log(args: HashMap<String, Value>) -> Value {
	let data = args.get("data");
	if data.is_none() {
		return Value::Null;
	}
	let data = data.unwrap();
	if !data.is_string() {
		return Value::Null;
	}
	let data = data.as_string().unwrap();

	logger::LOGGER.lock().unwrap().error(data);

	Value::Null
}
