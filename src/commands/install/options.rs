use std::path::PathBuf;
use std::process::exit;

#[derive(Clone, Debug, Default)]
struct OptionsBuilder {
	input: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Options {
	pub input: PathBuf,
}

impl From<OptionsBuilder> for Options {
	fn from(builder: OptionsBuilder) -> Self {
		Options {
			input: builder.input.expect("no input provided"),
		}
	}
}

impl<S> FromIterator<S> for Options
where
	S: AsRef<str>,
{
	fn from_iter<I>(args: I) -> Self
	where
		I: IntoIterator<Item = S>,
	{
		let mut options = OptionsBuilder::default();
		let mut args = args.into_iter();

		while let Some(arg) = args.next() {
			let arg = arg.as_ref();
			if arg.len() >= 2 && arg.starts_with('-') || arg.len() >= 3 && arg.starts_with("--") {
				match arg {
					"-h" | "-help" | "--help" | "-?" => {
						println!("get some help");
						exit(0);
					}
					_ => {
						println!("unrecognized option: {}", arg);
						exit(1);
					}
				}
			} else {
				options.input = Some(PathBuf::from(arg));
			}
		}

		options.into()
	}
}
