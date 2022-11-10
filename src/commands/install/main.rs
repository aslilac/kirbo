use std::collections::HashMap;
use std::fs;

use super::options::Options;
use crate::npm::PackageJson;
use crate::options;
use crate::resolver::Resolver;

pub async fn main(options: options::Options) {
	let options = options.remaining_args.into_iter().collect::<Options>();
	let package = serde_json::from_str::<PackageJson>(
		&fs::read_to_string(options.input.join("package.json")).unwrap(),
	)
	.unwrap();

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
		.unwrap_or_default()
		.into_iter()
		.chain(package.dev_dependencies.clone().unwrap_or_default())
		.collect::<HashMap<_, _>>();

	let mut resolver = Resolver::default();
	let installed_dependencies = resolver
		.resolve_dependencies(
			&joined_dependencies,
			package.name.as_ref().map(String::as_ref).unwrap_or("."),
			0,
		)
		.await;

	println!("========================================");
	println!("summary:");
	println!("  total dependencies: {}", installed_dependencies.len());
	println!("========================================");

	println!("{:#?}", installed_dependencies);
}
