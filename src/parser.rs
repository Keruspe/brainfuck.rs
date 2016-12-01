use ast::*;
use nom::{IResult, ErrorKind};

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
        use $crate::parser::skip_unknown;
        sep!($i, skip_unknown, $($args)*)
    });
);

pub fn next_token(i: &[u8]) -> IResult<&[u8], Option<Token>> {
    map!(i, strip_unknown!(take!(1)), |ch: &[u8]| Token::from_char(ch[0] as char))
}

pub fn parse_loop(mut i: &[u8]) -> IResult<&[u8], Vec<Node>> {
    let mut nodes = Vec::new();
    loop {
        let tk = next_token(i);
        match tk {
            IResult::Error(e)      => return IResult::Error(e),
            IResult::Incomplete(i) => return IResult::Incomplete(i),
            IResult::Done(_i, _o)  => {
                i = _i;
                if let Some(token) = _o {
                    match token {
                        Token::LShift   => nodes.push(Node::LShift),
                        Token::RShift   => nodes.push(Node::RShift),
                        Token::Plus     => nodes.push(Node::Inc),
                        Token::Minus    => nodes.push(Node::Dec),
                        Token::Dot      => nodes.push(Node::PutCh),
                        Token::Comma    => nodes.push(Node::GetCh),
                        Token::LBracket => {
                            let lp = parse_loop(i);
                            match lp {
                                IResult::Error(e)      => return IResult::Error(e),
                                IResult::Incomplete(i) => return IResult::Incomplete(i),
                                IResult::Done(_i, _o)  => {
                                    i = _i;
                                    nodes.push(Node::Loop(_o));
                                }
                            }
                        },
                        Token::RBracket => return IResult::Done(i, nodes),
                    }
                } else {
                    return IResult::Error(error_code!(ErrorKind::Custom(42)))
                }
            },
        }
    }
}

pub fn parse_node(i: &[u8]) -> IResult<&[u8], Node> {
    let tk = next_token(i);
    match tk {
        IResult::Error(e)      => IResult::Error(e),
        IResult::Incomplete(i) => IResult::Incomplete(i),
        IResult::Done(i, o)    => {
            if let Some(token) = o {
                match token {
                    Token::LShift   => IResult::Done(i, Node::LShift),
                    Token::RShift   => IResult::Done(i, Node::RShift),
                    Token::Plus     => IResult::Done(i, Node::Inc),
                    Token::Minus    => IResult::Done(i, Node::Dec),
                    Token::Dot      => IResult::Done(i, Node::PutCh),
                    Token::Comma    => IResult::Done(i, Node::GetCh),
                    Token::LBracket => map!(i, parse_loop, |nodes| Node::Loop(nodes)),
                    Token::RBracket => IResult::Error(error_code!(ErrorKind::Custom(42))),
                }
            } else {
                IResult::Error(error_code!(ErrorKind::Custom(42)))
            }
        },
    }
}

pub fn parse(i: &[u8]) -> Result<Vec<Node>, ErrorKind<u32>> {
    many0!(i, parse_node).to_result()
}

#[cfg(test)]
mod tests {
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
