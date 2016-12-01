extern crate core;

use std::convert::From;
use std::iter::IntoIterator;

#[derive(Debug)]
pub enum Token {
    LShift,
    RShift,
    Plus,
    Minus,
    Dot,
    Comma,
    LBracket,
    RBracket,
}

impl Token {
    pub fn from_char(ch: char) -> Option<Token> {
        match ch {
            '<' => Some(Token::LShift),
            '>' => Some(Token::RShift),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '.' => Some(Token::Dot),
            ',' => Some(Token::Comma),
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),
            _   => None,
        }
    }
}

#[derive(Debug)]
pub enum Node {
    LShift,
    RShift,
    Inc,
    Dec,
    PutCh,
    GetCh,
    Loop(Block),
}

#[derive(Debug)]
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
