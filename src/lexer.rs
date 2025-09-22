#[derive(PartialEq, Clone)]
pub enum Token {
    Dot,
    Comma,
    Question,
    Colon,
    Semicolon,
    Star,
    Hashtag,
    At,
    Pipe,
    Backslash,

    SmArrow,
    LgArrow,

    NotOp,
    PlusOneOp,
    MinusOneOp,

    BAndOp,

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
    Return,

    BoolLiteral(bool),
    IntLiteral(usize),
    CharLiteral(char),
    StringLiteral(String),
    Ident(String),
}

#[derive(Clone)]
pub struct Tokens<'a> {
    buff: std::str::Chars<'a>,
    token: Option<Token>,
}

impl<'a> Tokens<'a> {
    pub fn new(buff: &'a str) -> Self {
        let x: String;
        Self {
            buff: buff.chars(),
            token: None,
        }
    }

    pub fn peek(&self) -> Option<Token> {
        if let Some(token) = &self.token {
            return Some(token.clone());
        }
        self.clone().next()
    }

    fn peek_char(&self) -> Option<char> {
        self.buff.clone().next()
    }

    fn matches(&mut self, other: &str) -> bool {
        let mut tmp = self.buff.clone();
        let mut buff: String = String::new();
        for i in 0..other.len() {
            match tmp.next() {
                Some(c) => buff.insert(i, c),
                None => return false,
            }
        }
        if buff == other {
            for _ in 0..other.len() {
                self.buff.next();
            }
        }
        buff == other
    }

    fn match_char(&mut self, other: char) -> bool {
        let mut tmp = self.buff.clone();
        if tmp.next() == Some(other) {
            self.buff.next();
            true
        } else {
            false
        }
    }
}

impl Iterator for Tokens {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(token) = self.token.clone() {
            self.token = None;
            return Some(token);
        }
        Some(match self.buff.next()? {
            '\n' | ' ' | '\t' => self.next()?,

            '>' if self.matches(">=") => Token::ShRightEq,
            '<' if self.matches("<=") => Token::ShLeftEq,
            '>' if self.match_char('>') => Token::ShRightOp,
            '<' if self.match_char('<') => Token::ShLeftOp,
            '-' if self.match_char('>') => Token::SmArrow,
            '=' if self.match_char('>') => Token::LgArrow,
            '+' if self.match_char('+') => Token::PlusOneOp,
            '-' if self.match_char('-') => Token::MinusOneOp,
            '&' if self.match_char('&') => Token::AndOp,
            '|' if self.match_char('|') => Token::OrOp,
            '=' if self.match_char('=') => Token::EqCmp,
            '!' if self.match_char('=') => Token::NeqCmp,
            '<' if self.match_char('=') => Token::LeCmp,
            '>' if self.match_char('=') => Token::GeCmp,
            '+' if self.match_char('=') => Token::PlusEq,
            '-' if self.match_char('=') => Token::MinusEq,
            '%' if self.match_char('=') => Token::ModEq,
            '*' if self.match_char('=') => Token::TimesEq,
            '/' if self.match_char('=') => Token::DivEq,
            '&' if self.match_char('=') => Token::AndEq,
            '|' if self.match_char('=') => Token::OrEq,
            '^' if self.match_char('=') => Token::XorEq,

            '.' => Token::Dot,
            ',' => Token::Comma,
            '?' => Token::Question,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '*' => Token::Star,
            '#' => Token::Hashtag,
            '@' => Token::At,
            '|' => Token::Pipe,
            '!' => Token::NotOp,
            '\\' => Token::Backslash,
            '&' => Token::BAndOp,
            '^' => Token::XorOp,
            '%' => Token::ModOp,
            '/' => Token::DivOp,
            '+' => Token::PlusOp,
            '-' => Token::MinusOp,
            '<' => Token::LtCmp,
            '>' => Token::GtCmp,
            '=' => Token::Eq,
            '(' => Token::OParen,
            ')' => Token::CParen,
            '{' => Token::OCurly,
            '}' => Token::CCurly,
            '[' => Token::OBracket,
            ']' => Token::CBracket,

            '\'' => {
                let next = self.buff.next();
                if self.buff.next() != Some('\'') {
                    panic!("character without ending single quote")
                }
                match next {
                    Some(char) => Token::CharLiteral(char),
                    None => panic!("\' at the end of program"),
                }
            }

            '"' => {
                //TODO: this is being done awfully and has no handling for open quotes without
                //closing them
                let mut buff = String::new();
                while let Some(char) = self.buff.next() {
                    if char == '"' {
                        break;
                    }
                    buff.push(char);
                }
                Token::StringLiteral(buff)
            }

            n @ ('a'..='z' | 'A'..='Z' | '_') => {
                let mut buff: String = String::new();
                buff.push(n);

                while matches!(
                    self.peek_char(),
                    Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_')
                ) {
                    buff.push(self.buff.next().unwrap());
                }
                match &*buff {
                    "if" => Token::If,
                    "fn" => Token::Fn,
                    "else" => Token::Else,
                    "for" => Token::For,
                    "let" => Token::Let,
                    "return" => Token::Return,
                    "true" => Token::BoolLiteral(true),
                    "false" => Token::BoolLiteral(false),
                    name => Token::Ident(name.to_string()),
                }
            }
            n @ '0'..='9' => {
                let mut num: String = String::new();
                num.push(n);
                let base = match self.peek_char() {
                    Some('x') if n == '0' => {
                        self.buff.next();
                        16
                    }
                    Some('b') if n == '0' => {
                        self.buff.next();
                        2
                    }
                    Some('o') if n == '0' => {
                        self.buff.next();
                        8
                    }
                    _ => {
                        num.push(n);
                        10
                    }
                };
                while let Some('0'..='9' | 'a'..='f') = self.peek_char() {
                    num.push(self.buff.next().unwrap());
                }
                let t = usize::from_str_radix(&*num, base);
                match t {
                    Ok(n) => Token::IntLiteral(n),
                    Err(e) => todo!("no error handling for lexer number interpretation"),
                }
            }
            n => panic!("Unrecognized character in program"),
        })
    }
}
