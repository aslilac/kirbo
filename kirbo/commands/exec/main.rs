use colored::Colorize;
use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

use crate::options::Options;

pub fn main(options: Options) {
	println!("{}", "kirbo exec".bright_magenta().bold());
	let args = options.remaining_args;

	let mut path_env = env::current_dir()
		.unwrap_or(PathBuf::from("."))
		.join("node_modules/.bin/")
		.into_os_string();

	if let Some(path) = env::var_os("PATH") {
		path_env.extend([OsStr::new(":"), &path]);
	};

	Command::new(&args[0])
		.args(&args[1..])
		.env("PATH", path_env)
		.spawn()
		.unwrap()
		.wait()
		.unwrap();
}
