use ast::*;
use nom::{IResult, ErrorKind};

pub fn skip_unknown_bf(i: &[u8]) -> IResult<&[u8], &[u8]> {
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

named!(lshift<Node>,      do_parse!(tag_bf!("<") >> (Node::LShift)));
named!(rshift<Node>,      do_parse!(tag_bf!(">") >> (Node::RShift)));
named!(plus<Node>,        do_parse!(tag_bf!("+") >> (Node::Inc)));
named!(minus<Node>,       do_parse!(tag_bf!("-") >> (Node::Dec)));
named!(dot<Node>,         do_parse!(tag_bf!(".") >> (Node::PutCh)));
named!(comma<Node>,       do_parse!(tag_bf!(",") >> (Node::GetCh)));
named!(lbracket<Node>,    do_parse!(tag_bf!("[") >> node: map!(parse_loop, |block| Node::Loop(block)) >> (node)));
named!(node<Node>,        do_parse!(node: alt!(lshift | rshift | plus | minus | dot | comma | lbracket) >> (node)));
named!(parse_loop<Block>, do_parse!(block: map!(many_till!(call!(node), tag_bf!("]")), |(nodes, _)| From::from(nodes)) >> (block)));

pub fn parse(i: &[u8]) -> Result<Block, ErrorKind<u32>> {
    map!(i, many0!(node), From::from).to_result()
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    const EMPTY: &'static [u8] = b"";

    #[test]
    fn it_works() {
    }
}
