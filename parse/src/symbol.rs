#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Symbol {
    /* Multi-character symbols */
    Arrow,
    FatArrow,
    EqEq,
    NotEq,
    LtEq,
    GtEq,
    AndAnd,
    OrOr,

    /* Single-character symbols */
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Bang,
    Lt,
    Gt,
    Eq,
    Amp,
    Dot,
    Comma,
    Pipe,
    Tilde,
    At,
    Hash,
}

impl Symbol {
    pub fn from_parts(c: char, b: Option<char>) -> Option<Self> {
        Some(match (c, b) {
            /* Multi-character symbols */
            ('-', Some('>')) => Self::Arrow,
            ('=', Some('>')) => Self::FatArrow,
            ('=', Some('=')) => Self::EqEq,
            ('!', Some('=')) => Self::NotEq,
            ('<', Some('=')) => Self::LtEq,
            ('>', Some('=')) => Self::GtEq,
            ('&', Some('&')) => Self::AndAnd,
            ('|', Some('|')) => Self::OrOr,

            /* Single-character symbols */
            ('(', _) => Self::LeftParen,
            (')', _) => Self::RightParen,
            ('{', _) => Self::LeftBrace,
            ('}', _) => Self::RightBrace,
            ('[', _) => Self::LeftBracket,
            (']', _) => Self::RightBracket,
            ('+', _) => Self::Plus,
            ('-', _) => Self::Minus,
            ('*', _) => Self::Star,
            ('/', _) => Self::Slash,
            ('%', _) => Self::Percent,
            ('^', _) => Self::Caret,
            ('!', _) => Self::Bang,
            ('<', _) => Self::Lt,
            ('>', _) => Self::Gt,
            ('=', _) => Self::Eq,
            ('&', _) => Self::Amp,
            ('.', _) => Self::Dot,
            (',', _) => Self::Comma,
            ('|', _) => Self::Pipe,
            ('~', _) => Self::Tilde,
            ('@', _) => Self::At,
            ('#', _) => Self::Hash,
            _ => return None,
        })
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Symbol::Arrow => write!(f, "=>"),
            Symbol::FatArrow => write!(f, "->"),
            Symbol::EqEq => write!(f, "=="),
            Symbol::NotEq => write!(f, "!="),
            Symbol::LtEq => write!(f, "<="),
            Symbol::GtEq => write!(f, ">="),
            Symbol::AndAnd => write!(f, "&&"),
            Symbol::OrOr => write!(f, "||"),
            Symbol::LeftParen => write!(f, "("),
            Symbol::RightParen => write!(f, ")"),
            Symbol::LeftBrace => write!(f, "{{"),
            Symbol::RightBrace => write!(f, "}}"),
            Symbol::LeftBracket => write!(f, "["),
            Symbol::RightBracket => write!(f, "]"),
            Symbol::Plus => write!(f, "+"),
            Symbol::Minus => write!(f, "-"),
            Symbol::Star => write!(f, "*"),
            Symbol::Slash => write!(f, "/"),
            Symbol::Percent => write!(f, "%"),
            Symbol::Caret => write!(f, "^"),
            Symbol::Bang => write!(f, "!"),
            Symbol::Lt => write!(f, "<"),
            Symbol::Gt => write!(f, ">"),
            Symbol::Eq => write!(f, "="),
            Symbol::Amp => write!(f, "&"),
            Symbol::Dot => write!(f, "."),
            Symbol::Comma => write!(f, ","),
            Symbol::Pipe => write!(f, "|"),
            Symbol::Tilde => write!(f, "~"),
            Symbol::At => write!(f, "@"),
            Symbol::Hash => write!(f, "#"),
        }
    }
}
