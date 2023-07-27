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

impl<S, const N: usize> TryFrom<&[S; N]> for Options
where
	S: AsRef<str>,
{
	type Error = anyhow::Error;

	fn try_from(args: &[S; N]) -> Result<Self, Self::Error> {
		Options::try_from(&args[..])
	}
}

impl<S> TryFrom<&[S]> for Options
where
	S: AsRef<str>,
{
	type Error = anyhow::Error;

	fn try_from(args: &[S]) -> Result<Self, Self::Error> {
		if args.len() == 1 {
			match args[0].as_ref() {
				"-h" | "-help" | "--help" | "-?" | "help" => {
					print!("{}", include_str!("./help.txt"));
					exit(0);
				}
				"-v" | "-V" | "-version" | "--version" | "version" => {
					println!(
						"{} {}",
						env!("CARGO_PKG_NAME").bright_magenta().bold(),
						env!("CARGO_PKG_VERSION").bold()
					);
					exit(0);
				}
				_ => (),
			}
		}

		let mut options = OptionsBuilder::default();
		let mut args = args.into_iter();

		if let Some(arg) = args.next() {
			let arg = arg.as_ref();
			match arg {
				"add" | "install" | "i" => {
					options.command = Some(Command::Install);
				}
				"run" | "run-script" => {
					options.command = Some(Command::Run);
				}
				"exec" | "x" | "--" => {
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

		Ok(options.into())
	}
}
