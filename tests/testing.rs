use once_cell::sync::Lazy;
use std::env::current_dir;
use std::ffi::OsString;
use std::process::Command;
use std::sync::Once;

pub const EXE: Lazy<OsString> = Lazy::new(|| {
	current_dir()
		.unwrap()
		.join("./build/release/kirbo")
		.into_os_string()
});

static BUILD: Once = Once::new();

pub fn before() {
	BUILD.call_once(|| {
		Command::new("cargo")
			.args(&["build", "--release"])
			.status()
			.expect("failed to build test binary");
	});
}
