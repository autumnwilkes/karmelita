<program> -> <function>_
<function> -> <name>(<param>_) -> <type> {<statement>\*}

<param> -> <name>: <type>
<name> -> regex([a-z][a-z0-9_]*)
<statement> -> <expression>; | <variable_declaration> | <variable_assign> | <if_statement> | <return_statement>
<variable_declaration> -> let <name>: <type> [= <expression>]?;
<variable_assign> -> <name> = <expression>;
<return_statement> -> return <expression>;
<expression> -> <function_call> | <arithmatic_expression> | <logic_expression> | <literal> | <name>
<function_call> -> <name>(<name>*)
<if_statement> -> if (<expression>) {<statement>*} [else if (<expression>) {<statement>*}]* [else {<statement>*}]?
