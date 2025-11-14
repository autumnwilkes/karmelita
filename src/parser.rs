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

struct BinExpr {
    lhs: Box<Expression>,
    operation: BinOp,
    rhs: Box<Expression>,
}

enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    Or,
    And,
    Xor,
    ShiftLeft,
    ShiftRight,
}

// Maybe create stricter rules on how expressions can be created?
enum Expression {
    Binary(BinOp, Box<Expression>, Box<Expression>),
    Postfix(),
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
    RefMut(Box<Expression>),
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
        name: String, // could be path?
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
    next: Option<Token>,
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
            if self.next == Some(Token::CParen) {
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
                while self.next != Some(Token::CParen) {
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
        while self.next != Some(Token::CCurly) {
            if self.next == None {
                return None;
            }
            let statement = todo!(); // self.parse_statement();
            if let Some(s) = statement {
                block.push(s);
            }
        }
        Some(block)
    }

    fn prefix_expr(&mut self) -> Option<Expression> {
        match self.next {
            Some(Token::BAndOp) => match self.next_skip() {
                Some(Token::Mut) => {
                    self.consume();
                    Some(Expression::RefMut(Box::new(self.prefix_expr()?)))
                }
                _ => Some(Expression::Ref(Box::new(self.prefix_expr()?))),
            },
            Some(Token::AndOp) => match self.next_skip() {
                Some(Token::Mut) => {
                    self.consume();
                    Some(Expression::Ref(Box::new(Expression::RefMut(Box::new(
                        self.prefix_expr()?,
                    )))))
                }
                _ => Some(Expression::Ref(Box::new(Expression::Ref(Box::new(
                    self.prefix_expr()?,
                ))))),
            },
            Some(Token::Star) => {
                self.consume();
                Some(Expression::Deref(Box::new(self.prefix_expr()?)))
            }
            Some(Token::MinusOp) => {
                self.consume();
                Some(Expression::Negation(Box::new(self.prefix_expr()?)))
            }
            Some(Token::NotOp) => {
                self.consume();
                Some(Expression::Not(Box::new(self.prefix_expr()?)))
            }
            _ => self.base_expr(),
        }
    }

    fn postfix_to_expr(&mut self, e: Expression) -> Option<Expression> {
        todo!()
    }
    fn expression(&mut self) -> Option<Expression> {
        todo!()
    }
    fn base_expr(&mut self) -> Option<Expression> {
        match self.next() {
            Some(Token::IntLiteral(lit)) => Some(Expression::IntLiteral(lit)),
            Some(Token::CharLiteral(lit)) => Some(Expression::CharLiteral(lit)),
            Some(Token::StringLiteral(lit)) => Some(Expression::StrLiteral(lit)),
            Some(Token::BoolLiteral(lit)) => Some(Expression::BoolLiteral(lit)),
            Some(Token::Ident(id)) => {
                //todo: path stuff
                //also struct stuff
                if self.next != Some(Token::OBracket) {
                    return Some(Expression::Ident(id));
                }
                self.consume();
                let args: Vec<StructPart> = vec![];
                while self.next != Some(Token::CBracket) {
                    let Some(Token::Ident(field)) = self.next() else {
                        return None;
                    };
                    if self.next() != Some(Token::Colon) {
                        return None;
                    }
                    let Some(expr) = self.expression() else {
                        return None;
                    };
                    match self.next {
                        Some(Token::Comma) => self.consume(),
                        Some(Token::CBracket) => (),
                        _ => return None,
                    }
                    args.push(StructPart {
                        name: field,
                        value: expr,
                    })
                }
                Some(Expression::StructDeclaration {
                    name: id,
                    fields: args,
                })
            }
            Some(Token::OParen) => {
                let args = self.parse_args();
                if self.next() != Some(Token::CParen) {
                    return None;
                }
                Some(Expression::Tuple(args))
            }
            Some(Token::OCurly) => {
                let block = self.parse_block();
                if self.next() != Some(Token::CCurly) {
                    return None;
                }
                Some(Expression::Block(block))
            }
            Some(Token::OBracket) => {
                todo!()
            }
            Some(Token::Loop) => Some(Expression::Loop(Box::new(self.expression()?))),
            Some(Token::For) => todo!(),
            Some(Token::While) => Some(Expression::While(self.expression(), self.expression())),
            _ => todo!(),
        }
    }

    fn postfix_to_expression(&mut self, expr: Expression) -> Option<Expression> {
        todo!()
    }
    /*
        fn parse_statement(&mut self) -> Option<Statement> {
            match self.next {
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
                        && self.next == Some(Token::OCurly)
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
    */

    fn consume(&mut self) {
        self.next = self.tokens.next();
    }

    // I want to be able to call next(), then call a local variable to get the NEXT next token
    // I also want to be able to call next(), then go back on this decision?
    fn next(&mut self) -> Option<Token> {
        let tmp = self.next.clone();
        self.next = self.tokens.next();
        tmp
    }

    fn next_skip(&mut self) -> Option<Token> {
        let tmp = self.tokens.next();
        self.next = self.tokens.next();
        tmp
    }
}
