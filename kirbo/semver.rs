use anyhow::anyhow;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Version {
	pub major: u64,
	pub minor: u64,
	pub patch: u64,
	pub prerelease_info: Option<String>,
	pub build_info: Option<String>,
}

impl Display for Version {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
		if self.prerelease_info.is_some() {
			write!(f, "-{}", self.prerelease_info.as_ref().unwrap())?;
		}
		if self.build_info.is_some() {
			write!(f, "+{}", self.build_info.as_ref().unwrap())?;
		}
		Ok(())
	}
}

impl From<(u64, u64, u64)> for Version {
	fn from(version: (u64, u64, u64)) -> Self {
		let (major, minor, patch) = version;
		Version {
			major,
			minor,
			patch,
			prerelease_info: None,
			build_info: None,
		}
	}
}

impl FromStr for Version {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut prerelease_start = None;
		let mut build_start = None;
		let mut must_be_dot_or_positive_digit = true;
		for (i, c) in s.chars().enumerate() {
			if c == '+' {
				if must_be_dot_or_positive_digit {
					return Err(anyhow!("unexpected character {}", c));
				}
				build_start = Some(i);
				break;
			}

			if prerelease_start.is_some() {
				continue;
			}

			if c == '-' {
				if must_be_dot_or_positive_digit {
					return Err(anyhow!("unexpected character {}", c));
				}
				prerelease_start = Some(i);
				continue;
			}

			if !c.is_ascii_digit() && c != '.' {
				return Err(anyhow!("unexpected character {}", c));
			}

			if must_be_dot_or_positive_digit && (c == '0' || c == '.') {
				return Err(anyhow!("unexpected character {}", c));
			}

			must_be_dot_or_positive_digit = c == '.';
		}

		dbg!(&prerelease_start);

		let version = {
			let end = prerelease_start.unwrap_or(s.len());
			&s[0..end]
		};
		let prerelease_info = prerelease_start.map(|start| {
			let end = build_start.unwrap_or(s.len());
			s[start..end].to_string()
		});
		let build_info = build_start.map(|start| s[start..s.len()].to_string());

		prerelease_info
			.as_ref()
			.map(|it| it.is_empty())
			.unwrap_or(false);

		let mut parts = version.split('.');
		let major = parts
			.next()
			.ok_or(anyhow!("empty version is not allowed"))?
			.parse()?;
		// XXX: Should we be stricter and require minor and patch? Maybe only
		// require minor? How does this interact with ranges like `~1`?
		let (minor, patch) = {
			let mut next = || match parts.next() {
				Some(part) => dbg!(part).parse(),
				None => Ok(0),
			};
			(next()?, next()?)
		};

		if parts.next().is_some() {
			return Err(anyhow!("semver versions must have exactly three segments"));
		}

		Ok(Version {
			major,
			minor,
			patch,
			prerelease_info,
			build_info,
		})
	}
}

impl PartialOrd for Version {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Version {
	fn cmp(&self, other: &Self) -> Ordering {
		self
			.major
			.cmp(&other.major)
			.then(self.minor.cmp(&other.minor))
			.then(self.patch.cmp(&other.patch))
	}
}

impl Version {
	pub fn satisfies(&self, range: &SemverRange) -> bool {
		range.matches(self)
	}

	pub fn to_string(&self) -> String {
		format!("{}.{}.{}", self.major, self.minor, self.patch)
	}
}

#[derive(Clone)]
pub enum SemverRange {
	/// >x.y.z
	GreaterThan(Version),
	/// >=x.y.z
	GreaterThanOrEqual(Version),
	/// <x.y.z
	LessThan(Version),
	/// x.y.z
	Exact(Version),
	/// ~x.y.z
	Patched(Version),
	/// ^x.y.z
	Compatible(Version),
	/// *
	Any,
}

impl Debug for SemverRange {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "SemverRange(")?;
		match self {
			SemverRange::GreaterThan(range) => write!(f, ">{:?}", range),
			SemverRange::GreaterThanOrEqual(range) => write!(f, ">={:?}", range),
			SemverRange::LessThan(range) => write!(f, "<{:?}", range),
			SemverRange::Exact(range) => write!(f, "{:?}", range),
			SemverRange::Patched(range) => write!(f, "~{:?}", range),
			SemverRange::Compatible(range) => write!(f, "^{:?}", range),
			SemverRange::Any => write!(f, "*"),
		}?;
		write!(f, ")")
	}
}

impl FromStr for SemverRange {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s == "*" {
			Ok(Self::Any)
		} else if s.starts_with(|a| char::is_ascii_digit(&a)) {
			Ok(Self::Exact(s.parse()?))
		} else if s.starts_with('^') {
			Ok(Self::Compatible(s[1..].parse()?))
		} else if s.starts_with('~') {
			Ok(Self::Patched(s[1..].parse()?))
		} else if s.starts_with('^') {
			Ok(Self::Compatible(s[1..].parse()?))
		} else if s.starts_with(">=") {
			Ok(Self::GreaterThanOrEqual(s[2..].parse()?))
		} else if s.starts_with(">=") {
			Ok(Self::GreaterThan(s[2..].parse()?))
		} else if s.starts_with("<") {
			Ok(Self::LessThan(s[2..].parse()?))
		} else {
			Err(anyhow!("invalid semver range {}", s))
		}
	}
}

impl SemverRange {
	pub fn matches(&self, version: &Version) -> bool {
		match self {
			SemverRange::GreaterThan(range) => version > range,
			SemverRange::GreaterThanOrEqual(range) => version >= range,
			SemverRange::LessThan(range) => version < range,
			SemverRange::Exact(range) => version == range,
			SemverRange::Patched(range) => {
				version.major == range.major && version.minor == range.minor && version.patch >= range.patch
			}
			SemverRange::Compatible(range) => {
				if range.major < 1 {
					if range.minor < 1 {
						// ^0.0.z must be exact equality
						return version == range;
					}
					// ^0.y.z must only vary by patch
					return version.major == range.major
						&& version.minor == range.minor
						&& version.patch >= range.patch;
				}
				// ^x.y.z may have any newer minor version
				version.major == range.major && version >= range
			}
			SemverRange::Any => true,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_version() {
		assert_eq!("1".parse::<Version>().unwrap(), Version::from((1, 0, 0)));
		assert_eq!("1.2".parse::<Version>().unwrap(), Version::from((1, 2, 0)));
		assert_eq!(
			"1.2.3".parse::<Version>().unwrap(),
			Version::from((1, 2, 3))
		);
		assert!("1.2.3-".parse::<Version>().is_err());
		assert!("1.2.3.4".parse::<Version>().is_err());
		assert!("1.2.3b".parse::<Version>().is_err());

		assert_eq!(
			"1.2.3-alpha.1".parse::<Version>().unwrap(),
			Version {
				major: 1,
				minor: 2,
				patch: 3,
				prerelease_info: Some("alpha.1".to_string()),
				build_info: None,
			}
		);
		assert_eq!(
			"1.2.3+build-1024".parse::<Version>().unwrap(),
			Version {
				major: 1,
				minor: 2,
				patch: 3,
				prerelease_info: None,
				build_info: Some("build-1024".to_string()),
			}
		);
		assert_eq!(
			"1.2.3-alpha.1+build-1024".parse::<Version>().unwrap(),
			Version {
				major: 1,
				minor: 2,
				patch: 3,
				prerelease_info: Some("alpha.1".to_string()),
				build_info: Some("build-1024".to_string()),
			}
		);
	}

	#[test]
	fn parse_version_range() {
		matches!("1".parse(), Ok(SemverRange::Exact(_)));
		matches!("^1".parse(), Ok(SemverRange::Compatible(_)));
		matches!("~1".parse(), Ok(SemverRange::Patched(_)));
		matches!(">=1".parse(), Ok(SemverRange::GreaterThanOrEqual(_)));
		matches!(">1".parse(), Ok(SemverRange::GreaterThan(_)));
		matches!("<1".parse(), Ok(SemverRange::LessThan(_)));
		matches!("*".parse(), Ok(SemverRange::Any));
	}

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

		let range = Compatible((0, 0, 1).into());

		assert!(!Version::from((0, 0, 0)).satisfies(&range));
		assert!(Version::from((0, 0, 1)).satisfies(&range));
		assert!(!Version::from((0, 0, 2)).satisfies(&range));
		assert!(!Version::from((0, 1, 0)).satisfies(&range));
		assert!(!Version::from((1, 0, 0)).satisfies(&range));

		let range = Compatible((0, 1, 0).into());

		assert!(!Version::from((0, 0, 0)).satisfies(&range));
		assert!(Version::from((0, 1, 0)).satisfies(&range));
		assert!(Version::from((0, 1, 1)).satisfies(&range));
		assert!(!Version::from((0, 2, 0)).satisfies(&range));
		assert!(!Version::from((1, 0, 0)).satisfies(&range));

		let range = Compatible((1, 0, 0).into());

		assert!(!Version::from((0, 0, 0)).satisfies(&range));
		assert!(Version::from((1, 0, 0)).satisfies(&range));
		assert!(Version::from((1, 0, 1)).satisfies(&range));
		assert!(Version::from((1, 1, 0)).satisfies(&range));
		assert!(!Version::from((2, 0, 0)).satisfies(&range));

		let range = Compatible((1, 1, 1).into());

		assert!(!Version::from((0, 0, 0)).satisfies(&range));
		assert!(!Version::from((1, 0, 0)).satisfies(&range));
		assert!(!Version::from((1, 0, 1)).satisfies(&range));
		assert!(!Version::from((1, 1, 0)).satisfies(&range));
		assert!(Version::from((1, 1, 1)).satisfies(&range));
		assert!(Version::from((1, 1, 2)).satisfies(&range));
		assert!(Version::from((1, 2, 0)).satisfies(&range));
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
