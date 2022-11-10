use crate::options::Options;

pub fn main(options: Options) {
	let args = options.remaining_args;

	println!("{:?}", args);
}
