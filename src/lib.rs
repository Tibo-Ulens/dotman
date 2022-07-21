use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[macro_use]
extern crate serde_derive;

mod cli;
mod error;
mod gather;

pub use cli::get_args;
pub use error::Error;
pub use gather::gather;

/// Parsed content of the dotman config.toml file
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
	/// Where to save gathered dotfiles
	pub destination: PathBuf,
	/// List of managed dotfiles grouped by category
	pub dotfiles:    BTreeMap<String, Vec<PathBuf>>,
}

impl Config {
	/// Read and parse the dotman config file from its default location
	#[inline(always)]
	pub fn from_file() -> Result<Self, Error> {
		let config_home = env::var("XDG_CONFIG_HOME")
			.unwrap_or_else(|_| env::var("HOME").expect("HOME environment variable should be set"));

		let config_path = Path::new(&config_home).join("dotman").join("config.toml");

		let content = match fs::read_to_string(&config_path) {
			Ok(txt) => Ok(txt),
			Err(err) => {
				Err(Error::FileNotFound { source: err, file: config_path.display().to_string() })
			},
		}?;

		let config: Self = toml::from_str(&content)?;

		Ok(config)
	}

	/// Handle only option
	#[inline(always)]
	pub fn only(&mut self, only_list: &[String]) -> Result<(), Error> {
		let mut to_be_removed: Vec<String> = vec![];

		let mut accepted = 0usize;
		for category in self.dotfiles.keys() {
			if only_list.contains(category) {
				accepted += 1;
			} else {
				to_be_removed.push(category.to_string());
			}
		}

		if accepted < only_list.len() {
			let invalid = only_list.iter().find(|c| !(self.dotfiles.contains_key(*c)));
			return Err(Error::InvalidCategory(invalid.unwrap().to_string()));
		}

		for category in to_be_removed {
			self.dotfiles.remove(&category);
		}

		Ok(())
	}

	/// Handle exclude option
	#[inline(always)]
	pub fn exclude(&mut self, exclude_list: &[String]) -> Result<(), Error> {
		let mut to_be_removed: Vec<String> = vec![];

		for category in self.dotfiles.keys() {
			if exclude_list.contains(category) {
				to_be_removed.push(category.to_string());
			}
		}

		if to_be_removed.len() < exclude_list.len() {
			let invalid = exclude_list.iter().find(|c| !(self.dotfiles.contains_key(*c)));
			return Err(Error::InvalidCategory(invalid.unwrap().to_string()));
		}

		for category in to_be_removed {
			self.dotfiles.remove(&category);
		}

		Ok(())
	}
}
