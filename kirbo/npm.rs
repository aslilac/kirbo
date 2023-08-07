use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize)]
pub struct Workspace {
	pub workspaces: HashMap<PathBuf, PackageJson>,
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
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	pub dependencies: HashMap<String, String>,
	pub dist: PackageDist,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PackageDist {
	pub integrity: Option<String>,
	pub tarball: String,
}
