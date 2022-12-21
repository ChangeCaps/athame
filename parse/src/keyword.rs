#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Keyword {
    As,
    Break,
    Class,
    Const,
    Continue,
    Else,
    Enum,
    False,
    Fn,
    For,
    If,
    In,
    Let,
    Loop,
    Match,
    Pub,
    Return,
    Self_,
    Static,
    Super,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "as" => Some(Keyword::As),
            "break" => Some(Keyword::Break),
            "class" => Some(Keyword::Class),
            "const" => Some(Keyword::Const),
            "continue" => Some(Keyword::Continue),
            "else" => Some(Keyword::Else),
            "enum" => Some(Keyword::Enum),
            "false" => Some(Keyword::False),
            "fn" => Some(Keyword::Fn),
            "for" => Some(Keyword::For),
            "if" => Some(Keyword::If),
            "in" => Some(Keyword::In),
            "let" => Some(Keyword::Let),
            "loop" => Some(Keyword::Loop),
            "match" => Some(Keyword::Match),
            "pub" => Some(Keyword::Pub),
            "return" => Some(Keyword::Return),
            "self" => Some(Keyword::Self_),
            "static" => Some(Keyword::Static),
            "super" => Some(Keyword::Super),
            _ => None,
        }
    }
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Keyword::As => write!(f, "as"),
            Keyword::Break => write!(f, "break"),
            Keyword::Class => write!(f, "class"),
            Keyword::Const => write!(f, "const"),
            Keyword::Continue => write!(f, "continue"),
            Keyword::Else => write!(f, "else"),
            Keyword::Enum => write!(f, "enum"),
            Keyword::False => write!(f, "false"),
            Keyword::Fn => write!(f, "fn"),
            Keyword::For => write!(f, "for"),
            Keyword::If => write!(f, "if"),
            Keyword::In => write!(f, "in"),
            Keyword::Let => write!(f, "let"),
            Keyword::Loop => write!(f, "loop"),
            Keyword::Match => write!(f, "match"),
            Keyword::Pub => write!(f, "pub"),
            Keyword::Return => write!(f, "return"),
            Keyword::Self_ => write!(f, "self"),
            Keyword::Static => write!(f, "static"),
            Keyword::Super => write!(f, "super"),
        }
    }
}
