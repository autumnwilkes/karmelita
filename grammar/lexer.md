Dot ::= (?<!.).(?!.)<br>
Comma ::= ,<br>
Question ::= ?<br>
Colon ::= :<br>
Semicolon ::= ;<br>
Star ::= \*<br>
Hashtag ::= #<br>
At ::= @<br>
Pipe ::= |<br>
Backslash ::= \\<br>
SmArrow ::= -><br>
LgArrow ::= =><br>
NotOp ::= !<br>
PlusOneOp ::= ++<br>
MinusOneOp ::= --<br>
BAndOp ::= &<br>
ShLeftOp ::= <<<br>
ShRightOp ::= >><br>
AndOp ::= &&<br>
OrOp ::= ||<br>
XorOp ::= ^<br>
ModOp ::= %<br>
PlusOp ::= +<br>
MinusOp ::= -<br>
DivOp,

EqCmp,
NeqCmp,
LtCmp,
LeCmp,
GtCmp,
GeCmp,

Eq, //\*
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

BoolLiteral(bool),
IntLiteral(usize),
CharLiteral(char),
StringLiteral(String),
Ident(String),

// In addition to being used for explicit dereferencing operations with the
/// (unary) `*` operator in immutable contexts, `Deref` is also used implicitly
