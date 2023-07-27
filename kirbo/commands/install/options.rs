use std::convert::TryFrom;
use std::process::exit;

#[derive(Clone, Debug, Default)]
struct OptionsBuilder {
	packages_to_add: Vec<NewPackage>,
}

#[derive(Clone, Debug)]
pub struct Options {
	pub packages_to_add: Vec<NewPackage>,
}

#[derive(Clone, Debug)]
pub enum NewPackage {
	Dependency(String),
	DevDependency(String),
	TestDependency(String),
	OptionalDependency(String),
}

impl TryFrom<OptionsBuilder> for Options {
	type Error = anyhow::Error;

	fn try_from(builder: OptionsBuilder) -> Result<Self, Self::Error> {
		Ok(Options {
			// input: builder.input.ok_or(anyhow!("no input provided"))?,
			// input: env::current_dir()?,
			packages_to_add: builder.packages_to_add,
		})
	}
}

impl<S> TryFrom<&[S]> for Options
where
	S: AsRef<str>,
{
	type Error = anyhow::Error;

	fn try_from(args: &[S]) -> Result<Self, Self::Error> {
		let mut options = OptionsBuilder::default();

		if args.len() == 1 {
			match args[0].as_ref() {
				"-h" | "-help" | "--help" | "-?" | "help" => {
					print!("{}", include_str!("./help.txt"));
					exit(0);
				}
				_ => (),
			}
		}

		let mut args = args.into_iter();

		while let Some(arg) = args.next() {
			let arg = arg.as_ref();

			// `a` should add "a" to `dependencies`
			if arg.starts_with(|a| char::is_ascii_alphanumeric(&a)) {
				options
					.packages_to_add
					.push(NewPackage::Dependency(arg.to_string()));
				continue;
			}

			// `-Da` should add "a" to `devDependencies`
			if arg.len() >= 3
				&& arg.starts_with("-D")
				&& arg.bytes().nth(2).unwrap().is_ascii_alphanumeric()
			{
				options
					.packages_to_add
					.push(NewPackage::DevDependency(arg[2..].to_string()));
				continue;
			}

			// `-Ta` should add "a" to `testDependencies`
			if arg.len() >= 3
				&& arg.starts_with("-T")
				&& arg.bytes().nth(2).unwrap().is_ascii_alphanumeric()
			{
				options
					.packages_to_add
					.push(NewPackage::TestDependency(arg[2..].to_string()));
				continue;
			}

			// `-Oa` should add "a" to `optionalDependencies`
			if arg.len() >= 3
				&& arg.starts_with("-O")
				&& arg.bytes().nth(2).unwrap().is_ascii_alphanumeric()
			{
				options
					.packages_to_add
					.push(NewPackage::OptionalDependency(arg[2..].to_string()));
				continue;
			}

			if arg.len() >= 2 && arg.starts_with('-') || arg.len() >= 3 && arg.starts_with("--") {
				match arg {
					_ => {
						println!("unrecognized option: {}", arg);
						exit(1);
					}
				}
			}
		}

		options.try_into()
	}
}
