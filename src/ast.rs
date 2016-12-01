pub enum Token {
    LShift,
    RShift,
    Plus,
    Minus,
    Dot,
    Comma,
    LBracket,
    RBracket,
}

impl Token {
    pub fn from_char(ch: char) -> Option<Token> {
        match ch {
            '<' => Some(Token::LShift),
            '>' => Some(Token::RShift),
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '.' => Some(Token::Dot),
            ',' => Some(Token::Comma),
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),
            _   => None,
        }
    }
}

pub enum Node {
    LShift,
    RShift,
    Inc,
    Dec,
    PutCh,
    GetCh,
    Loop(Vec<Node>),
}
