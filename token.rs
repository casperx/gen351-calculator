use std::fmt;

#[derive(Clone)]
pub enum Token {
    End,

    Line,

    Plus,
    Dash,

    Star,
    Slash,
    Percent,

    StarStar,

    And,
    Pipe,
    Hat,

    Worm,
    Exc,

    LessLess,
    MoreMore,

    OpenParen,
    CloseParen,

    Num(i64),
    Str(String),

    Name(String)
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let res = match self {
            Token::End => "#",

            Token::Line => ";",

            Token::Plus => "+",
            Token::Dash => "-",

            Token::Star  => "*",
            Token::Slash => "/",

            Token::Percent => "%",

            Token::StarStar => "**",

            Token::And  => "&",
            Token::Pipe => "|",
            Token::Hat  => "^",

            Token::Worm => "~",
            Token::Exc => "!",

            Token::LessLess => "<<",
            Token::MoreMore => ">>",

            Token::OpenParen  => "(",
            Token::CloseParen => ")",

            Token::Num(val) => return write!(f, "{}", val),
            Token::Str(val) => return write!(f, r#""{}""#, val),

            Token::Name(name) => return write!(f, "{}", name)
        };
        write!(f, "{}", res)
    }
}
