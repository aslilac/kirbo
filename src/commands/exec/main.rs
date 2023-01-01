use colored::Colorize;
use std::env;
use std::process::Command;

use crate::options::Options;

pub fn main(options: Options) {
	println!("{}", "kirbo exec".bright_magenta().bold());
	let args = options.remaining_args;

	let path_env = format!(
		"{}:{}",
		env::current_dir()
			.unwrap()
			.join("node_modules/.bin/")
			.display(),
		env::var("PATH").unwrap()
	);

	Command::new(&args[0])
		.args(&args[1..])
		.env("PATH", path_env)
		.spawn()
		.unwrap()
		.wait()
		.unwrap();
}
