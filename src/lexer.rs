#[derive(PartialEq, Clone, Debug)]
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

    NotOp, //*
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

    Eq, //*
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

    If,
    Else,
    Fn,
    For,
    Let,
    Return,
    Struct,
    Impl,
    Trait,
    Enum,
    Underscore,
    Mut, //afayutmyfmtuyfamt

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

    fn matchs(&mut self, other: &str) -> bool {
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

    fn matchc(&mut self, other: char) -> bool {
        let mut tmp = self.buff.clone();
        if tmp.next() == Some(other) {
            self.buff.next();
            true
        } else {
            false
        }
    }
}

impl Iterator for Tokens<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if let Some(token) = self.token.clone() {
            self.token = None;
            return Some(token);
        }
        Some(match self.buff.next()? {
            '\n' | ' ' | '\t' => self.next()?,

            '>' if self.matchs(">=") => Token::ShRightEq,
            '<' if self.matchs("<=") => Token::ShLeftEq,
            '>' if self.matchc('>') => Token::ShRightOp,
            '<' if self.matchc('<') => Token::ShLeftOp,
            '-' if self.matchc('>') => Token::SmArrow,
            '=' if self.matchc('>') => Token::LgArrow,
            '+' if self.matchc('+') => Token::PlusOneOp,
            '-' if self.matchc('-') => Token::MinusOneOp,
            '&' if self.matchc('&') => Token::AndOp,
            '|' if self.matchc('|') => Token::OrOp,
            '=' if self.matchc('=') => Token::EqCmp,
            '!' if self.matchc('=') => Token::NeqCmp,
            '<' if self.matchc('=') => Token::LeCmp,
            '>' if self.matchc('=') => Token::GeCmp,
            '+' if self.matchc('=') => Token::PlusEq,
            '-' if self.matchc('=') => Token::MinusEq,
            '%' if self.matchc('=') => Token::ModEq,
            '*' if self.matchc('=') => Token::TimesEq,
            '/' if self.matchc('=') => Token::DivEq,
            '&' if self.matchc('=') => Token::AndEq,
            '|' if self.matchc('=') => Token::OrEq,
            '^' if self.matchc('=') => Token::XorEq,

            '/' if self.matchc('/') => {
                loop {
                    if self.buff.next() == None || self.buff.next() == Some('\n') {
                        break;
                    }
                }
                self.next()?
            }

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
                let next = match self.buff.next() {
                    // still not right, but close
                    Some('\\') => parse_escape(self.buff.next()),
                    n => n,
                };
                if self.buff.next() != Some('\'') {
                    panic!("character without ending single quote")
                }
                match next {
                    Some(char) => Token::CharLiteral(char),
                    None => panic!("\' at the end of program"),
                }
            }
            '"' => {
                let mut buff = String::new();
                loop {
                    match self.buff.next() {
                        Some('"') => break,
                        Some('\\') => buff.push(parse_escape(self.buff.next()).unwrap()),
                        Some(char) => buff.push(char),
                        None => panic!("evil quote at end of file"),
                    }
                }
                Token::StringLiteral(buff)
            }
            n @ ('a'..='z' | 'A'..='Z' | '_') => {
                let mut buff: String = String::new();
                buff.push(n);
                while matches!(
                    self.peek_char(),
                    Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\\')
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
                    "struct" => Token::Struct,
                    "trait" => Token::Trait,
                    "enum" => Token::Enum,
                    "impl" => Token::Impl,
                    "_" => Token::Underscore,
                    "true" => Token::BoolLiteral(true),
                    "false" => Token::BoolLiteral(false),
                    name => Token::Ident(name.to_string()),
                }
            }
            n @ '0'..='9' => {
                let mut num: String = String::new();
                let radix = match self.peek_char() {
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
                while let Some('0'..='9' | 'a'..='z' | 'A'..='Z' | '_') = self.peek_char() {
                    if self.peek_char() == Some('_') {
                        self.buff.next();
                        continue;
                    }
                    num.push(self.buff.next().unwrap());
                }
                let t = usize::from_str_radix(&*num, radix);
                match t {
                    Ok(n) => Token::IntLiteral(n),
                    Err(e) => todo!("no error handling for lexer number interpretation: {}", e),
                }
            }
            n => panic!("Unrecognized character in program"),
        })
    }
}

fn parse_escape(val: Option<char>) -> Option<char> {
    match val {
        Some('\\') => Some('\\'),
        Some('n') => Some('\n'),
        Some('t') => Some('\t'),
        Some('\"') => Some('"'),
        Some('\'') => Some('\''),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod numbers {}

    mod tokens {
        use super::*;
        macro_rules! token {
            ($name: ident, $x: literal, $y:expr) => {
                #[test]
                pub fn $name() {
                    let lexer = Tokens::new($x);
                    assert_eq!(lexer.collect::<Vec<Token>>(), vec![$y]);
                }
            };
        }

        // TODO: error handling will make this the wrong solution
        macro_rules! invalid {
            ($name: ident, $x: literal) => {
                #[test]
                #[should_panic]
                pub fn $name() {
                    let lexer = Tokens::new($x);
                    lexer.for_each(drop);
                }
            };
        }

        invalid!(single_apostrophe, "'");
        invalid!(single_quote, "\"");
        invalid!(escape_char, "'\\'");
        invalid!(escape_string, "\"\\\"");
        invalid!(radix, "0b3");
        invalid!(number_syntax, "0m");
        invalid!(
            extremely_large_integer,
            "10000000000000000000000000000000000"
        );

        #[test]
        pub fn empty() {
            let mut lexer = Tokens::new("");
            assert_eq!(lexer.next(), None);
        }

        #[test]
        pub fn comment() {
            let mut lexer = Tokens::new("// hi");
            assert_eq!(lexer.next(), None);
        }

        #[test]
        pub fn whitespace() {
            let mut lexer = Tokens::new("\n\t\n\t                 ");
            assert_eq!(lexer.next(), None);
        }

        token!(dot, ".", Token::Dot);
        token!(comma, ",", Token::Comma);
        token!(question, "?", Token::Question);
        token!(colon, ":", Token::Colon);
        token!(semicolon, ";", Token::Semicolon);
        token!(star, "*", Token::Star);
        token!(hashtag, "#", Token::Hashtag);
        token!(at, "@", Token::At);
        token!(pipe, "|", Token::Pipe);
        token!(backslash, "\\", Token::Backslash);
        token!(sm_arrow, "->", Token::SmArrow);
        token!(lg_arrow, "=>", Token::LgArrow);
        token!(not, "!", Token::NotOp);
        token!(plus_one, "++", Token::PlusOneOp);
        token!(minus_one, "--", Token::MinusOneOp);
        token!(and, "&", Token::BAndOp);
        token!(shift_left, "<<", Token::ShLeftOp);
        token!(shift_right, ">>", Token::ShRightOp);
        token!(lazy_and, "&&", Token::AndOp);
        token!(lazy_or, "||", Token::OrOp);
        token!(xor, "^", Token::XorOp);
        token!(modulus, "%", Token::ModOp);
        token!(plus, "+", Token::PlusOp);
        token!(minus, "-", Token::MinusOp);
        token!(divide, "/", Token::DivOp);
        token!(equals_comparison, "==", Token::EqCmp);
        token!(not_equals, "!=", Token::NeqCmp);
        token!(less_than, "<", Token::LtCmp);
        token!(less_equal, "<=", Token::LeCmp);
        token!(greater_than, ">", Token::GtCmp);
        token!(greater_equal, ">=", Token::GeCmp);
        token!(equals, "=", Token::Eq);
        token!(plus_equals, "+=", Token::PlusEq);
        token!(minus_equals, "-=", Token::MinusEq);
        token!(times_equals, "*=", Token::TimesEq);
        token!(divided_equals, "/=", Token::DivEq);
        token!(mod_equals, "%=", Token::ModEq);
        token!(and_equals, "&=", Token::AndEq);
        token!(or_equals, "|=", Token::OrEq);
        token!(xor_equals, "^=", Token::XorEq);
        token!(shift_left_equals, "<<=", Token::ShLeftEq);
        token!(shift_right_equals, ">>=", Token::ShRightEq);
        token!(open_paren, "(", Token::OParen);
        token!(closed_paren, ")", Token::CParen);
        token!(open_bracket, "[", Token::OBracket);
        token!(closed_bracket, "]", Token::CBracket);
        token!(open_curly, "{", Token::OCurly);
        token!(closed_curly, "}", Token::CCurly);
        token!(if_keyword, "if", Token::If);
        token!(else_keyword, "else", Token::Else);
        token!(fn_keyword, "fn", Token::Fn);
        token!(for_keyword, "for", Token::For);
        token!(let_keyword, "let", Token::Let);
        token!(return_keyword, "return", Token::Return);
        token!(true_keyword, "true", Token::BoolLiteral(true));
        token!(false_keyword, "false", Token::BoolLiteral(false));
        token!(
            double_keyword_ident,
            "iflet",
            Token::Ident("iflet".to_string())
        );
        token!(ident, "x", Token::Ident("x".to_string()));
        token!(zero, "0", Token::IntLiteral(0));
        token!(small, "15", Token::IntLiteral(15));
        token!(int_underscore, "1_1", Token::IntLiteral(11));
        token!(large_integer, "40000000000", Token::IntLiteral(40000000000));
        token!(
            very_large_integer,
            "1000000000000000",
            Token::IntLiteral(1000000000000000)
        );
        token!(char, "'a'", Token::CharLiteral('a'));
        token!(char_apostrophe, "'\\''", Token::CharLiteral('\'')); // '\''
        token!(char_line, "'\\n'", Token::CharLiteral('\n')); // '\n'
        token!(char_tab, "'\\t'", Token::CharLiteral('\t')); // '\t'
        token!(char_quote, "'\"'", Token::CharLiteral('"')); // '"'
        token!(char_quote_escaped, "'\\\"'", Token::CharLiteral('"')); // '\"'
        token!(string, "\"str\"", Token::StringLiteral("str".to_string()));

        token!(
            comment_plus,
            "// test
+",
            Token::PlusOp
        );
    }

    #[test]
    pub fn test_string() {
        let string = "\"here is a string\"";
        let mut lexer = Tokens::new(string);
        assert_eq!(
            lexer.next(),
            Some(Token::StringLiteral("here is a string".into()))
        );
    }

    #[test]
    pub fn test_int() {
        let hex = "0xfe15";
        let mut lexer = Tokens::new(hex);
        assert_eq!(lexer.next(), Some(Token::IntLiteral(0xfe15)));
    }

    #[test]
    pub fn test_fn() {
        let function = "
fn test() {
    let x = true;
    if (x) {
        test2();
    }
}
";
        let lexer = Tokens::new(function);
        let tokens = lexer.collect::<Vec<Token>>();
        let correct = vec![
            Token::Fn,
            Token::Ident("test".into()),
            Token::OParen,
            Token::CParen,
            Token::OCurly,
            Token::Let,
            Token::Ident("x".into()),
            Token::Eq,
            Token::BoolLiteral(true),
            Token::Semicolon,
            Token::If,
            Token::OParen,
            Token::Ident("x".into()),
            Token::CParen,
            Token::OCurly,
            Token::Ident("test2".into()),
            Token::OParen,
            Token::CParen,
            Token::Semicolon,
            Token::CCurly,
            Token::CCurly,
        ];
        assert_eq!(tokens, correct)
    }
}
