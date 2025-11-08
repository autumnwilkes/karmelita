program -\> \<function> <program> | <null>
function -\> \<name>(<params>) -> <type> {<block>}
params -\> \<null> | <name>: <type><np_params>
np_params -\> , \<name>: <type><np_params> | <null>
block -\> \<statement> <block> | <null>
statement -\> \<expression>; | <variable_declaration> | <variable_assign> | <if_statement> | <return_statement>
variable_declaration -\> let \<name>: <type>= <expression>; | let <name>: <type>; // This is not deterministic
variable_assign -\> = \<expression>;
lhs_assign -\> let \<name>: <type> | <name>
return_statement -\> return \<expression>;
expression -\> \<function_call> | <arithmatic_expression> | <logic_expression> | <literal> | <name>
function_call -\> \<name>(<tuple>)
tuple -\> \<null> | <nn_tuple>
nn_tuple -\> \<expression>, <nn_tuple> | <expression>
if_statement -\> if (\<expression>) {<block>} <elif_statement>
elif_statement -\> else if (\<expression>) {<block>} <elif_statement> | else {<block>} | <null>
arithmatic_expression -\> TODO
logic_expression -\> TODO
statement -\> \<name>

np means non principal
currently, this grammar encounters issues when coming across a line starting with \<name\>, where it cannot tell if this is a variable assign statement or an expression statement

ret -\>
| return \<ret\>
| break \<ret\>
| \<assign\>

assign -\>
| \<range\> = <assign>
| \<range\> <compound_assign> <assign>
| \<range\>

compound_assign -\>
| +=
| -=
| \*=
| /=
| %=
| \<<=
| \>>=
| &=
| |=
| ^=

range -\>
| \<?\>..<?>
| ..\<?\>
| \<?\>..
| ..
| ..=\<?\>
| \<?\>..=<?>
| \<lazy_or\>

lazy_or -\>
| \<cmp\> || <lazy_or>
| \<lazy_and\>

lazy_and -\>
| \<cmp\> && <lazy_and>
| \<cmp\>

cmp -\>
| \<?\> == <cmp>
| \<?\> != <cmp>
| \<?\> < <cmp>
| \<?\> <= <cmp>
| \<?\> > <cmp>
| \<?\> >= <cmp>

bitwise_or -\>
| \<bitwise_xor\> | <bitwise_or>
| \<bitwise_xor\>

bitwise_xor -\>
| \<bitwise_and\> & <bitwise_xor>
| \<bitwise_and\>

bitwise_and -\>
| \<bit_shift\> & <bitwise_and>
| \<bit_shift\>

bit_shift -\>
| \<term\> << <bit_shift>
| \<term\> >> <bit_shift>
| \<term\>

term -\>
| \<factor\> + <term>
| \<factor\> - <term>
| \<factor\>

factor -\>
| \<type_cast\> \* <factor>
| \<type_cast\> / <factor>
| \<type_cast\> % <factor>
| \<type_cast\>

type_cast -\>
| : // IDK WHAT'S GOING ON HERE LOL
| \<unary\> as <type_cast> // TODO: bad
| \<unary\>

unary -\>
| -\<unary\>
| !\<unary\>
| \*\<unary\>
| &\<unary\>
| &mut \<unary\>
| \<function_call\>

field_access -\>
| \<field_access\><call_or_index>.<field_access>
| \<field_access\>

call_or_index -\>
| []\<call_or_index\>
| ()\<call_or_index\>
| \<null\>

field_access -\>
| \<method_call\>.<field_access>
| \<method_call\>

method_call -\>
| \<method_call\>.<method_call>() // evil!!!
| \<path\>
| \<path\>

field_access -\>
|

\<some-thing\> ::=<br>
| \<literal\>
|

\<literal\> ::=<br>
| \<string\><br>
| \<integer\><br>
| \<boolean\><br>
| \<character\><br>
| \<float\><br>
<br>

\<keyword\>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;::=<br>
| "as"
| "break"
| "const"
| "continue"
| "crate"
| "else"
| "enum"
| "extern"
| "false"
| "fn"
| "for"
| "if"
| "impl"
| "in"
| "let"
| "loop"
| "match"
| "mod"
| "move"
| "mut"
| "pub"
| "ref"
| "return"
| "self"
| "Self"
| "static"
| "struct"
| "super"
| "trait"
| "true"
| "type"
| "unsafe"
| "use"
| "where"
| "while"
| "async"
| "await"
| "dyn"

### Regex for terminals

\<ident-or-keyword\>&nbsp;::= \[a-zA-Z\_\]\[a-zA-Z0-9\_\]\*<br>
\<string\>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;::= "(\\\\\[0nt"'\\\\\]|\[^"\\\\\])\*"<br>
\<integer\>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;::= (0b\[0-1\]\+)|(0x\[0-9a-f\]+)|(0o\[0-7\]+)|(\[0-9\]+)<br>
\<boolean\>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;::= true|false<br>
\<character\>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;::= '(\\\[0nt"'\\\]|\[^"\])'<br>
\<float\>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;::= \[0-9\]+\\.\[0-9\]+<br>
