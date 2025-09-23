use crate::lexer::{Token, Tokens};
use std::rc::Rc;

enum ParseError {}

struct Function {
    id: String,
    params: Vec<Variable>,
    return_type: Type,
    contents: Block,
}

struct Variable {
    ident: String,
    var_type: Type,
}

type Block = Vec<Statement>;

enum Statement {
    If {
        condition: Expression,
        contents: Block,
    },
    Assignment {
        lhs: AssignLhs,
        rhs: Expression,
    },
    Expression(Expression),
    Return(Expression),
    Declaration {
        name: String,
        var_type: Type,
        rhs: Option<Expression>,
    },
}

struct Expression {}

enum AssignLhs {
    Declaration(String, Type),
    Reassign(Rc<Variable>),
}

// could a type be a string OR some other type (ha) of modification on a string?
// Should primitive types be treated differently? (yes! How tho?)
// So like

enum Type {
    UserDefined(String),
    Ptr(Box<Type>),
    PtrMut(Box<Type>),
    Tuple(Vec<Type>),
    Array(Box<Type>, usize),
    Int, // and more...
}

struct Parser<'a> {
    tokens: Tokens<'a>,

    cur_token: Option<Token>,
    next_token: Option<Token>,
}

impl Parser<'_> {
    fn increment_iter(&mut self) -> Option<Token> {
        let tmp = self.cur_token.clone();
        self.cur_token = self.next_token.clone();
        self.next_token = self.tokens.next();
        tmp
    }

    fn parse(mut tokens: Tokens) {
        loop {
            match tokens.next() {
                Some(Token::Fn) => {
                    if let Some(Token::Ident(id)) = tokens.next() {
                        if Some(Token::OParen) != tokens.next() {
                            panic!("Function defined without params");
                        }
                        let params: Vec<Variable> = Vec::new();
                        loop {
                            let next = tokens.next();
                            if next == Some(Token::CParen) {
                                break;
                            }
                            if let Some(Token::Ident(param_id)) = next {
                                if Some(Token::Colon) != tokens.next() {
                                    panic!("function params defined incorrectly");
                                }
                                // if let Some(Token::Ident {})
                            } else {
                                panic!("function params defined incorrectly");
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn parse_variable(name: String, tokens: &mut Tokens) -> Option<Variable> {
        todo!()
    }

    fn parse_type(&mut self) -> Option<Type> {
        match self.next() {
            Some(Token::Ident(id)) => Some(Type::UserDefined(id)),
            _ => todo!(),
        }
    }

    fn parse_block(&mut self) -> Option<Block> {
        let mut block: Block = Vec::new();
        while self.cur_token != Some(Token::CCurly) {
            if self.cur_token == None {
                return None;
            }
            let statement = self.parse_statement();
            if let Some(s) = statement {
                block.push(s);
            }
        }
        Some(block)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            None => None,
            Some(Token::Ident(_)) => {
                if self.next_token == Some(Token::Dot) {
                    if let Some(field) = self.parse_field() {
                        todo!();
                    }
                }
                None
            }
            Some(Token::IntLiteral(_))
            | Some(Token::BoolLiteral(_))
            | Some(Token::CharLiteral(_))
            | Some(Token::StringLiteral(_)) => {
                if let Some(expr) = self.parse_expression() {
                    if self.cur_token == Some(Token::Semicolon) {
                        return Some(Statement::Expression(expr));
                    }
                }
                None
            }
            Some(Token::Let) => {
                self.next();
                if let Some(Token::Ident(id)) = self.next() {
                    if self.next() != Some(Token::Colon) {
                        return None;
                    }
                    if let Some(var_type) = self.parse_type() {
                        return match self.next() {
                            Some(Token::Semicolon) => Some(Statement::Declaration {
                                name: id,
                                var_type,
                                rhs: None,
                            }),
                            Some(Token::Eq) => {
                                if let Some(expr) = self.parse_expression()
                                    && self.next() == Some(Token::Semicolon)
                                {
                                    Some(Statement::Declaration {
                                        name: id,
                                        var_type,
                                        rhs: Some(expr),
                                    })
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        };
                    }
                }
                None
            }
            Some(Token::If) => {
                self.next();
                if let Some(cond) = self.parse_expression()
                    && self.cur_token == Some(Token::OCurly)
                {
                    if let Some(block) = self.parse_block() {
                        return Some(Statement::If {
                            condition: cond,
                            contents: block,
                        });
                    }
                }
                None
            }
            Some(Token::Return) => {
                self.next();
                if let Some(ret) = self.parse_expression() {
                    Some(Statement::Return(ret))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn next(&mut self) -> Option<Token> {
        let tmp = self.cur_token.clone();
        self.cur_token = self.next_token.clone();
        self.next_token = self.tokens.next();
        tmp
    }

    // Finishes evaluation when it meets an unexpected token,
    // returns None if the tokens up to that point do not form an expression
    fn parse_expression(&mut self) -> Option<Expression> {
        todo!()
    }

    fn parse_field(&mut self) -> Option<()> {
        todo!()
    }
}
