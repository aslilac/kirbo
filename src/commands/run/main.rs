use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::npm;
use crate::options::Options;

pub fn main(options: Options) {
	let args = options.remaining_args;

	let package = serde_json::from_str::<npm::PackageJson>(
		&fs::read_to_string(PathBuf::from(&args[0]).join("package.json")).unwrap(),
	)
	.unwrap();

	let path_env = format!(
		"{}:{}",
		env::current_dir()
			.unwrap()
			.join(&args[0])
			.join("node_modules/.bin/")
			.display(),
		env::var("PATH").unwrap()
	);

	Command::new("sh")
		.arg("-c")
		.arg(&package.scripts[&args[1]])
		.env("PATH", path_env)
		.current_dir(&args[0])
		.spawn()
		.unwrap()
		.wait()
		.unwrap();
}
