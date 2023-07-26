use colored::Colorize;
use std::borrow::Cow;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
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

	if args.len() == 0 {
		println!("Available scripts:");
		for (name, script) in package.scripts {
			println!("\n  - {}\n    {}\n", name.bold(), script);
		}
		return;
	}

	let mut path_env = env::current_dir()
		.unwrap_or(PathBuf::from("."))
		.join("node_modules/.bin/")
		.into_os_string();

	if let Some(path) = env::var_os("PATH") {
		// TODO: Windows ; `env::join_paths` blah blah
		path_env.extend([OsStr::new(":"), &path]);
	};

	let mut script = Cow::from(package.scripts[&args[0]].clone());
	// Pass in additional arguments. I always thought this npm/yarn behavior felt hacky,
	// but I like it even less after seeing what it takes to implement it.
	for it in 1..args.len() {
		script.to_mut().extend([format!(" ${}", it - 1)]);
	}

	Command::new("sh")
		.arg("-c")
		.arg(&*script)
		.args(&args[1..])
		.env("PATH", path_env)
		.spawn()
		.unwrap()
		.wait()
		.unwrap();
}
