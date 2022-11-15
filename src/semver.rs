use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;

#[derive(Clone, Default, Eq, PartialEq)]
pub struct Version {
	major: u64,
	minor: u64,
	patch: u64,
}

impl Debug for Version {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{{v{}.{}.{}}}", self.major, self.minor, self.patch)
	}
}

impl From<(u64, u64, u64)> for Version {
	fn from(version: (u64, u64, u64)) -> Self {
		let (major, minor, patch) = version;
		Version {
			major,
			minor,
			patch,
		}
	}
}

impl PartialOrd for Version {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Version {
	fn cmp(&self, other: &Self) -> Ordering {
		self.major
			.cmp(&other.major)
			.then(self.minor.cmp(&other.minor))
			.then(self.patch.cmp(&other.patch))
	}
}

pub enum SemverRange {
	GreaterThan(Version),
	GreaterThanOrEqual(Version),
	LessThan(Version),
	Exact(Version),
	Patched(Version),
	Compatible(Version),
	Any,
}

impl SemverRange {
	pub fn matches(&self, version: &Version) -> bool {
		match self {
			SemverRange::GreaterThan(v) => version > v,
			SemverRange::GreaterThanOrEqual(v) => version >= v,
			SemverRange::LessThan(v) => version < v,
			SemverRange::Exact(v) => version == v,
			SemverRange::Patched(v) => {
				version.major == v.major && version.minor == v.minor && version.patch >= v.patch
			}
			SemverRange::Compatible(v) => {
				version.major == v.major && version.minor >= v.minor && version.patch >= v.patch
			}
			SemverRange::Any => true,
		}
	}
}

impl Version {
	pub fn satisfies(&self, range: &SemverRange) -> bool {
		range.matches(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn greater_than() {
		use SemverRange::GreaterThan;

		let range = GreaterThan((1, 0, 0).into());

		assert!(!Version::from((0, 0, 0)).satisfies(&range));
		assert!(!Version::from((1, 0, 0)).satisfies(&range));
		assert!(Version::from((1, 0, 1)).satisfies(&range));
		assert!(Version::from((2, 0, 0)).satisfies(&range));
	}

	#[test]
	fn greater_than_or_equal() {
		use SemverRange::GreaterThanOrEqual;

		let range = GreaterThanOrEqual((1, 0, 0).into());

		assert!(!Version::from((0, 0, 0)).satisfies(&range));
		assert!(Version::from((1, 0, 0)).satisfies(&range));
		assert!(Version::from((1, 0, 1)).satisfies(&range));
		assert!(Version::from((2, 0, 0)).satisfies(&range));
	}

	#[test]
	fn less_than() {
		use SemverRange::LessThan;

		let range = LessThan((1, 0, 0).into());

		assert!(Version::from((0, 0, 0)).satisfies(&range));
		assert!(!Version::from((1, 0, 0)).satisfies(&range));
		assert!(!Version::from((1, 0, 1)).satisfies(&range));
		assert!(!Version::from((2, 0, 0)).satisfies(&range));
	}

	#[test]
	fn exact() {
		use SemverRange::Exact;

		let range = Exact((1, 0, 0).into());

		assert!(!Version::from((0, 0, 0)).satisfies(&range));
		assert!(Version::from((1, 0, 0)).satisfies(&range));
		assert!(!Version::from((1, 0, 1)).satisfies(&range));
	}

	#[test]
	fn patched() {
		use SemverRange::Patched;

		let range = Patched((1, 0, 0).into());

		assert!(!Version::from((0, 0, 0)).satisfies(&range));
		assert!(Version::from((1, 0, 0)).satisfies(&range));
		assert!(Version::from((1, 0, 1)).satisfies(&range));
		assert!(!Version::from((1, 1, 0)).satisfies(&range));
		assert!(!Version::from((2, 0, 0)).satisfies(&range));
	}

	#[test]
	fn compatible() {
		use SemverRange::Compatible;

		let range = Compatible((1, 0, 0).into());

		assert!(!Version::from((0, 0, 0)).satisfies(&range));
		assert!(Version::from((1, 0, 0)).satisfies(&range));
		assert!(Version::from((1, 0, 1)).satisfies(&range));
		assert!(Version::from((1, 1, 0)).satisfies(&range));
		assert!(!Version::from((2, 0, 0)).satisfies(&range));
	}

	#[test]
	fn any() {
		use SemverRange::Any;

		assert!(Version::from((0, 0, 0)).satisfies(&Any));
		assert!(Version::from((1, 0, 0)).satisfies(&Any));
		assert!(Version::from((1, 0, 1)).satisfies(&Any));
		assert!(Version::from((1, 1, 0)).satisfies(&Any));
		assert!(Version::from((2, 0, 0)).satisfies(&Any));
	}
}
