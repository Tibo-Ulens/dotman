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
		let config_home = env::var("XDG_CONFIG_HOME").unwrap_or(env::var("HOME").unwrap());

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

	/// Handle exclude and only options
	///
	/// ## Arguments
	///
	/// - only_list: A list of the only categories to be processed, ignored if empty
	/// - excluded_list: A list of categories to ignore, ignored if empty
	#[inline(always)]
	pub fn finalise_file_map(&mut self, only_list: &[String], excluded_list: &[String]) {
		let mut to_be_removed = Vec::<String>::new();

		if only_list.len() != 0 {
			for (category, _) in self.dotfiles.iter() {
				// If the only_list does not contain a category, it should be excluded
				if !(only_list.contains(category)) {
					to_be_removed.push(category.to_string());
				}
			}
		}

		if excluded_list.len() != 0 {
			for (category, _) in self.dotfiles.iter() {
				// If the excluded_list contains a category, it should be excluded
				if excluded_list.contains(category) {
					to_be_removed.push(category.to_string());
				}
			}
		}

		for category in to_be_removed {
			self.dotfiles.remove(&category);
		}
	}
}
