use ast::{Block, Node};

use std::io::{self, Read};

pub struct Context {
    lbuf:  Vec<i8>,
    rbuf:  Vec<i8>,
    index: i64,
}

impl Context {
    pub fn new() -> Context {
        Context {
            lbuf:  vec![0],
            rbuf:  vec![0],
            index: 0,
        }
    }

    fn get(&self) -> Option<&i8> {
        if self.index < 0 {
            self.lbuf.get((-self.index - 1) as usize)
        } else {
            self.rbuf.get(self.index as usize)
        }
    }

    fn get_mut(&mut self) -> Option<&mut i8> {
        if self.index < 0 {
            self.lbuf.get_mut((-self.index - 1) as usize)
        } else {
            self.rbuf.get_mut(self.index as usize)
        }
    }

    fn loop_cond(&self) -> bool {
        self.get().map(|e| *e != 0).unwrap_or(false)
    }

    fn run_node(&mut self, node: &Node) {
        match *node {
            Node::LShift => {
                self.index -= 1;
                if self.index < 0 {
                    while self.lbuf.len() <= ((-self.index - 1) as usize) {
                        self.lbuf.push(0);
                    }
                }
            },
            Node::RShift => {
                self.index += 1;
                if self.index >= 0 {
                    while self.rbuf.len() <= (self.index as usize) {
                        self.rbuf.push(0);
                    }
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
