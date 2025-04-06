```
program       ::=     fn_decl* EOF ;

fn_decl       ::=     "fn" IDENTIFIER "(" parameters? ")" block ;

parameters    ::=     IDENTIFIER ;

statement     ::=     block
                    | let_stmt
                    | if_stmt
                    | print_stmt
                    | return_stmt;

block         ::=     "{" statement* "}" ;

let_stmt      ::=     "let" IDENTIFIER "=" expression ";" ;

if_stmt       ::=     "if" "(" expression ")" statement ( "else" statement )? ;

print_stmt    ::=     "print" expression  ";" ;

return_stmt   ::=     "return" expression? ";" ;

expression    ::=     assignment ;

assignment    ::=     IDENTIFIER "=" assignment | logic_or ;

logic_or      ::=     logic_and ( "or" logic_and )* ;

logic_and     ::=     equality ( "and" equality )* ;

equality      ::=     comparison ( ( "!=" | "==" ) comparison )* ;

comparison    ::=     term ( ( ">" | ">=" | "<" | "<=" ) term )* ;

term          ::=     factor ( ( "-" | "+" ) factor )* ;

factor        ::=     unary ( ( "/" | "*" ) unary )* ;

unary         ::=     ( "!" | "-" ) unary | call | native_call;

call          ::=     primary ( "(" expression? ")" )* ;

native_call   ::=     "extern" primary( "(" expression* ")" )* ;

primary       ::=     NUMBER_INT
                    | NUMBER_FLOAT
                    | STRING
                    | "true"
                    | "false"
                    | "(" expression ")"
                    | IDENTIFIER ;
```


### Some clarifications

- CAPITAL case words are values of the type described by the word
- lower case words are **Non-terminal Symbol**
- quoted strings are **Terminal Symbols**
- `::=` indentifier before it defines the rule name, after it defines the rule's body.
- `;` marks end of a rule
- `|` is OR
- `(` and `)` for grouping
- `*` previous item can appear zero or multiple times
- `+` previous item can appear atleast once
- `?` previous item can appear zero or one time, but not more

**Terminal Symbol** : A terminal is a letter from the grammar’s alphabet. You can think of it like a literal value. In the syntactic grammar we’re defining, the terminals are individual lexemes—tokens coming from the scanner like if or 1234.

**Non-Terminal Symbol** : A nonterminal is a named reference to another rule in the grammar. It means “play that rule and insert whatever it produces here”. In this way, the grammar composes.
