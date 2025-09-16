use crate::lexer::{Token, Tokens};
use std::rc::Rc;

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
}

struct Expression {}

enum AssignLhs {
    Declaration(String, Type), // okay yeah types should NOT be strings
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

fn parse(mut tokens: Tokens) {
    loop {
        match tokens.next() {
            Some(Token::Fn) => {
                if let Some(Token::Ident { name: id }) = tokens.next() {
                    if Some(Token::OParen) != tokens.next() {
                        panic!("Function defined without params");
                    }
                    let params: Vec<Variable> = Vec::new();
                    loop {
                        let next = tokens.next();
                        if next == Some(Token::CParen) {
                            break;
                        }
                        if let Some(Token::Ident { name: param_id }) = next {
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

fn parse_variable(tokens: &mut Tokens) {}
