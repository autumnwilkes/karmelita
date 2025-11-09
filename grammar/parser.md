np means non principal

TODO: macro invocation stuff
note: `&` as a prefix needs to play nice with `&&`

### Program grammar

```
<program> ::= <item> <program> | <null>
<item> ::=
  <function>
| <struct>
| <enum>
| <mod>
| <use>
| <const>
<function> ::= <ident>(<params>) <ret-type> <block>
<params> ::= <ident>: <type> <np-params> | <null>
<np-params> ::= , <ident>: <type> <np-params> | <null>
<statement> ::= <expr>; | <var-declaration>
<var-decleration> ::= let <ident><var-typing>; | let <ident><var-typing> = <expr>;
<var-typing> ::= : <type> | <null>
<ret-type> ::= -> <type> | <null>
```

### Expression grammar

```
<expr>              ::=
  return <expr>
| break <expr>
| <assign>

<assign> ::=
  <primary> = <assign>
| <primary> <compound-assign> <assign>
| <range>

<range> ::=
  <or><range-op><or>
| <range-op><or>
| <or><range-op>
| <lazy-or>

<range-op> ::= .. | ..=

<lazy-or> ::= <lazy-and> '||' <lazy-or> | <lazy-and>

<lazy-and> ::= <cmp> && <lazy-and> | <cmp>

<cmp> ::= <or> <cmp-op> <or> | <or>

<cmp-op> ::=
  ==
| !=
| <
| <=
| >
| >=

<or> ::= <xor> '|' <or> | <xor>

<xor> ::= <and> ^ <xor> | <and>

<and> ::= <shift> & <and> | <shift>

<shift> ::= <sum> << <shift> | <sum> >> <shift> | <sum>

<sum>   ::= <mult> + <sum> | <mult> - <sum> | <mult>

<mult>   ::=
  <cast> * <mult>
| <cast> / <mult>
| <cast> % <mult>
| <cast>

<cast> ::= <primary> as <type> | <primary>

<primary> ::= <prefix><base><postfix>

<prefix> ::=
  &<prefix>
| &mut<prefix>
| &raw const<prefix>
| &raw mut<prefix>
| *<prefix>
| -<prefix>
| !<prefix>
| <null>

<postfix>   ::=
  .<field><postfix>
| (<args>)<postfix>
| [<expr>]<postfix>
| ?<postfix>
| <null>

<field>   ::=
  await
| <path>()
| <ident>

<base>              ::=
  <literal>
| <ident>
| <tuple>
| <path>
| <block>
| <array>
| <struct-expr>
| <closure>
| <loop>
| <for>
| <while>
| <if>
| <match>
| _

<tuple> ::= (<args>)
<path> ::= <ident>::<path> | <ident>::<generic-params> | <ident>
<generic-params> ::= <not-implemented>
<block> ::= {<np-block>}
<np-block> ::= <statement><np-block> | <expr> | <null>
<array> ::= [<args>] | [<expr>; <expr>]
<struct-expr> ::= <ident> {<struct-args>}
<struct-args> ::= <struct-arg>, <struct-args> | <struct-arg> | <null>
<struct-arg> ::= <ident>: <expr> | <ident>
<closure> ::= <not-implemented>
<loop> ::= loop <block>
<for> ::= for <ident> in <expr> <block>
<while> ::= while <expr> <block>
<if> ::= if <expr> <block> <else>
<else> ::= else <if> | else <block>
<match> ::= <not-implemented>

<literal>           ::=
  <string>
| <int>
| <bool>
| <char>
| <float>

<ident>           ::= <word> & !<strict> & !<reserved>

<keyword>         ::=
  <strict>
| <reserved>
| <weak>

<strict-keyword>  ::=
  as
| break
| const
| continue
| crate
| else
| enum
| extern
| false
| fn
| for
| if
| impl
| in
| let
| loop
| match
| mod
| move
| mut
| pub
| ref
| return
| self
| Self
| static
| struct
| super
| trait
| true
| type
| unsafe
| use
| where
| while
| async
| await
| dyn

<reserved> ::=
  abstract
| become
| box
| do
| final
| macro
| override
| priv
| typeof
| unsized
| virtual
| yield
| try
| gen

<weak>     ::=
  static
| macro_rules
| raw
| safe
| union
```

## Regex for complex tokens

```bfn
<word>      ::= [a-zA-Z_][a-zA-Z_0-9]*
<string>    ::= "(\\[0nt"'\\]|[^"\\])*"
<int>   ::= (0b[0-1]+)|(0x[0-9a-f]+)|(0o[0-7]+)|([0-9]+)
<bool>   ::= true|false
<char> ::= '(\\[0nt"'\\]|[^"])'
<float>     ::= [0-9]+\.[0-9]+
```
