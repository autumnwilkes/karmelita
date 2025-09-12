## Organizational plans

Lexer produces a `TokenStream`, which implements iter and peekable, and which gives the next token when called.
Tokens are basic elements of syntax, such as identifiers, symbols, and keywords.

Should the lexer track certain sets of symbols (e.g. ++), or should these be fully dealt with by the parser?

How should ambiguity around certain tokens (e.g. identifiers at the start of statements as parts of an expression statement or an assignment?) be dealt with by the parser? Does it need to have some sort of "uncertain" state, where it needs to look ahead to figure out if the identifier is part of an assignment or an expression?

foo (state = `statement`) -> state = `identifier`
= (state = `identifier`) -> state = `rhs_assign`
. (state = `identifier`) -> state = `method`

- (state = `identifier`) -> state = `add(identifier, _)`
