extern crate core;

use std::convert::From;
use std::iter::IntoIterator;

#[derive(Debug, PartialEq)]
pub enum Node {
    LShift,
    RShift,
    Inc,
    Dec,
    PutCh,
    GetCh,
    Loop(Block),
}

#[derive(Debug, PartialEq)]
pub struct Block(Vec<Node>);

impl Block {
    pub fn new() -> Block {
        Block(Vec::new())
    }

    pub fn push(&mut self, node: Node) {
        self.0.push(node);
    }
}

impl From<Vec<Node>> for Block {
    fn from(nodes: Vec<Node>) -> Self {
        Block(nodes)
    }
}

impl<'a> IntoIterator for &'a Block {
    type Item     = &'a Node;
    type IntoIter = self::core::slice::Iter<'a, Node>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
