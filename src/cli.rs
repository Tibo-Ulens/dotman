//! Command line argument parsing

use clap::{Arg, ArgMatches, Command};

/// Read and validate command line arguments
#[inline(always)]
pub fn get_args() -> ArgMatches {
	Command::new(env!("CARGO_PKG_NAME"))
		.version(env!("CARGO_PKG_VERSION"))
		.author(env!("CARGO_PKG_AUTHORS"))
		.about(env!("CARGO_PKG_DESCRIPTION"))
		.arg_required_else_help(true)
		.propagate_version(true)
		.subcommand_required(true)
		.subcommand(
			Command::new("gather")
				.about("Gather all dotfiles listed in the config file to the destination directory")
				.arg(
					Arg::new("excluded")
						.short('e')
						.long("exclude")
						.help("A list of categories to exclude")
						.conflicts_with("only")
						.ignore_case(true)
						.takes_value(true)
						.multiple_values(true)
						.require_value_delimiter(true)
						.multiple_occurrences(true),
				)
				.arg(
					Arg::new("only")
						.short('o')
						.long("only")
						.help("Only gather these categories")
						.conflicts_with("excluded")
						.ignore_case(true)
						.takes_value(true)
						.multiple_values(true)
						.require_value_delimiter(true)
						.multiple_occurrences(true),
				),
		)
		.subcommand(
			Command::new("scatter")
				.about("Move all dotfiles listed in the config file to their respective origin")
				.long_about(
					"Move all dotfiles listed in the config file to their respective origin\nAlso \
					 makes a backup of all old dotfiles in the form of <oldfile>.old",
				)
				.arg(
					Arg::new("excluded")
						.short('e')
						.long("exclude")
						.help("A list of categories to exclude")
						.conflicts_with("only")
						.ignore_case(true)
						.takes_value(true)
						.multiple_values(true)
						.require_value_delimiter(true)
						.multiple_occurrences(true),
				)
				.arg(
					Arg::new("only")
						.short('o')
						.long("only")
						.help("Only scatter these categories")
						.conflicts_with("excluded")
						.ignore_case(true)
						.takes_value(true)
						.multiple_values(true)
						.require_value_delimiter(true)
						.multiple_occurrences(true),
				),
		)
		.subcommand(
			Command::new("restore")
				.about("Undo the last scattering by restoring the old dotfiles")
				.arg(
					Arg::new("excluded")
						.short('e')
						.long("exclude")
						.help("A list of categories to exclude")
						.conflicts_with("only")
						.ignore_case(true)
						.takes_value(true)
						.multiple_values(true)
						.require_value_delimiter(true)
						.multiple_occurrences(true),
				)
				.arg(
					Arg::new("only")
						.short('o')
						.long("only")
						.help("Only restore these categories")
						.conflicts_with("excluded")
						.ignore_case(true)
						.takes_value(true)
						.multiple_values(true)
						.require_value_delimiter(true)
						.multiple_occurrences(true),
				),
		)
		.subcommand(
			Command::new("add")
				.about("Add a new dotfile to the config under the specified category")
				.arg(Arg::new("targets").required(true).min_values(1)),
		)
		.subcommand(
			Command::new("remove")
				.about("Remove a dotfile or category from the config")
				.arg(Arg::new("targets").required(true).min_values(1)),
		)
		.subcommand(
			Command::new("destination")
				.about("Set the directory into which dotfiles will be gathered")
				.arg(Arg::new("dest").required(true).max_values(1)),
		)
		.get_matches()
}
