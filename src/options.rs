use std::process::exit;

#[derive(Clone, Debug, Default)]
struct OptionsBuilder {
	command: Option<Command>,
	remaining_args: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Options {
	pub command: Command,
	pub remaining_args: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
	Install,
	Run,
}

impl From<OptionsBuilder> for Options {
	fn from(builder: OptionsBuilder) -> Self {
		Options {
			command: builder.command.unwrap_or(Command::Install),
			remaining_args: builder.remaining_args,
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

		if let Some(arg) = args.next() {
			let arg = arg.as_ref();
			match arg {
				"-h" | "-help" | "--help" | "-?" => {
					println!("get some help");
					exit(0);
				}
				"-v" | "-V" | "--version" => {
					println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
					exit(0);
				}
				"install" => {
					options.command = Some(Command::Install);
				}
				"run" => {
					options.command = Some(Command::Run);
				}
				_ => {
					options.command = Some(Command::Install);
					options.remaining_args.push(arg.to_string());
				}
			}
		}

		options
			.remaining_args
			.extend(args.map(|s| s.as_ref().to_string()));

		options.into()
	}
}
