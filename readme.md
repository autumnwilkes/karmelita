## Notes

Currently not working on any sort of error handling; instead, the program will simply panic whenever anything goes wrong </3

## Organizational plans

Lexer produces a `TokenStream`, which implements `Iterator`, and which gives the next token when called.
Tokens are basic elements of syntax, such as identifiers, symbols, and keywords. Whitespace is not a token, comments are not a token.

Should the lexer track certain sets of symbols (e.g. ++), or should these be fully dealt with by the parser?
It should probably count separate symbol patterns together, as it sounds really annoying to do that stuff in the parser

How should ambiguity around certain tokens (e.g. identifiers at the start of statements as parts of an expression statement or an assignment?) be dealt with by the parser? Does it need to have some sort of "uncertain" state, where it needs to look ahead to figure out if the identifier is part of an assignment or an expression?

- foo (state = `statement`) -> state = `identifier`
- = (state = `identifier`) -> state = `rhs_assign`
- . (state = `identifier`) -> state = `method`

- - (state = `identifier`) -> state = `add(identifier, _)`

## Abstract Syntax Tree

Function := {
name: String,
params: Vec<(String, Type)>
return: Type
code: Vec\<Statement\>
}

Type :=
defined(String) |
ptr(Type) |
PrimitiveType |
Tuple(Vec\<Type\>) |
Array(Type, usize)

PrimitiveType :=
integer |
bool |
string |
char |
float

Statement :=
Expression(Expression) |
Return(Expression) |
If(Expression, Vec\<Statement\>, Option\<ElseStatement\>) |
Assignment(Lhs, Expression) |
Declaration(Variable)

ElseStatement :=
Elif(Expression, Vec\<Statement\>, Option\<ElseStatement\>) |
Else(Vec\Statement\>)

Lhs :=
Declaration(Variable) |
Assign(VarOrField)

VarOrField :=
Var(Variable) |
Index(Variable, usize) |
Field(Variable, Variable) | // This seems wrong? but idk what would be better
TupleIdx(Variable, usize) // Does this need to be different from Index?
