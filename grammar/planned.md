np means non principal

TODO: macro invocation stuff
note: `&` as a prefix needs to play nice with `&&`

### Program grammar

```
<program>         ::= <item> <program> | <null>
<item>            ::= <function> | <struct> | <enum>
<struct>          ::= struct <ident> {<params>}
<enum>            ::= enum <ident> {<variants>}
<variants>        ::= <variant>, <variants> | <variant> | <null>
<variant>         ::= <ident> | <ident>(<types>)
<types>           ::= <type>, <types> | <type> | <null>
<function>        ::= fn <ident>(<params>) <ret-type> <block>
<params>          ::= <ident>: <type> <np-params> | <null>
<np-params>       ::= , <ident>: <type> <np-params> | <null>
<statement>       ::= <expr>; | <var-declaration>
<var-decleration> ::= let <ident><var-typing>; | let <ident><var-typing> = <expr>;
<var-typing>      ::= : <type> | <null>
<ret-type>        ::= -> <type> | <null>
<type>            ::= [<type>; <int>] | (<types>) | *<type> | <ident> | int | string | bool | char | float

<expr>            ::= return <expr> | break <expr> | <assign>
<assign>          ::= <primary> = <assign> | <primary> <compound-assign> <assign> | <lazy-or>
<lazy-or>         ::= <lazy-and> '||' <lazy-or> | <lazy-and>
<lazy-and>        ::= <cmp> && <lazy-and> | <cmp>
<cmp>             ::= <or> <cmp-op> <or> | <or>
<cmp-op>          ::= == | != | < | <= | > | >=
<or>              ::= <xor> '|' <or> | <xor>
<xor>             ::= <and> ^ <xor> | <and>
<and>             ::= <shift> & <and> | <shift>
<shift>           ::= <sum> << <shift> | <sum> >> <shift> | <sum>
<sum>             ::= <mult> + <sum> | <mult> - <sum> | <mult>
<mult>            ::= <primary> * <mult> | <primary> / <mult> | <primary> % <mult> | <primary>
<primary>         ::= <prefix><base><postfix>
<prefix>          ::= &<prefix> | *<prefix> | -<prefix> | !<prefix> | <null>
<postfix>         ::= .<ident><postfix> | (<args>)<postfix> | [<expr>]<postfix> | <null>
<base>            ::= <literal> | <ident> | <tuple> | <block> | <array> | <struct-expr> | <loop> | <for> | <while> | <if>
<tuple>           ::= (<args>)
<block>           ::= {<np-block>}
<np-block>        ::= <statement><np-block> | <expr> | <null>
<array>           ::= [<args>] | [<expr>; <expr>]
<struct-expr>     ::= <ident> {<struct-args>}
<struct-args>     ::= <struct-arg>, <struct-args> | <struct-arg> | <null>
<struct-arg>      ::= <ident>: <expr> | <ident>
<loop>            ::= loop <block>
<for>             ::= for <ident> in <expr> <block>
<while>           ::= while <expr> <block>
<if>              ::= if <expr> <block> <else>
<else>            ::= else <if> | else <block>
<literal>         ::= <string> | <int> | <bool> | <char> | <float>
<ident>           ::= <word> & !<keyword>
<keyword>         ::= break | continue | else | enum | false | fn | for | if | in | let | loop | match | return | struct | true | while
```

## Regex for complex tokens

```
<word>   ::= [a-zA-Z_][a-zA-Z_0-9]*
<string> ::= "(\\[0nt"'\\]|[^"\\])*"
<int>    ::= (0b[0-1]+)|(0x[0-9a-f]+)|(0o[0-7]+)|([0-9]+)
<bool>   ::= true|false
<char>   ::= '(\\[0nt"'\\]|[^"])'
<float>  ::= [0-9]+\.[0-9]+
```
