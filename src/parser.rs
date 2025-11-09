use std::ops::MulAssign;

use crate::lexer::{Token, Tokens};

enum Item {
    Function(Function),
    Struct(Struct),
    Module(String, Vec<Item>),
    Use, // Need to think a bit more abt linking
    Enum(Enum),
    Trait(String, Vec<Function>),
    Impl(String, Vec<Function>),
    ImplTrait {
        trait_name: String,
        item_name: String,
        implementation: Vec<Function>,
    },
}

struct Enum {
    id: String,
    variants: Vec<EnumVariant>,
}

struct EnumVariant {
    name: String,
    kind: EnumVariantType,
}
enum EnumVariantType {
    Thin,
    Struct { fields: Vec<Variable> },
    Tuple { elements: Vec<Type> },
}

struct Struct {
    id: String,
    fields: Vec<Variable>,
}
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

enum IdentExprRhs {}

type Block = Vec<Statement>;

enum Statement {
    // Need to include all equal statements (pluseq...)
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
// Maybe create stricter rules on how expressions can be created?
enum Expression {
    Add {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Sub {
        minuend: Box<Expression>,
        subtrahend: Box<Expression>,
    },
    Mult {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Div {
        dividend: Box<Expression>,
        divisor: Box<Expression>,
    },
    Modulo {
        value: Box<Expression>,
        modulus: Box<Expression>,
    },
    Or {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    And {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Xor {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    BShiftLeft {
        value: Box<Expression>,
        shift: Box<Expression>,
    },
    BShiftRight {
        value: Box<Expression>,
        shift: Box<Expression>,
    },

    FnCall {
        function: Box<Expression>,
        params: Vec<Expression>,
    },

    Field {
        parent: Box<Expression>,
        child: Box<Expression>, // could be path or ident
    }, // I am pretending that methods are also fields?

    // indexing an array
    Index {
        array: Box<Expression>,
        index: Box<Expression>,
    },

    // Does nothing (parenthesis)
    Null(Box<Expression>),

    // inverts value
    Not(Box<Expression>),
    Negation(Box<Expression>),

    Ident(String),

    Ref(Box<Expression>),
    Deref(Box<Expression>),

    // ErrorPropogation(Box<Expression>),

    // Pattern stuff?
    Equals {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LessThan {
        less: Box<Expression>,
        greater: Box<Expression>,
    },
    GreaterThan {
        greater: Box<Expression>,
        less: Box<Expression>,
    },

    LazyOr {
        first: Box<Expression>,
        second: Box<Expression>,
    },
    LazyAnd {
        first: Box<Expression>,
        second: Box<Expression>,
    },

    IntLiteral(usize),
    // FloatLiteral(f64),
    StrLiteral(String),
    BoolLiteral(bool),
    CharLiteral(char),
    // Path
    // TypeCast {expression: Box<Expression>, new_type: Type},
    // Assignment {lhs: Box<Expression>, rhs: Box<Expression>}
    Block {
        block: Vec<Statement>,
        ret: Option<Box<Expression>>,
    },
    Array(Vec<Expression>),
    ArrayWithCopy {
        value: Box<Expression>,
        count: Box<Expression>,
    },
    ArrayIndex {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    Tuple(Vec<Expression>),
    TupleIndex {
        tuple: Box<Expression>,
        index: usize,
    },
    StructDeclaration {
        name: Box<Expression>, // could be path?
        fields: Vec<StructPart>,
    },

    // Closure {},
    Loop(Box<Expression>), // block
    Break(Box<Expression>),
    Continue,
    If {
        predicate: Box<Expression>,
        block: Box<Expression>,
    }, // TODO: Add if let syntax
    Match {
        scrutinee: Box<Expression>,
        arms: Vec<MatchArm>,
    },
    Return(Box<Expression>),
    Underscore,
}

struct MatchArm {
    pattern: Pattern,
    block: Expression,
}

struct StructPart {
    name: String,
    value: Expression,
}

enum Type {
    UserDefined(String),
    Ptr(Box<Type>),
    Tuple(Vec<Type>),
    Array(Box<Type>, usize),
    Integer,
    String,
    Char,
    Bool,
    // and primitives lol
}
struct Parser<'a> {
    tokens: Tokens<'a>,
    cur_token: Option<Token>,
    next_token: Option<Token>,
}

enum Pattern {}

#[allow(unused)]
impl Parser<'_> {
    fn parse(&mut self) {
        let mut items: Vec<Item> = vec![];
        loop {
            match self.next() {
                Some(Token::Fn) => items.push(Item::Function(self.parse_function().unwrap())),
                Some(Token::Struct) => {}
                Some(Token::Impl) => {}
                Some(Token::Trait) => {}
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
            };
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

    fn parse_expression(&mut self) -> Option<Expression> {
        todo!()
    }

    fn parse_ret(&mut self) -> Option<Expression> {
        let x;
        x = return None;
    }

    fn parse_expression_0(&mut self) -> Option<Expression> {
        match &self.next_token {
            None => None,
            Some(Token::OParen) => {
                let Some(expr) = self.parse_expression() else {
                    return None;
                };

                if self.next() == Some(Token::CParen) {
                    return Some(expr);
                }
                None
            }
            Some(Token::Ident(name)) => {
                let rhs = self.parse_expression_ident();
                match rhs {
                    Some(thing) => todo!(),
                    None => todo!(),
                }
            }
            _ => todo!(),
        }
    }

    fn parse_expression_ident(&mut self) -> Option<IdentExprRhs> {
        let Some(ident) = self.next() else {
            return None;
        };
        match self.cur_token {
            None => None,
            Some(Token::Dot) => {
                self.next();
                match self.cur_token {
                    Some(Token::IntLiteral(int)) => todo!(),
                    _ => todo!(),
                }
            }
            _ => todo!(), // Some(Expression::Ident(ident))
        }
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
}
