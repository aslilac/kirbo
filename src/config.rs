use once_cell::sync::Lazy;
use std::env::current_exe;
use std::path::PathBuf;

pub struct Env {
	execpath: PathBuf,
}

pub const ENV: Lazy<Env> = Lazy::new(|| {
	let execpath = current_exe().unwrap();
	Env { execpath }
});
