use anyhow::anyhow;
use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::PackageJson;
use crate::PackageJsonError;

pub struct Workspace {
	root: PathBuf,
	packages_by_name: HashMap<String, PackageJson>,
}

impl Workspace {
	fn new(path: &Path) -> anyhow::Result<Self> {
		// The farthest up directory which contains a package.json that eventually leads
		// back up to us.
		let mut root = None;
		let mut next_path = Some(path);

		while let Some(path) = next_path {
			let package_json = PackageJson::try_from(path.join("package.json"));

			match package_json {
				Ok(package_json) => {
					dbg!(&package_json);
					root = Some(path);
				}
				Err(PackageJsonError::JsonError(_)) => {
					println!("failed to parse package.json in {}", path.display())
				}
				Err(PackageJsonError::IoError(err)) if err.kind() == io::ErrorKind::NotFound => {
					println!("no package.json in {}, that's fine", path.display())
				}
				Err(PackageJsonError::IoError(err)) => println!("other io::Error {:?}", err),
			}

			// Process to check next parent if it exists.
			next_path = path.parent();
		}

		let Some(root) = root else {
			return Err(anyhow!("fuck off"));
		};

		Ok(Workspace {
			root: root.to_path_buf(),
			packages_by_name: Default::default(),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rejects_no_package() {
		let workspace = Workspace::new(Path::new("testdata/no_package/"));
		assert!(workspace.is_err());
	}

	#[test]
	fn resolves_single_package_from_root() {
		let workspace = Workspace::new(Path::new("testdata/single_package/")).unwrap();
		assert_eq!(&workspace.root, Path::new("testdata/single_package/"));
	}

	#[test]
	fn resolves_single_package_from_nested_directory() {
		let workspace = Workspace::new(Path::new("testdata/single_package/docs/")).unwrap();
		assert_eq!(&workspace.root, Path::new("testdata/single_package/"));
	}

	#[test]
	fn resolves_single_package_inside_another_package() {
		let workspace = Workspace::new(Path::new("testdata/single_package/internal/")).unwrap();
		assert_eq!(
			&workspace.root,
			Path::new("testdata/single_package/internal/")
		);
	}

	#[test]
	fn resolves_single_package_inside_workspaces() {
		let workspace = Workspace::new(Path::new("testdata/workspace_direct/internal/")).unwrap();
		assert_eq!(
			&workspace.root,
			Path::new("testdata/workspace_direct/internal/")
		);
		let workspace = Workspace::new(Path::new("testdata/workspace_nested/internal/")).unwrap();
		assert_eq!(
			&workspace.root,
			Path::new("testdata/workspace_nested/internal/")
		);
	}

	#[test]
	fn resolves_workspace_direct_from_root() {
		let workspace = Workspace::new(Path::new("testdata/workspace_direct/")).unwrap();
		assert_eq!(&workspace.root, Path::new("testdata/workspace_direct/"));
	}

	#[test]
	fn resolves_workspace_direct_from_nested_directory() {
		let workspace = Workspace::new(Path::new("testdata/workspace_direct/docs/")).unwrap();
		assert_eq!(&workspace.root, Path::new("testdata/workspace_direct/"));
	}

	#[test]
	fn resolves_workspace_direct_from_package() {
		let workspace = Workspace::new(Path::new("testdata/workspace_direct/poyo/")).unwrap();
		assert_eq!(&workspace.root, Path::new("testdata/workspace_direct/"));
	}

	#[test]
	fn resolves_workspace_direct_from_package_nested_directory() {
		let workspace = Workspace::new(Path::new("testdata/workspace_direct/poyo/docs/")).unwrap();
		assert_eq!(&workspace.root, Path::new("testdata/workspace_direct/"));
	}

	#[test]
	fn resolves_workspace_nested_from_root() {
		let workspace = Workspace::new(Path::new("testdata/workspace_nested/")).unwrap();
		assert_eq!(&workspace.root, Path::new("testdata/workspace_nested/"));
	}

	#[test]
	fn resolves_workspace_nested_from_nested_directory() {
		let workspace = Workspace::new(Path::new("testdata/workspace_nested/docs/")).unwrap();
		assert_eq!(&workspace.root, Path::new("testdata/workspace_nested/"));
	}

	#[test]
	fn resolves_workspace_nested_from_package() {
		let workspace = Workspace::new(Path::new("testdata/workspace_nested/packages/poyo/")).unwrap();
		assert_eq!(&workspace.root, Path::new("testdata/workspace_nested/"));
	}

	#[test]
	fn resolves_workspace_nested_from_package_nested_directory() {
		let workspace =
			Workspace::new(Path::new("testdata/workspace_nested/packages/poyo/docs/")).unwrap();
		assert_eq!(&workspace.root, Path::new("testdata/workspace_nested/"));
	}

	#[test]
	#[ignore]
	fn resolves_workspace_nested_wildcard_from_root() {}

	#[test]
	#[ignore]
	fn resolves_workspace_nested_wildcard_from_nested_directory() {}
}
