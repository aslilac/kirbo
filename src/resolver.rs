use async_recursion::async_recursion;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::npm;

static CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

#[derive(Debug, Default)]
pub struct Resolver {
	package_docs: HashMap<String, npm::RegistryDoc>,
	panic: i32,
}

impl Resolver {
	pub async fn query_package(&mut self, package: &str) -> &npm::RegistryDoc {
		if !self.package_docs.contains_key(package) {
			print!("❄️ ");
			let doc = CLIENT
				.get(format!("https://registry.npmjs.org/{}", package))
				.send()
				.await
				.unwrap()
				.json::<npm::RegistryDoc>()
				.await
				.unwrap();

			self.package_docs.insert(package.to_string(), doc);
		}

		self.package_docs.get(package).unwrap()
	}

	#[async_recursion]
	pub async fn resolve_dependencies<'a, D>(
		&mut self,
		dependencies: D,
		parent: &str,
		layer: usize,
	) -> HashMap<String, String>
	where
		D: IntoIterator<Item = (&'a String, &'a String)> + Send,
		D::IntoIter: Send,
	{
		let mut resolved_dependencies = HashMap::new();

		for (dependency, version) in dependencies {
			// Not ideal to clone, but the borrow checker thinks this is a mutable borrow?
			let doc = self.query_package(dependency).await.clone();

			let desired_version = doc
				.versions
				.get(doc.dist_tags.get("latest").unwrap())
				.unwrap();

			let desired_tarball = desired_version.dist.tarball.clone();

			// println!("----------------------------------------");
			println!(
				"\t{}| {} > {}@{}",
				"  ".repeat(layer),
				parent,
				dependency,
				version
			);
			// println!("----------------------------------------");
			// println!("{:#?}\n\n", &desired_tarball);

			if dependency == "function-bind" {
				if self.panic > 100 {
					panic!("PANIC");
				}
				self.panic += 1;
			}

			resolved_dependencies.insert(dependency.clone(), desired_tarball);

			if let Some(transitive_dependencies) = &desired_version.dependencies {
				resolved_dependencies.extend(
					self.resolve_dependencies(
						transitive_dependencies
							.iter()
							.filter(|&(name, _)| !self.package_docs.contains_key(name))
							.collect::<HashMap<_, _>>(),
						dependency,
						layer + 1,
					)
					.await,
				);
			};
		}

		resolved_dependencies
	}
}
