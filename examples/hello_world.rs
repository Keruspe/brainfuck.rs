use brainfuck::context::Context;
use brainfuck::parser;

fn main() {
    let hello_world = parser::CompleteByteSlice(include_bytes!("hello_world.bf"));
    let mut ctx     = Context::new();
    let block       = parser::parse(hello_world).expect("Failed parsing input file");
    ctx.run(&block);
}
