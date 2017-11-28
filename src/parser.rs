use ast::{Block, Node};
use nom;

const ALLOWED: &'static str = "<>+-.,[]";

pub fn skip_unknown_bf(i: &[u8]) -> Result<(&[u8], &[u8]), nom::Err<&[u8], u32>> {
    is_not!(i, ALLOWED).or(Ok((i, &i[0..0])))
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
named!(pub parse_loop<Node>, preceded!(tag_bf!("["), map!(many_till!(call!(node), tag_bf!("]")), |(nodes, _)| Node::Loop(From::from(nodes)))));
named!(pub node<Node>,       alt!(lshift | rshift | plus | minus | dot | comma | parse_loop));

pub fn parse(i: &[u8]) -> Result<Block, nom::Err<&[u8], u32>> {
    do_parse!(i,
        res: map!(many0!(complete!(node)), From::from) >>
             eof!()                                    >>
        (res)
    ).map(|(_, block)| block)
}

#[cfg(test)]
mod tests {
    use super::*;

    use ast::{Block, Node};
    use nom::{self, Needed};

    const EMPTY: &'static [u8] = b"";

    #[test]
    fn test_lshift() {
        assert_eq!(lshift(&b"<"[..]),   Ok((EMPTY, Node::LShift)));
        assert_eq!(lshift(&b"a"[..]),   Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(lshift(&b"a<b"[..]), Ok((EMPTY, Node::LShift)));
    }

    #[test]
    fn test_rshift() {
        assert_eq!(rshift(&b">"[..]),   Ok((EMPTY, Node::RShift)));
        assert_eq!(rshift(&b"a"[..]),   Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(rshift(&b"a>b"[..]), Ok((EMPTY, Node::RShift)));
    }

    #[test]
    fn test_plus() {
        assert_eq!(plus(&b"+"[..]),   Ok((EMPTY, Node::Inc)));
        assert_eq!(plus(&b"a"[..]),   Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(plus(&b"a+b"[..]), Ok((EMPTY, Node::Inc)));
    }

    #[test]
    fn test_minus() {
        assert_eq!(minus(&b"-"[..]),   Ok((EMPTY, Node::Dec)));
        assert_eq!(minus(&b"a"[..]),   Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(minus(&b"a-b"[..]), Ok((EMPTY, Node::Dec)));
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(&b"."[..]),   Ok((EMPTY, Node::PutCh)));
        assert_eq!(dot(&b"a"[..]),   Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(dot(&b"a.b"[..]), Ok((EMPTY, Node::PutCh)));
    }

    #[test]
    fn test_comma() {
        assert_eq!(comma(&b","[..]),   Ok((EMPTY, Node::GetCh)));
        assert_eq!(comma(&b"a"[..]),   Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(comma(&b"a,b"[..]), Ok((EMPTY, Node::GetCh)));
    }

    #[test]
    fn test_parse_loop() {
        let nodes1 = vec![Node::RShift, Node::Inc, Node::LShift, Node::PutCh];
        let nodes2 = vec![Node::RShift, Node::Inc, Node::LShift, Node::PutCh];
        assert_eq!(parse_loop(&b"[>+<.]"[..]),            Ok((EMPTY, Node::Loop(From::from(nodes1)))));
        assert_eq!(parse_loop(&b"a"[..]),                 Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(parse_loop(&b"a[ b>  +e<//.'r]@"[..]), Ok((EMPTY, Node::Loop(From::from(nodes2)))));
    }

    #[test]
    fn test_parse_nested_loop() {
        let iinodes = vec![Node::Inc,    Node::LShift];
        let inodes  = vec![Node::RShift, Node::Loop(From::from(iinodes)), Node::Dec];
        let nodes   = vec![Node::GetCh,  Node::Loop(From::from(inodes)),  Node::PutCh];
        assert_eq!(parse_loop(&b"[,[>[+<]-].]"[..]), Ok((EMPTY, Node::Loop(From::from(nodes)))));
    }

    #[test]
    fn test_node() {
        let nodes = vec![Node::RShift, Node::Inc, Node::LShift, Node::PutCh];
        assert_eq!(node(&b"<"[..]),      Ok((EMPTY, Node::LShift)));
        assert_eq!(node(&b">"[..]),      Ok((EMPTY, Node::RShift)));
        assert_eq!(node(&b"+"[..]),      Ok((EMPTY, Node::Inc)));
        assert_eq!(node(&b"-"[..]),      Ok((EMPTY, Node::Dec)));
        assert_eq!(node(&b"."[..]),      Ok((EMPTY, Node::PutCh)));
        assert_eq!(node(&b","[..]),      Ok((EMPTY, Node::GetCh)));
        assert_eq!(node(&b"[>+<.]"[..]), Ok((EMPTY, Node::Loop(From::from(nodes)))));
        assert_eq!(node(&b"a"[..]),      Err(nom::Err::Incomplete(Needed::Size(1))));
    }

    #[test]
    fn test_parse() {
        let mut block = Block::new();
        block.push(Node::Loop(From::from(vec![Node::LShift, Node::RShift])));
        block.push(Node::PutCh);
        assert_eq!(parse(&b"abc[<>]."[..]), Ok(block));
        assert_eq!(parse(EMPTY),            Ok(Block::new()));
    }
}
