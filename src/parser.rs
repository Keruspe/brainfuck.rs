use ast::{Block, Node};
use nom::{IResult, ErrorKind};

fn skip_unknown_bf(i: &[u8]) -> IResult<&[u8], &[u8]> {
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

macro_rules! tag_bf (
    ($i: expr, $tag: expr) => ({
        use $crate::parser::skip_unknown_bf;
        sep!($i, skip_unknown_bf, tag!($tag))
    });
);

named!(pub lshift<Node>,     do_parse!(tag_bf!("<") >> (Node::LShift)));
named!(pub rshift<Node>,     do_parse!(tag_bf!(">") >> (Node::RShift)));
named!(pub plus<Node>,       do_parse!(tag_bf!("+") >> (Node::Inc)));
named!(pub minus<Node>,      do_parse!(tag_bf!("-") >> (Node::Dec)));
named!(pub dot<Node>,        do_parse!(tag_bf!(".") >> (Node::PutCh)));
named!(pub comma<Node>,      do_parse!(tag_bf!(",") >> (Node::GetCh)));
named!(parse_loop<Node>, do_parse!(tag_bf!("[") >> block: map!(many_till!(call!(node), tag_bf!("]")), |(nodes, _)| From::from(nodes)) >> (Node::Loop(block))));
named!(node<Node>,       do_parse!(node: alt!(lshift | rshift | plus | minus | dot | comma | parse_loop) >> (node)));

pub fn parse(i: &[u8]) -> Result<Block, ErrorKind<u32>> {
    map!(i, many0!(node), From::from).to_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    use ast::Node;
    use nom::{IResult, Needed};

    const EMPTY: &'static [u8] = b"";

    #[test]
    fn test_lshift() {
        assert_eq!(lshift(b"<"),   IResult::Done(EMPTY, Node::LShift));
        assert_eq!(lshift(b"a"),   IResult::Incomplete(Needed::Size(2)));
        assert_eq!(lshift(b"a<b"), IResult::Done(EMPTY, Node::LShift));
    }

    #[test]
    fn test_rshift() {
        assert_eq!(rshift(b">"),   IResult::Done(EMPTY, Node::RShift));
        assert_eq!(rshift(b"a"),   IResult::Incomplete(Needed::Size(2)));
        assert_eq!(rshift(b"a>b"), IResult::Done(EMPTY, Node::RShift));
    }

    #[test]
    fn test_plus() {
        assert_eq!(plus(b"+"),   IResult::Done(EMPTY, Node::Inc));
        assert_eq!(plus(b"a"),   IResult::Incomplete(Needed::Size(2)));
        assert_eq!(plus(b"a+b"), IResult::Done(EMPTY, Node::Inc));
    }

    #[test]
    fn test_minus() {
        assert_eq!(minus(b"-"),   IResult::Done(EMPTY, Node::Dec));
        assert_eq!(minus(b"a"),   IResult::Incomplete(Needed::Size(2)));
        assert_eq!(minus(b"a-b"), IResult::Done(EMPTY, Node::Dec));
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(b"."),   IResult::Done(EMPTY, Node::PutCh));
        assert_eq!(dot(b"a"),   IResult::Incomplete(Needed::Size(2)));
        assert_eq!(dot(b"a.b"), IResult::Done(EMPTY, Node::PutCh));
    }

    #[test]
    fn test_comma() {
        assert_eq!(comma(b","),   IResult::Done(EMPTY, Node::GetCh));
        assert_eq!(comma(b"a"),   IResult::Incomplete(Needed::Size(2)));
        assert_eq!(comma(b"a,b"), IResult::Done(EMPTY, Node::GetCh));
    }
}
