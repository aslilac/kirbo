use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum PackageJsonError {
	IoError(io::Error),
	JsonError(serde_json::Error),
}

impl From<io::Error> for PackageJsonError {
	fn from(err: io::Error) -> Self {
		Self::IoError(err)
	}
}

impl From<serde_json::Error> for PackageJsonError {
	fn from(err: serde_json::Error) -> Self {
		Self::JsonError(err)
	}
}

#[derive(Clone, Debug)]
pub struct PackageJson(pub PathBuf, pub Package);

impl TryFrom<PathBuf> for PackageJson {
	type Error = PackageJsonError;

	fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
		let package_text = fs::read_to_string(&path)?;
		let package = serde_json::from_str(&package_text)?;

		Ok(PackageJson(path, package))
	}
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Package {
	pub private: Option<bool>,
	pub name: Option<String>,
	pub version: Option<String>,
	pub license: Option<String>,
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	pub dependencies: HashMap<String, String>,
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	pub peer_dependencies: HashMap<String, String>,
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	pub dev_dependencies: HashMap<String, String>,
	#[serde(default, skip_serializing_if = "HashMap::is_empty")]
	pub scripts: HashMap<String, String>,
}
