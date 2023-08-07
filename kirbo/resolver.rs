use anyhow::anyhow;
use async_recursion::async_recursion;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::str::FromStr;

use crate::npm;
use crate::semver::SemverRange;
use crate::semver::Version;

static CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

#[derive(Debug, Default)]
pub struct Resolver {
	package_docs: HashMap<String, npm::RegistryDoc>,
}

impl Resolver {
	pub async fn query_package(&mut self, package: &str) -> anyhow::Result<&npm::RegistryDoc> {
		if !self.package_docs.contains_key(package) {
			print!("❄️ ");
			let doc = CLIENT
				.get(format!("https://registry.npmjs.org/{}", package))
				.send()
				.await?
				.json::<npm::RegistryDoc>()
				.await?;

			self.package_docs.insert(package.to_string(), doc);
		}

		// We checked for it above, so it better be in there!
		Ok(self.package_docs.get(package).unwrap())
	}

	#[async_recursion(?Send)]
	pub async fn resolve_dependencies<'a, D>(
		&mut self,
		dependencies: D,
		layer: usize,
	) -> anyhow::Result<HashMap<String, String>>
	where
		D: IntoIterator<Item = (&'a String, &'a String)> + Send,
		D::IntoIter: Send,
	{
		let mut resolved_dependencies = HashMap::new();

		for (dependency, version) in dependencies {
			// Not ideal to clone, but the borrow checker thinks this is a mutable borrow?
			let doc = self.query_package(dependency).await?.clone();
			let mut available_versions = doc
				.versions
				.keys()
				.map(AsRef::as_ref)
				.map(Version::from_str)
				.flatten()
				.collect::<Vec<_>>();
			// This is less than ideal, but whatever
			available_versions.sort();
			available_versions.reverse();
			let version_range = SemverRange::from_str(version)
				.map_err(|_| anyhow!("Invalid version range \"{}\"", version))?;
			let matched_version = available_versions
				.iter()
				.find(|version| version.satisfies(&version_range))
				.ok_or_else(|| anyhow!("no version of {} satisfies {}", dependency, version))?;

			println!("about to unwrap {}", dependency);
			let desired_version = doc.versions.get(&matched_version.to_string()).unwrap();

			let desired_tarball = desired_version.dist.tarball.clone();

			// println!("----------------------------------------");
			println!(
				// "\t{}⎣ {}@{}",
				"\t{}├ {}@{}",
				"⎜ ".repeat(layer),
				dependency,
				matched_version
			);
			// println!("----------------------------------------");
			// println!("{:#?}\n\n", &desired_tarball);

			if layer > 100 {
				panic!("PANIC");
			}

			resolved_dependencies.insert(dependency.clone(), desired_tarball);

			let transitive_dependencies = &desired_version.dependencies;

			resolved_dependencies.extend(
				self
					.resolve_dependencies(
						transitive_dependencies
							.iter()
							.filter(|&(name, _)| !self.package_docs.contains_key(name))
							.collect::<HashMap<_, _>>(),
						layer + 1,
					)
					.await?,
			);
		}

		Ok(resolved_dependencies)
	}
}
