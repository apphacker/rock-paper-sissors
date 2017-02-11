extern crate rand;
#[macro_use]
extern crate clap;
extern crate term_painter;

mod display;
mod log;
mod game;
mod pieces;

use clap::App;
use log::*;

#[derive(Debug)]
pub struct Options {
    verbose: bool,
    very_verbose: bool,
    colors: bool,
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let options = Options {
        verbose: matches.is_present("verbose"),
        very_verbose: matches.occurrences_of("verbose") > 1,
        colors: matches.is_present("colors"),
    };

    v(format!("Playing game with options: {:?}", options), &options);
    game::run(&options);
}
