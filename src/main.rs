#[macro_use]
extern crate brainfuck;
#[macro_use]
extern crate nom;

use brainfuck::ast::Node;
use brainfuck::context::Context;
use brainfuck::parser::parse_node;

use nom::IResult;

fn run(i: &[u8]) {
    let mut ctx = Context::new();
    let nodes: IResult<&[u8], Vec<Node>> = many0!(i, parse_node);
    if let IResult::Done(_, nodes) = nodes {
        for node in &nodes {
            ctx.run(node);
        }
    }
}

fn main() {
    let hello_world = include_bytes!("hello_world.bf");
    run(hello_world);
}
