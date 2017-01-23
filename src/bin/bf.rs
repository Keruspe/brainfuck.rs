extern crate brainfuck;
extern crate clap;

use brainfuck::context::Context;
use brainfuck::parser;
use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() {
    let matches = App::new("Brainfuck interpreter")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Read, parse and execute a brainfuck program")
        .arg(Arg::with_name("INPUT")
             .help("Set the input file to use")
             .required(true)
             .index(1))
        .get_matches();
    let input   = matches.value_of("INPUT").unwrap();

    if let Err(err) = run(input) {
        println!("Error while running {}: {}", input, err);
        std::process::exit(1);
    }
}

fn run(input: &str) -> Result<(), Box<Error>> {
    let mut contents = vec![];

    File::open(input)?.read_to_end(&mut contents)?;

    Ok(Context::new().run(&parser::parse(contents.as_ref())?))
}
