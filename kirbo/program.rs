use std::env;

mod commands;
mod config;
mod lock;
mod npm;
mod options;
mod resolver;
mod semver;
mod workspace;
use options::Command::*;
use options::Options;

fn main() -> anyhow::Result<()> {
	let program = async {
		let options = Options::try_from(&*env::args().skip(1).collect::<Vec<_>>())?;

		match &options.command {
			Install => commands::install::main::main(options).await?,
			Run => commands::run::main::main(options),
			Exec => commands::exec::main::main(options),
		}

		Ok(())
	};

	#[cfg(not(target_os = "wasi"))]
	return tokio::runtime::Builder::new_multi_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(program);

	#[cfg(target_os = "wasi")]
	return tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap()
		.block_on(program);
}
