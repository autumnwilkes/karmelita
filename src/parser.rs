use crate::lexer::{Token, Tokens};

struct Function {
    id: String,
    params: Vec<Variable>,
    return_type: Type,
    contents: Block,
}

// Variable vs function call vs property (aka both of those?)
struct Variable {
    ident: String,
    var_type: Type,
}

struct Field {
    id: String,
}

enum Pattern {} // The most terrifying enum ever lol

type Block = Vec<Statement>;

enum Statement {
    If {
        condition: Expression,
        contents: Block,
    },
    Assignment {
        lhs: Expression,
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

enum Type {
    UserDefined(String),
    Ptr(Box<Type>),
    Tuple(Vec<Type>),
    Array(Box<Type>, usize),
    // and primitives lol
}
struct Parser<'a> {
    tokens: Tokens<'a>,

    cur_token: Option<Token>,
    next_token: Option<Token>,
}

#[allow(unused)]
impl Parser<'_> {
    fn parse(&mut self) {
        loop {
            match self.next() {
                Some(Token::Fn) => {}
                _ => {}
            }
        }
    }

    fn parse_function(&mut self) -> Option<Function> {
        let Some(Token::Ident(id)) = self.next() else {
            return None;
        };
        if self.next() != Some(Token::OParen) {
            return None;
        }
        let mut params: Vec<Variable> = Vec::new();
        loop {
            // TODO: turn into while (?)
            if self.cur_token == Some(Token::CParen) {
                break;
            }
            let Some(Token::Ident(param_id)) = self.next() else {
                return None;
            };
            if self.next() != Some(Token::Colon) {
                return None;
            }; // if let Some(Token::Ident {})
            let Some(var_type) = self.parse_type() else {
                return None;
            };
            let var = Variable {
                ident: param_id,
                var_type,
            };
            params.push(var);
            match self.next() {
                Some(Token::CParen) => break,
                Some(Token::Comma) => continue,
                _ => return None,
            }
        }
        if self.next() != Some(Token::SmArrow) {
            return None;
        }
        let Some(return_type) = self.parse_type() else {
            return None;
        };
        let Some(block) = self.parse_block() else {
            return None;
        };
        Some(Function {
            id,
            params,
            return_type,
            contents: block,
        })
    }

    fn parse_type(&mut self) -> Option<Type> {
        match self.next() {
            Some(Token::Ident(id)) => Some(Type::UserDefined(id)),
            Some(Token::Star) => Some(Type::Ptr(Box::new(self.parse_type()?))),
            Some(Token::OParen) => {
                let mut tuple: Vec<Type> = Vec::new();
                while self.cur_token != Some(Token::CParen) {
                    tuple.push(self.parse_type()?);
                }
                self.next();
                Some(Type::Tuple(tuple))
            }
            Some(Token::OBracket) => {
                let arr_type = self.parse_type()?;
                if self.next() != Some(Token::Semicolon) {
                    return None;
                };
                let Some(Token::IntLiteral(len)) = self.next() else {
                    // doesn't handle negatives correctly
                    return None;
                };
                Some(Type::Array(Box::new(self.parse_type()?), len))
            }
            _ => None,
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
                let Some(expr) = self.parse_expression() else {
                    return None;
                };
                match self.next() {
                    Some(Token::Eq) => {
                        let Some(rhs) = self.parse_expression() else {
                            return None;
                        };
                        if self.next() != Some(Token::Semicolon) {
                            return None;
                        }
                        Some(Statement::Assignment { lhs: expr, rhs })
                    }
                    Some(Token::Semicolon) => Some(Statement::Expression(expr)),
                    _ => None,
                }
            }
            Some(Token::Let) => {
                self.next();
                let Some(Token::Ident(id)) = self.next() else {
                    return None;
                };
                if self.next() != Some(Token::Colon) {
                    return None;
                }
                let Some(var_type) = self.parse_type() else {
                    return None;
                };
                return match self.next() {
                    Some(Token::Semicolon) => Some(Statement::Declaration {
                        name: id,
                        var_type,
                        rhs: None,
                    }),
                    Some(Token::Eq) => {
                        let Some(expr) = self.parse_expression() else {
                            return None;
                        };
                        if self.next() != Some(Token::Semicolon) {
                            return None;
                        }

                        Some(Statement::Declaration {
                            name: id,
                            var_type,
                            rhs: Some(expr),
                        })
                    }
                    _ => None,
                };
            }
            Some(Token::If) => {
                self.next();
                if let Some(cond) = self.parse_expression() // TODO: flatten
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
            _ => {
                if let Some(expr) = self.parse_expression() {
                    if self.next() == Some(Token::Semicolon) {
                        return Some(Statement::Expression(expr));
                    }
                }
                None
            }
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
    // prob not a real function lol
    fn parse_property(&mut self) -> Option<()> {
        todo!()
    }
}
