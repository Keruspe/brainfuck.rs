extern crate brainfuck;
extern crate clap;

use brainfuck::context::Context;
use brainfuck::parser;
use clap::{App, Arg};
use std::fs::File;
use std::io::Read;

fn main() {
    let matches     = App::new("Brainfuck interpreter")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Read, parse and execute a brainfuck program")
        .arg(Arg::with_name("INPUT")
             .help("Set the input file to use")
             .required(true)
             .index(1))
        .get_matches();
    let input       = matches.value_of("INPUT").unwrap();
    let mut file    = match File::open(input) {
        Ok(file) => file,
        Err(err) => panic!("Failed to open {}: {:?}", input, err),
    };
    let mut ctx      = Context::new();
    let mut contents = vec![];

    if let Err(err)  = file.read_to_end(&mut contents) {
        panic!("Failed to read {}: {:?}", input, err);
    }

    match parser::parse(contents.as_ref()) {
        Ok(block) => ctx.run(&block),
        Err(err)  => panic!("Failed to run {}: {:?}", input, err),
    }
}
