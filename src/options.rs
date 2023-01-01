use std::process::exit;

use colored::Colorize;

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
	Exec,
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
					println!(
						"{} {}",
						env!("CARGO_PKG_NAME").bright_magenta().bold(),
						env!("CARGO_PKG_VERSION").bold()
					);
					exit(0);
				}
				"install" => {
					options.command = Some(Command::Install);
				}
				"run" => {
					options.command = Some(Command::Run);
				}
				"exec" | "--" => {
					options.command = Some(Command::Exec);
				}
				_ => {
					options.command = if (arg.len() >= 2 && arg.starts_with('-'))
						|| arg.len() >= 3 && arg.starts_with("--")
					{
						// If it's a `--flag`, then we probably want `kirbo install`
						Some(Command::Install)
					} else {
						// If it's a `word`, then we probably want `kirbo run $word`
						Some(Command::Run)
					};
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
