use std::process::Command;

mod testing;
use testing::EXE;

#[test]
fn install_easy() {
	testing::before();

	let result = Command::new(&*EXE)
		.current_dir("./tests/testdata/01-easy/")
		.output()
		.unwrap();

	assert!(!result.status.success());
}

#[test]
fn run_prettier() {
	testing::before();

	let result = Command::new(&*EXE)
		.current_dir("./tests/testdata/11-prettier/")
		.args(["--", "prettier", "--check", "."])
		.output()
		.unwrap();

	assert!(!result.status.success());
}
