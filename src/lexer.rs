#[allow(unused)]
pub enum Token {
    Dot,
    Comma,
    Question,
    Colon,
    Semicolon,
    Wild,
    Star,
    Hashtag,
    At,
    Pipe,
    Backslash,
    SQuote,
    DQuote,

    SmArrow,
    LgArrow,

    NotOp,
    PlusOneOp,
    MinusOneOp,

    BAndOp,
    BOrOp,
    BXorOp,

    ShLeftOp,
    ShRightOp,

    AndOp,
    OrOp,
    XorOp,

    ModOp,
    PlusOp,
    MinusOp,
    DivOp,

    EqCmp,
    NeqCmp,
    LtCmp,
    LeCmp,
    GtCmp,
    GeCmp,

    Eq,
    PlusEq,
    MinusEq,
    TimesEq,
    DivEq,
    ModEq,

    AndEq,
    OrEq,
    XorEq,
    ShLeftEq,
    ShRightEq,

    OParen,
    CParen,
    OBracket,
    CBracket,
    OCurly,
    CCurly,

    // Add more keywords lol
    If,
    Else,
    Fn,
    For,
    Let,

    BoolLiteral { val: bool },
    IntLiteral { val: i32 },
    CharLiteral { val: char },
    StringLiteral { val: String }, // &str? something else?
    Ident { name: String },
}

pub struct Tokens<'a> {
    buff: std::str::Chars<'a>,
}

impl<'a> Tokens<'a> {
    fn new(buff: &'a str) -> Self {
        Self { buff: buff.chars() }
    }

    fn peek(&self) -> Option<char> {
        self.buff.clone().next()
    }

    fn peek_nth(&self, n: usize) -> Option<char> {
        let mut tmp = self.buff.clone();
        for _ in 0..n - 1 {
            tmp.next();
        }
        tmp.next()
    }
}

macro_rules! token {
    ($i:ident) => {
        Some(Token::$i)
    };
}

impl Iterator for Tokens<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        println!();
        match self.buff.next().unwrap_or(0.into()) {
            '\0' => None,
            '\n' | ' ' | '\t' => self.next(),
            '.' => token!(Dot),
            ',' => token!(Comma),
            '?' => token!(Question),
            ':' => token!(Colon),
            ';' => token!(Semicolon),
            '_' => token!(Wild),
            '*' => token!(Star),
            '#' => token!(Hashtag),
            '@' => token!(At),
            '|' => token!(Pipe),
            '!' => token!(NotOp),
            '\\' => token!(Backslash),
            '\'' => token!(SQuote),
            '"' => token!(DQuote),
            '-' if self.peek() == '>' => token!(SmArrow),
            '=' if self.peek() == '>' => token!(LgArrow),
            '+' if self.peek() == '+' => token!(PlusOneOp),
            '-' if self.peek() == '-' => token!(MinusOneOp),

            _ => todo!(),
        }
    }
}
