use std::env;

mod commands;
mod config;
mod lock;
mod npm;
mod options;
mod resolver;
mod semver;
use options::Command::*;
use options::Options;

#[tokio::main]
async fn main() {
	let options = env::args().skip(1).collect::<Options>();

	match &options.command {
		Install => commands::install::main::main(options).await,
		Run => commands::run::main::main(options),
		Exec => commands::exec::main::main(options),
	}
}
