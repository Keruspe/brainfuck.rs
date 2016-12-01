use ast::{Block, Node};

use std::io::{self, Read};

pub struct Context {
    buf:   Vec<i8>,
    index: usize,
}

impl Context {
    pub fn new() -> Context {
        Context {
            buf:   vec![0],
            index: 0,
        }
    }

    fn get(&self) -> Option<&i8> {
        self.buf.get(self.index)
    }

    fn get_mut(&mut self) -> Option<&mut i8> {
        self.buf.get_mut(self.index)
    }

    fn loop_cond(&self) -> bool {
        self.get().map(|e| *e != 0).unwrap_or(false)
    }

    fn run_node(&mut self, node: &Node) {
        match *node {
            Node::LShift => {
                self.index -= 1;
            },
            Node::RShift => {
                self.index += 1;
                while self.buf.len() <= self.index {
                    self.buf.push(0);
                }
            },
            Node::Inc => {
                if let Some(elem) = self.get_mut() {
                    *elem += 1;
                }
            },
            Node::Dec => {
                if let Some(elem) = self.get_mut() {
                    *elem -= 1;
                }
            },
            Node::PutCh => {
                if let Some(elem) = self.get_mut() {
                    print!("{}", (*elem as u8) as char);
                }
            },
            Node::GetCh => {
                let mut buffer = [0;1];
                io::stdin().read_exact(&mut buffer).expect("Failed to read from stdin");
                if let Some(elem) = self.get_mut() {
                    *elem = buffer[0] as i8;
                }
            },
            Node::Loop(ref block) => {
                while self.loop_cond() {
                    self.run(block);
                }
            },
        }
    }

    pub fn run(&mut self, block: &Block) {
        for node in block.into_iter() {
            self.run_node(node);
        }
    }
}
