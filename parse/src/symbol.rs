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
    Pipe,
    Tilde,
    At,
    Hash,
}

impl Symbol {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "=>" => Some(Symbol::Arrow),
            "->" => Some(Symbol::FatArrow),
            "==" => Some(Symbol::EqEq),
            "!=" => Some(Symbol::NotEq),
            "<=" => Some(Symbol::LtEq),
            ">=" => Some(Symbol::GtEq),
            "&&" => Some(Symbol::AndAnd),
            "||" => Some(Symbol::OrOr),
            "+" => Some(Symbol::Plus),
            "-" => Some(Symbol::Minus),
            "*" => Some(Symbol::Star),
            "/" => Some(Symbol::Slash),
            "%" => Some(Symbol::Percent),
            "^" => Some(Symbol::Caret),
            "!" => Some(Symbol::Bang),
            "<" => Some(Symbol::Lt),
            ">" => Some(Symbol::Gt),
            "=" => Some(Symbol::Eq),
            "&" => Some(Symbol::Amp),
            "|" => Some(Symbol::Pipe),
            "~" => Some(Symbol::Tilde),
            "@" => Some(Symbol::At),
            "#" => Some(Symbol::Hash),
            _ => None,
        }
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
            Symbol::Pipe => write!(f, "|"),
            Symbol::Tilde => write!(f, "~"),
            Symbol::At => write!(f, "@"),
            Symbol::Hash => write!(f, "#"),
        }
    }
}
