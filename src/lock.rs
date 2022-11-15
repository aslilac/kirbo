use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KirboLock {
	lock_version: String,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	binaries: BTreeMap<String, String>,
	#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
	packages: BTreeMap<String, KirboLockPackage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct KirboLockPackage {
	// version: String,
	resolved: String,
	sha512: String,
	#[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
	dependencies: BTreeSet<String>,
	#[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
	peer_dependencies: BTreeSet<String>,
	#[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
	dev_dependencies: BTreeSet<String>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn serialization_prettier() {
		let lock_object = KirboLock {
			lock_version: "0a".to_string(),
			binaries: BTreeMap::from([("prettier".to_string(), "prettier@2.7.1".to_string())]),
			packages: BTreeMap::from([(
				"prettier@2.7.1".to_string(),
				KirboLockPackage {
					resolved: "https://registry.npmjs.org/prettier/-/prettier-2.7.1.tgz"
						.to_string(),
					sha512: "abcdefghijklmnopqrstuvwxyz".to_string(),
					..Default::default()
				},
			)]),
		};

		let lock_snapshot = include_str!("./testdata/prettier.lock");

		assert_eq!(serde_yaml::to_string(&lock_object).unwrap(), lock_snapshot);
		assert_eq!(
			serde_yaml::from_str::<KirboLock>(&lock_snapshot).unwrap(),
			lock_object
		);
	}

	#[test]
	fn serialization_succulent() {
		let lock_object = KirboLock {
			lock_version: "0a".to_string(),
			binaries: BTreeMap::new(),
			packages: BTreeMap::from([(
				"succulent@^0.18.0".to_string(),
				KirboLockPackage {
					resolved: "https://registry.npmjs.org/succulent/-/succulent-0.18.0.tgz"
						.to_string(),
					sha512: "abcdefghijklmnopqrstuvwxyz".to_string(),
					..Default::default()
				},
			)]),
		};

		let lock_snapshot = include_str!("./testdata/succulent.lock");

		assert_eq!(serde_yaml::to_string(&lock_object).unwrap(), lock_snapshot);
		assert_eq!(
			serde_yaml::from_str::<KirboLock>(&lock_snapshot).unwrap(),
			lock_object
		);
	}
}
