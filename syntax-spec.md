```
program       ->     fn_decl* EOF ;

fn_decl       ->     "fn" IDENTIFIER "(" parameters? ")" block ;

parameters    ->     IDENTIFIER ( "," IDENTIFIER )* ;

let_decl      ->     "let" IDENTIFIER "=" expression ";" ;

block         ->     "{" ( let_decl | statement )* "}" ;

statement     ->     block
                  |  if_stmt
                  |  print_stmt
                  |  return_stmt
                  |  for_stmt
                  |  while_stmt ;

for_stmt      ->     "for" "(" ( let_decl | expr_stmt | ";" ) expression? ";" expression? ")" statement ;

if_stmt       ->     "if" "(" expression ")" statement ( "else" statement )? ;

print_stmt    ->     "print" expression  ";" ;

while_stmt    ->     "while" "(" expression ")" statement ;

return_stmt   ->     "return" expression? ";" ;

expr_stmt     ->     expression ";" ;

expression    ->     assignment ;


assignment    ->     IDENTIFIER "=" assignment | logic_or ;

call          ->     IDENTIFIER ( "(" arguments? ")" )* ;

arguments     ->     expression ( "," expression )* ;

logic_or      ->     logic_and ( "or" logic_and )* ;

logic_and     ->     equality ( "and" equality )* ;

equality      ->     comparison ( ( "!=" | "==" ) comparison )* ;

comparison    ->     term ( ( ">" | ">=" | "<" | "<=" ) term )* ;

term          ->     factor ( ( "-" | "+" ) factor )* ;

factor        ->     unary ( ( "/" | "*" ) unary )* ;

unary         ->     ( "!" | "-" ) unary | primary ;

primary       ->     NUMBER_INT
                  |  NUMBER_FLOAT
                  |  STRING
                  |  "true"
                  |  "false"
                  |  call
                  |  "(" expression ")"
                  |  IDENTIFIER ;
```


### Some clarifications

- CAPITAL case words are values of the type described by the word
- lower case words are **Non-terminal Symbol**
- quoted strings are **Terminal Symbols**
- `->` indentifier before it defines the rule name, after it defines the rule's body.
- `;` marks end of a rule
- `|` is OR
- `(` and `)` for grouping
- `*` previous item can appear zero or multiple times
- `+` previous item can appear atleast once
- `?` previous item can appear zero or one time, but not more

**Terminal Symbol** : A terminal is a letter from the grammar’s alphabet. You can think of it like a literal value. In the syntactic grammar we’re defining, the terminals are individual lexemes—tokens coming from the scanner like if or 1234.

**Non-Terminal Symbol** : A nonterminal is a named reference to another rule in the grammar. It means “play that rule and insert whatever it produces here”. In this way, the grammar composes.
