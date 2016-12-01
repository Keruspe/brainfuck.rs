use ast::Node;

use std::io::{self, Read};

pub struct Context {
    buf:      Vec<char>,
    index:    usize,
}

impl Context {
    pub fn new() -> Context {
        Context {
            buf:      Vec::new(),
            index:    0,
        }
    }

    fn loop_cond(&self) -> bool {
        self.buf.get(self.index).map(|e| *e != (0 as char)).unwrap_or(false)
    }

    pub fn run(&mut self, node: &Node) {
        match *node {
            Node::LShift => {
                assert!(self.index > 0);
                self.index -= 1;
            },
            Node::RShift => {
                self.index += 1;
                while self.buf.len() <= self.index {
                    self.buf.push(0 as char);
                }
            },
            Node::Inc => {
                if let Some(elem) = self.buf.get_mut(self.index) {
                    let mut val = *elem as u8;
                    val += 1;
                    *elem = val as char;
                }
            },
            Node::Dec => {
                if let Some(elem) = self.buf.get_mut(self.index) {
                    let mut val = *elem as u8;
                    val -= 1;
                    *elem = val as char;
                }
            },
            Node::PutCh => {
                if let Some(elem) = self.buf.get_mut(self.index) {
                    print!("{}", *elem);
                }
            },
            Node::GetCh => {
                let mut buffer = [0;1];
                io::stdin().read_exact(&mut buffer).expect("Failed to read from stdin");
                if let Some(elem) = self.buf.get_mut(self.index) {
                    *elem = buffer[0] as char;
                }
            },
            Node::Loop(ref nodes) => {
                while self.loop_cond() {
                    for node in nodes {
                        self.run(node);
                    }
                }
            },
        }
    }
}
