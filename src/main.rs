#[macro_use]
extern crate nom;

use nom::IResult;

use std::io::Read;

pub fn skip_unknown(i: &[u8]) -> IResult<&[u8], &[u8]> {
    if i.len() == 0 {
        IResult::Done::<&[u8], &[u8]>(i, &i[0..0])
    } else {
        match i.iter().enumerate().position(|(_, item)| {
            match *item as char {
                '>'|'<'|'+'|'-'|'.'|','|'['|']' => true,
                _                               => false,
            }
        }) {
            Some(index) => IResult::Done(&i[index..], &i[..index]),
            None        => IResult::Done(&i[i.len()..], i),
        }
    }
}

macro_rules! strip_unknown (
    ($i: expr, $($args:tt)*) => ({
        sep!($i, skip_unknown, $($args)*)
    });
);

struct Context {
    buf:      Vec<char>,
    index:    usize,
    nest_lvl: u64,
}

impl Context {
    pub fn new() -> Context {
        Context {
            buf:      Vec::new(),
            index:    0,
            nest_lvl: 0,
        }
    }
}

fn apply(ch: char, ctx: &mut Context) {
    match ch {
        '>' => {
            ctx.index = ctx.index + 1;
            if ctx.buf.len() <= ctx.index {
                let size = ctx.index - ctx.buf.len() + 1;
                ctx.buf.resize(size, 0 as char);
            }
        },
        '<' => {
            assert!(ctx.index > 0);
            ctx.index = ctx.index - 1;
        },
        '+' => {
            if let Some(elem) = ctx.buf.get_mut(ctx.index) {
                *elem = ((*elem as u8) + 1) as char;
            }
        },
        '-' => {
            if let Some(elem) = ctx.buf.get_mut(ctx.index) {
                *elem = ((*elem as u8) - 1) as char;
            }
        },
        '.' => {
            if let Some(elem) = ctx.buf.get(ctx.index) {
                print!("{}", elem);
            }
        },
        ',' => {
            let mut buffer = [0;1];
            std::io::stdin().read_exact(&mut buffer).expect("Failed to read from stdin");
            if let Some(elem) = ctx.buf.get_mut(ctx.index) {
                *elem = buffer[0] as char;
            }
        },
        '[' => {
            ctx.nest_lvl = ctx.nest_lvl + 1;
        },
        ']' => {
            assert!(ctx.nest_lvl > 0);
            ctx.nest_lvl = ctx.nest_lvl - 1;
        },
        _   => panic!("Unexpected character"),
    }
}

fn run(mut i: &[u8]) {
    let mut ctx = Context::new();
    loop {
        match strip_unknown!(i, take!(1)) {
            IResult::Done(_i, _o)  => {
               i = _i;
               apply(_o[0] as char, &mut ctx);
            },
            IResult::Incomplete(_) => {
                if ctx.nest_lvl != 0 {
                    panic!("Unbalanced braces");
                } else {
                    break;
                }
            },
            _                      => break,
        }
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    use nom::IResult;

    const EMPTY: &'static [u8] = b"";

    #[test]
    fn test_strip_unknown() {
        let all:      &[u8]                 = b"This is a test +- [Does it work?\n]><\n";
        let expected: Vec<&[u8]>            = vec![b"+", b"-", b"[", b"]", b">", b"<"];
        let got: IResult<&[u8], Vec<&[u8]>> = many1!(all, strip_unknown!(take!(1)));

        assert_eq!(got, IResult::Done(EMPTY, expected));
    }
}
