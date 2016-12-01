use nom::IResult;

pub fn skip_unknown(i: &[u8]) -> IResult<&[u8], &[u8]> {
    if i.length() == 0 {
        IResult::Done::<&[u8], &[u8]>(i, i.slice(0..0))
    } else {
        match i.iter_indices().position(|(_, item)| {
            match *item as char {
                '>'|'<'|'+'|'-'|'.'|','|'['|']' => true,
                _                               => false,
            }
        }) {
            Some(index) => IResult::Done(i.slice(index..), i.slice(..index)),
            None        => IResult::Done(i.slice(i.input_len()..), i),
        }
    }
}

macro_rules! strip_unknown (
    ($i: expr, $($args:tt)*) => ({
        sep!($i, skip_unknown, $($args)*)
    });
);

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

        assert_eq!(foo, IResult::Done(EMPTY, expected));
    }
}
