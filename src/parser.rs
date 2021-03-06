pub use nom::types::CompleteByteSlice;

use crate::ast::{Block, Node};
use nom::{alt, call, complete, do_parse, eof, is_not, many0, many_till, map, preceded, tag};

const ALLOWED: &'static str = "<>+-.,[]";

pub fn bf_skip_unknown<T>(i: T) -> nom::IResult<T, T, u32>
    where T:            nom::InputTake+nom::InputTakeAtPosition+Copy,
          &'static str: nom::FindToken<<T as nom::InputTakeAtPosition>::Item> {
    is_not!(i, ALLOWED).or_else(|e| match e {
        nom::Err::Incomplete(size) => Err(nom::Err::Incomplete(size)),
        _                          => Ok((i, i.take(0))),
    })
}

pub fn bf_skip_unknown_no_incomplete<T>(i: T) -> nom::IResult<T, T, u32>
    where T:            nom::InputTake+nom::InputTakeAtPosition+Copy,
          &'static str: nom::FindToken<<T as nom::InputTakeAtPosition>::Item> {
    bf_skip_unknown(i).or(Ok((i, i.take(0))))
}

macro_rules! bf_tag (
    ($i: expr, $tag: expr) => ({
        do_parse!($i,
                 bf_skip_unknown               >>
            res: tag!($tag)                    >>
                 bf_skip_unknown_no_incomplete >>
            (res)
        )
    });
);

macro_rules! bf_named {
    (pub $name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
        fn $name<T>( i: T ) -> nom::IResult<T, $o, u32>
            where T:            nom::InputTake+nom::InputTakeAtPosition+nom::AtEof+nom::Compare<&'static str>+Copy+PartialEq,
                  &'static str: nom::FindToken<<T as nom::InputTakeAtPosition>::Item> {
            $submac!(i, $($args)*)
        }
    );
}

bf_named!(pub lshift<Node>,     do_parse!(bf_tag!("<") >> (Node::LShift)));
bf_named!(pub rshift<Node>,     do_parse!(bf_tag!(">") >> (Node::RShift)));
bf_named!(pub plus<Node>,       do_parse!(bf_tag!("+") >> (Node::Inc)));
bf_named!(pub minus<Node>,      do_parse!(bf_tag!("-") >> (Node::Dec)));
bf_named!(pub dot<Node>,        do_parse!(bf_tag!(".") >> (Node::PutCh)));
bf_named!(pub comma<Node>,      do_parse!(bf_tag!(",") >> (Node::GetCh)));
bf_named!(pub parse_loop<Node>, preceded!(bf_tag!("["), map!(many_till!(call!(node), bf_tag!("]")), |(nodes, _)| Node::Loop(From::from(nodes)))));
bf_named!(pub node<Node>,       alt!(lshift | rshift | plus | minus | dot | comma | parse_loop));

pub fn parse<T>(i: T) -> Result<Block, nom::Err<T, u32>>
    where T:            nom::InputTake+nom::InputTakeAtPosition+nom::InputLength+nom::AtEof+nom::Compare<&'static str>+Copy+PartialEq,
          &'static str: nom::FindToken<<T as nom::InputTakeAtPosition>::Item> {
    do_parse!(i,
        res: map!(many0!(complete!(node)), From::from) >>
             eof!()                                    >>
        (res)
    ).map(|(_, block)| block)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ast::{Block, Node};
    use nom::{self, Needed};

    const EMPTY: &'static [u8] = b"";

    #[test]
    fn test_lshift() {
        assert_eq!(lshift(&b"<"[..]),   Ok((EMPTY, Node::LShift)));
        assert_eq!(lshift(&b"a"[..]),   Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(lshift(&b"a<b"[..]), Ok((&b"b"[..], Node::LShift)));
    }

    #[test]
    fn test_rshift() {
        assert_eq!(rshift(&b">"[..]),   Ok((EMPTY, Node::RShift)));
        assert_eq!(rshift(&b"a"[..]),   Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(rshift(&b"a>b"[..]), Ok((&b"b"[..], Node::RShift)));
    }

    #[test]
    fn test_plus() {
        assert_eq!(plus(&b"+"[..]),   Ok((EMPTY, Node::Inc)));
        assert_eq!(plus(&b"a"[..]),   Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(plus(&b"a+b"[..]), Ok((&b"b"[..], Node::Inc)));
    }

    #[test]
    fn test_minus() {
        assert_eq!(minus(&b"-"[..]),   Ok((EMPTY, Node::Dec)));
        assert_eq!(minus(&b"a"[..]),   Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(minus(&b"a-b"[..]), Ok((&b"b"[..], Node::Dec)));
    }

    #[test]
    fn test_dot() {
        assert_eq!(dot(&b"."[..]),   Ok((EMPTY, Node::PutCh)));
        assert_eq!(dot(&b"a"[..]),   Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(dot(&b"a.b"[..]), Ok((&b"b"[..], Node::PutCh)));
    }

    #[test]
    fn test_comma() {
        assert_eq!(comma(&b","[..]),   Ok((EMPTY, Node::GetCh)));
        assert_eq!(comma(&b"a"[..]),   Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(comma(&b"a,b"[..]), Ok((&b"b"[..], Node::GetCh)));
    }

    #[test]
    fn test_parse_loop() {
        let nodes1 = vec![Node::RShift, Node::Inc, Node::LShift, Node::PutCh];
        let nodes2 = vec![Node::RShift, Node::Inc, Node::LShift, Node::PutCh];
        assert_eq!(parse_loop(&b"[>+<.]"[..]),            Ok((EMPTY, Node::Loop(From::from(nodes1)))));
        assert_eq!(parse_loop(&b"a"[..]),                 Err(nom::Err::Incomplete(Needed::Size(1))));
        assert_eq!(parse_loop(&b"a[ b>  +e<//.'r]@"[..]), Ok((&b"@"[..], Node::Loop(From::from(nodes2)))));
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
        assert_eq!(parse(CompleteByteSlice(&b"abc[<>]."[..])), Ok(block));
        assert_eq!(parse(CompleteByteSlice(EMPTY)),            Ok(Block::new()));
    }
}
