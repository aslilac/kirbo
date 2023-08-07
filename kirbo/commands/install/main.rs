use colored::Colorize;
use std::collections::HashMap;
use std::env;
use std::fs;

use super::options::Options;
use crate::npm::PackageJson;
use crate::options;
use crate::resolver::Resolver;

pub async fn main(options: options::Options) -> anyhow::Result<()> {
	println!("{}", "kirbo install".bright_magenta().bold());

	let options = Options::try_from(&*options.remaining_args)?;
	let package = serde_json::from_str::<PackageJson>(
		&fs::read_to_string(env::current_dir().unwrap().join("package.json")).unwrap(),
	)
	.unwrap();

	println!("{:?}", &options.packages_to_add);

	println!("========================================");
	println!(
		"{}@{}",
		package.name.as_ref().unwrap(),
		package.version.as_ref().unwrap()
	);
	println!("========================================\n\n\n");

	let joined_dependencies = package
		.dependencies
		.clone()
		.into_iter()
		.chain(package.dev_dependencies.clone())
		.collect::<HashMap<_, _>>();

	let mut resolver = Resolver::default();
	let installed_dependencies = resolver
		.resolve_dependencies(&joined_dependencies, 0)
		.await?;

	println!("========================================");
	println!("summary:");
	println!("  total dependencies: {}", installed_dependencies.len());
	println!("========================================");

	println!("{:#?}", installed_dependencies);

	Ok(())
}
