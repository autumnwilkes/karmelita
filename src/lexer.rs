pub struct Tokens {}

pub enum Token {
    Dot,
    DotDot,
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
    BLeftOp,
    BRightOp,

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
    BLeftEq,
    BRightEq,

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

impl Iterator for Tokens {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
