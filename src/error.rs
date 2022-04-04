//! Custom error types

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Could not find destination directory")]
	DestinationNotFound,
	#[error("Could not read file `{file}`")]
	FileNotFound { source: std::io::Error, file: String },
	#[error(transparent)]
	IOError(#[from] std::io::Error),
	#[error("Could not parse config file")]
	ConfigError {
		#[from]
		source: toml::de::Error,
	},
	#[error("Could not parse arguments")]
	ArgParseError {
		#[from]
		source: clap::Error,
	},
}
