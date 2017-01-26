use ast::{Block, Node};

use std::io::{self, Read};

#[derive(Debug, PartialEq)]
pub struct Context {
    lbuf:  Vec<i8>,
    rbuf:  Vec<i8>,
    index: i64,
}

impl Context {
    pub fn new() -> Context {
        Context {
            lbuf:  Vec::new(),
            rbuf:  vec![0],
            index: 0,
        }
    }

    #[cfg(test)]
    pub fn new_with_data(lbuf: Vec<i8>, rbuf: Vec<i8>, index: i64) -> Context {
        Context {
            lbuf:  lbuf,
            rbuf:  rbuf,
            index: index,
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

#[cfg(test)]
mod tests {
    use super::*;

    use ast::Node;

    #[test]
    fn test_lshift() {
        let mut ctx = Context::new();
        ctx.run_node(&Node::LShift);
        assert_eq!(ctx, Context::new_with_data(vec![0], vec![0], -1));
    }

    #[test]
    fn test_rshift() {
        let mut ctx = Context::new();
        ctx.run_node(&Node::RShift);
        assert_eq!(ctx, Context::new_with_data(Vec::new(), vec![0, 0], 1));
    }

    #[test]
    fn test_inc() {
        let mut ctx = Context::new();
        ctx.run_node(&Node::Inc);
        assert_eq!(ctx, Context::new_with_data(Vec::new(), vec![1], 0));
    }

    #[test]
    fn test_dec() {
        let mut ctx = Context::new();
        ctx.run_node(&Node::Dec);
        assert_eq!(ctx, Context::new_with_data(Vec::new(), vec![-1], 0));
    }

    #[test]
    fn test_putch() {
        let mut ctx = Context::new();
        ctx.run_node(&Node::PutCh);
        assert_eq!(ctx, Context::new());
    }

    #[test]
    fn test_block() {
        let mut ctx = Context::new();
        ctx.run(&From::from(vec![Node::Inc, Node::RShift, Node::Inc, Node::LShift, Node::LShift, Node::LShift, Node::Dec]));
        assert_eq!(ctx, Context::new_with_data(vec![0, -1], vec![1, 1], -2));
    }

    #[test]
    fn test_loop() {
        let mut ctx = Context::new();
        ctx.run_node(&Node::Inc);
        ctx.run_node(&Node::Inc);
        ctx.run_node(&Node::Loop(From::from(vec![Node::Dec, Node::RShift, Node::Inc, Node::LShift])));
        assert_eq!(ctx, Context::new_with_data(Vec::new(), vec![0, 2], 0));
    }

    #[test]
    fn test_left_loop() {
        let mut ctx = Context::new();
        ctx.run_node(&Node::LShift);
        ctx.run_node(&Node::Inc);
        ctx.run_node(&Node::Inc);
        ctx.run_node(&Node::Loop(From::from(vec![Node::Dec, Node::LShift, Node::Inc, Node::RShift])));
        assert_eq!(ctx, Context::new_with_data(vec![0, 2], vec![0], -1));
    }
}
