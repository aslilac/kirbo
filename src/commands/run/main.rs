use colored::Colorize;
use std::env;
use std::fs;
use std::process::Command;

use crate::npm;
use crate::options::Options;

pub fn main(options: Options) {
	println!("{}", "kirbo run".bright_magenta().bold());
	let args = options.remaining_args;

	let package = serde_json::from_str::<npm::PackageJson>(
		&fs::read_to_string(env::current_dir().unwrap().join("package.json")).unwrap(),
	)
	.unwrap();

	let path_env = format!(
		"{}:{}",
		env::current_dir()
			.unwrap()
			.join("node_modules/.bin/")
			.display(),
		env::var("PATH").unwrap()
	);

	Command::new("sh")
		.arg("-c")
		.arg(&package.scripts[&args[0]])
		.args(&args[1..])
		.env("PATH", path_env)
		.spawn()
		.unwrap()
		.wait()
		.unwrap();
}
