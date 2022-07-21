//! Gathering dotfiles into a central location

use std::fs;

use ansi_term::Colour::Blue;
use anyhow::{Context, Error};

use super::Config;

/// Gather all dotfiles in the config into the destination directory
pub fn gather(cfg: &Config) -> Result<(), Error> {
	println!("Gathering dotfiles to {}", Blue.bold().paint(cfg.destination.display().to_string()));

	for (category, files) in &cfg.dotfiles {
		println!(
			"\n{} Gathering dotfiles for {}",
			Blue.bold().paint("::"),
			Blue.bold().paint(category),
		);

		let category_path = cfg.destination.join(category);

		// Clear out the directory in case the files in a category changed
		if category_path.exists() {
			fs::remove_dir_all(&category_path).with_context(|| {
				format!("Could not clear directory {}", category_path.display())
			})?;

			fs::create_dir(&category_path).with_context(|| {
				format!("Could not create directory {}", category_path.display())
			})?;
		} else {
			fs::create_dir_all(&category_path).with_context(|| {
				format!("Could not create directory {}", category_path.display())
			})?;
		}

		for file in files {
			let destination = category_path.join(file.file_name().unwrap());

			println!("    {} -> {}", file.display(), destination.display());

			fs::copy(file, &destination).with_context(|| {
				format!("Could not copy {} to {}", file.display(), destination.display())
			})?;
		}
	}

	Ok(())
}
