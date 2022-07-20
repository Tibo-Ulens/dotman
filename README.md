# DOTfile MANager

Command line tool to easily move dotfiles to a central location (eg. for pushing to github)
and to move dotfiles to their final location.

## Configuration

Dotmans configuration file is located in `${XDG_CONFIG_HOME:-$HOME/.config}/dotman/config`
and is written according to the TOML format.\
eg.
```toml
destination = "/path/to/dotfile/destination/directory"

[files]
bash = [ "/home/foo/.bashrc", "/home/foo/.bash_aliases" ]
i3 = [ "/home/foo/.config/i3/config" ]
```

This file can be modified directly using a text editor, or using the commands `dotman add/remove/destination`
(see [Usage](##Usage)).

Furthermore it is also possible to provide dotman with a list of hooks in the directory
`{config_dir}/hooks/`, see [Hooks](##Hooks) for more info on hooks.

## Usage

### Collecting dotfiles

Use `dotman gather [ -e/--exclude category... ] [ -o/--only category... ]`
to gather all configured dotfiles into the configured destination directory.
The files will be grouped into subdirectories based on their categories.

Any categories specified after the `--exclude` option will not be gathered.\
If the `--only` argument is given only the specified categories will be gathered,
if a specified category is not listed in the config file an error will be returned.

### Installing dotfiles

Use `dotman scatter [ -e/--exclude category... ] [ -o/--only category... ]`
to move all configured dotfiles from the collection directory ino their final location.

This command will also create a backup of any replaced dotfiles by renaming the file to
`<dotfile>.old`, these can then later be restored using `dotman restore`.

Any categories specified after the `--exclude` option will not be copied.\
If the `--only` argument is given only the specified categories will be copied,
if a specified category is not listed in the config file an error will be returned.

### Restoring old dotfiles

Use `dotman restore [ -e/--exclude category... ] [ -o/--only category... ]`
to restore all dotfiles from their respective `.old` files.

Any categories specified after the `--exclude` option will not be restored.\
If the `--only` argument is given only the specified categories will be restored,
if a specified category is not listed in the config file an error will be returned.

### Editing the configuration

Use `dotman add <category> <path/to/file>` to add a dotfile to a given category,
if the category does not exist it will be created.

Use `dotman remove { <path/to/file> | <category> }` to remove either a specific dotfile
or an entire category from the config. Does nothing if the file/category aren't in the config.

Use `dotman destination <path/to/dotfile/destination/directory>` to set the directory into which
dotfiles will be gathered. Will throw an error if the directory doesn't exist

## Hooks

Dotman can execute user-specified commands called 'hooks' before or after either gathering or scattering.
These hooks are listed in `{config_dir}/hooks/` and take the following format:
```txt
Description = <description of the hook>
When = [ PreGather | PostGather | PreScatter | PostScatter ]
Exec = <command to execute>
```
