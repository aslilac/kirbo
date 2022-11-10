use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
	pub name: Option<String>,
	pub version: Option<String>,
	pub dependencies: Option<HashMap<String, String>>,
	pub dev_dependencies: Option<HashMap<String, String>>,
	pub scripts: Option<HashMap<String, String>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RegistryDoc {
	pub dist_tags: HashMap<String, String>,
	pub versions: HashMap<String, RegistryDocVersion>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RegistryDocVersion {
	pub version: String,
	pub dependencies: Option<HashMap<String, String>>,
	pub dist: PackageDist,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PackageDist {
	pub integrity: Option<String>,
	pub tarball: String,
}
