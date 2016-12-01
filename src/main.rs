#[macro_use]
extern crate brainfuck;

use brainfuck::context::Context;
use brainfuck::parser;

fn main() {
    let hello_world = include_bytes!("hello_world.bf");
    let mut ctx     = Context::new();
    let nodes       = parser::parse(hello_world).expect("Failed parsing input file");
    ctx.run(&nodes);
}
