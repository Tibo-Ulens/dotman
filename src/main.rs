//!      _       _
//!   __| | ___ | |_ _ __ ___   __ _ _ __
//!  / _` |/ _ \| __| '_ ` _ \ / _` | '_ \
//! | (_| | (_) | |_| | | | | | (_| | | | |
//!  \__,_|\___/ \__|_| |_| |_|\__,_|_| |_|
//!
//! # Dotfile management tool

use std::process;

use ansi_term::Colour::Red;
use anyhow::Error;
use dotman;
use dotman::Config;

/// Neatly print out an error and then exit
fn print_err_and_exit(err: Error) -> ! {
	eprintln!("{} {}", Red.bold().paint("error:"), err);
	err.chain().skip(1).for_each(|cause| {
		eprintln!("  {} {}", Red.bold().paint("caused by:"), cause);
	});

	process::exit(1)
}

fn main_() -> Result<(), Error> {
	let matches = dotman::get_args();

	let mut config = Config::from_file()?;

	match matches.subcommand() {
		Some((cmd, sub_matches)) if cmd == "gather" || cmd == "scatter" || cmd == "restore" => {
			let excluded: Vec<String> = match sub_matches.values_of_t("excluded") {
				Ok(val) => Ok(val),
				Err(err) => {
					match err.kind() {
						// No argument is not an error, just means the list
						// is empty
						clap::ErrorKind::ArgumentNotFound => Ok(vec![]),
						_ => Err(err),
					}
				},
			}?;

			let only: Vec<String> = match sub_matches.values_of_t("only") {
				Ok(val) => Ok(val),
				Err(err) => {
					match err.kind() {
						// No argument is not an error, just means the list
						// is empty
						clap::ErrorKind::ArgumentNotFound => Ok(vec![]),
						_ => Err(err),
					}
				},
			}?;

			config.finalise_file_map(&only, &excluded);

			match cmd {
				"gather" => dotman::gather(&config)?,
				// "scatter" => (),
				// "restore" => (),
				_ => unreachable!("Prevented by match guard"),
			}
		},
		_ => unreachable!("Prevented by subcommand_required"),
	}

	Ok(())
}

fn main() {
	if let Err(err) = main_() {
		print_err_and_exit(err.into());
	}
}
