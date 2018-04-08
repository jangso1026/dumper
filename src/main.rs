#[macro_use]
extern crate clap;
use clap::App;

fn main() {
	let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

	let file = matches.value_of("file").unwrap_or("nothing");

	let kitty = matches.value_of("kitty").unwrap_or("yowl");

	println!("{}", file);
	println!("{}", kitty);
}
