# brainfuck

[![Build Status](https://travis-ci.org/Keruspe/brainfuck.rs.svg?branch=master)](https://travis-ci.org/Keruspe/brainfuck.rs)
[![Coverage Status](https://coveralls.io/repos/github/Keruspe/brainfuck.rs/badge.svg?branch=master)](https://coveralls.io/github/Keruspe/brainfuck.rs?branch=master)

Implementation of a brainfuck parser written in rust using nom

Example:

```rust
extern crate brainfuck;

use brainfuck::context::Context;
use brainfuck::parser;

fn main() {
    let hello_world = include_bytes!("hello_world.bf");
    let mut ctx     = Context::new();
    let block       = parser::parse(hello_world).expect("Failed parsing input file");
    ctx.run(&block);
}
```

You can run `cargo run --example hello_world` to try this
