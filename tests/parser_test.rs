use evelin::ast::{
    BinOp, DType, Expr, FnDecl, FnStDeclField, LiteralExpr, LiteralValue, Stmt, StructDecl, Token,
    TokenType,
};
use evelin::lexer::Lexer;
use evelin::parser::Parser;

fn tokenize<T: Into<String>>(input: T) -> Vec<evelin::lexer::Token> {
    let l = input.into();
    let mut lexer = Lexer::from(&l);
    lexer.start().unwrap();
    let t = lexer.tokens();
    (*t).clone()
}

fn parse_fn(source: &str) -> Vec<FnDecl> {
    let tokens = tokenize(source.to_string());
    let mut parser = Parser::from(&tokens);
    parser.parse();
    parser.fn_decls
}

fn parser_struct(source: &str) -> Vec<StructDecl> {
    let tokens = tokenize(source.to_string());
    let mut parser = Parser::from(&tokens);
    parser.parse();
    parser.struct_decls
}

#[test]
fn parses_empty_struct() {
    let parser = parser_struct("struct Foo {}");

    assert_eq!(parser.len(), 1);
    let s = &parser[0];
    assert_eq!(s.name, "Foo");
    assert!(s.fields.is_empty());
}

#[test]
fn parses_struct_with_fields() {
    let parser = parser_struct("struct Point { x: int, y: float }");

    assert_eq!(parser.len(), 1);
    let s = &parser[0];
    assert_eq!(s.name, "Point");
    assert_eq!(
        s.fields,
        vec![
            FnStDeclField {
                field_name: "x".to_string(),
                field_type: DType::Primitive(Token {
                    lexeme: "int".to_string(),
                    line: 1,
                    ttype: TokenType::TypeInt,
                    literal: LiteralValue::Null
                }),
            },
            FnStDeclField {
                field_name: "y".to_string(),
                field_type: DType::Primitive(Token {
                    lexeme: "float".to_string(),
                    line: 1,
                    ttype: TokenType::TypeFloat,
                    literal: LiteralValue::Null
                }),
            },
        ]
    );
}

#[test]
fn parses_let_stmt() {
    let parser = parse_fn("fn test() -> int { let a = 123; }");

    if let Stmt::Let(let_stmt) = &parser[0].body[0] {
        match &let_stmt.initialiser {
            Expr::Literal(lit) => {
                matches!(lit.value, LiteralValue::NumberInt(123));
            }
            _ => panic!("Expected literal int"),
        }

        assert_eq!("a".to_string(), let_stmt.name);
    } else {
        panic!("Expected let stmt");
    }
}

#[test]
fn parses_struct_init_stmt() {
    let parser = parse_fn("fn test() -> int { let a = Point { x: 2, y: 3 }; }");
    if let Stmt::StructInit(struct_init) = &parser[0].body[0] {
        assert_eq!("Point".to_string(), struct_init.struct_name);
        assert_eq!("a".to_string(), struct_init.name);

        let first = &struct_init.arguments[0];
        assert_eq!(first.field_name, "x".to_owned());
        matches!(
            first.field_expr,
            Expr::Literal(LiteralExpr {
                value: LiteralValue::NumberInt(2),
            }),
        );

        let second = &struct_init.arguments[1];
        assert_eq!(second.field_name, "y".to_owned());
        matches!(
            second.field_expr,
            Expr::Literal(LiteralExpr {
                value: LiteralValue::NumberInt(3),
            }),
        );
    } else {
        panic!("Expected struct init stmt");
    }
}

#[test]
fn parses_function_without_param() {
    let parser = parse_fn("fn main() -> void { return 42; }");

    assert_eq!(parser.len(), 1);
    let f = &parser[0];
    assert_eq!(f.name, "main");
    assert!(f.parameter.is_none());
    assert_eq!(f.return_type.ttype, TokenType::TypeVoid);
    assert!(!f.body.is_empty());
}

#[test]
fn parses_function_with_param() {
    let parser = parse_fn("fn inc(x: int) -> int { return x; }");

    assert_eq!(parser.len(), 1);
    let f = &parser[0];
    assert_eq!(f.name, "inc");
    assert_eq!(
        f.parameter,
        Some(FnStDeclField {
            field_name: "x".to_string(),
            field_type: DType::Primitive(Token {
                lexeme: "int".to_string(),
                line: 1,
                ttype: TokenType::TypeInt,
                literal: LiteralValue::Null
            })
        })
    );
    assert_eq!(f.return_type.ttype, TokenType::TypeInt);
}

#[test]
fn parses_if_else_statement() {
    let parser = parse_fn("fn test() -> int { if (true) { print 1; } else { print 2; } }");

    let body = &parser[0].body;
    assert!(matches!(body[0], Stmt::If(_)));
}

#[test]
fn parses_literal_expression() {
    let parser = parse_fn("fn test() -> int { return 123; }");

    if let Stmt::Return(ret_stmt) = &parser[0].body[0] {
        match &ret_stmt.value {
            Some(val) => match val {
                Expr::Literal(lit) => {
                    matches!(lit.value, LiteralValue::NumberInt(123));
                }
                _ => panic!("Expected literal value"),
            },

            None => panic!("None value for return stmt"),
        }
    } else {
        panic!("Expected return stmt");
    }
}

#[test]
fn parses_binary_expression() {
    let parser = parse_fn("fn test() -> int { return 1 + 2 * 3; }");

    if let Stmt::Return(ret_stmt) = &parser[0].body[0] {
        match &ret_stmt.value {
            Some(val) => match val {
                Expr::Binary(bin) => {
                    matches!(bin.op, BinOp::Add);
                }
                _ => panic!("Expected binary expression"),
            },

            None => panic!("None value for return stmt"),
        }
    } else {
        panic!("Expected return stmt");
    }
}

#[test]
fn parses_nested_blocks() {
    let parser = parse_fn("fn test() -> int { { { print 1; } } }");

    assert_eq!(parser.len(), 1);
    let outer_block = &parser[0].body[0];
    assert!(matches!(outer_block, Stmt::Block(_)));
}

#[test]
fn parses_call_without_arg() {
    let parser = parse_fn("fn main() -> int { main(); }");

    assert_eq!(parser.len(), 1);
    let block = &parser[0].body[0];
    match block {
        Stmt::Expression(stmt) => match stmt {
            Expr::Call(call) => {
                match call.callee.clone() {
                    Expr::Variable(var) => {
                        assert_eq!(var.name, "main".to_string());
                    }
                    _ => panic!("Expected Expr::Variable"),
                }

                assert!(call.arg.is_none());
            }
            _ => panic!("Expressed call expression."),
        },
        _ => panic!("Expected expression stmt."),
    }
}

#[test]
fn parses_call_with_arg() {
    let parser = parse_fn("fn main() -> int { main(1 + 1); }");

    assert_eq!(parser.len(), 1);
    let block = &parser[0].body[0];
    match block {
        Stmt::Expression(stmt) => match stmt {
            Expr::Call(call) => {
                match call.callee.clone() {
                    Expr::Variable(var) => {
                        assert_eq!(var.name, "main".to_string());
                    }
                    _ => panic!("Expected Expr::Variable"),
                }

                match call.arg.clone() {
                    Some(arg) => match arg {
                        Expr::Binary(bin) => {
                            match bin.left {
                                Expr::Literal(_) => {}
                                _ => panic!("Expected literal"),
                            }

                            matches!(bin.op, BinOp::Add);
                            match bin.right {
                                Expr::Literal(_) => {}
                                _ => panic!("Expected literal"),
                            }
                        }
                        _ => panic!("Should have been a binary expr"),
                    },
                    None => panic!("arg is none."),
                }
            }
            _ => panic!("Expressed call expression."),
        },
        _ => panic!("Expected expression stmt."),
    }
}

#[test]
fn parses_native_call_without_arg() {
    let parser = parse_fn("fn main() -> int { extern main(); }");

    assert_eq!(parser.len(), 1);
    let block = &parser[0].body;
    match block.first().unwrap() {
        Stmt::Expression(stmt) => match stmt {
            Expr::NativeCall(call) => {
                match call.callee.clone() {
                    Expr::Variable(var) => {
                        assert_eq!(var.name, "main".to_string());
                    }
                    _ => panic!("Expected Expr::Variable"),
                }

                assert_eq!(call.args.len(), 0);
            }
            _ => panic!("Expressed call expression."),
        },
        _ => panic!("Expected expression stmt."),
    }
}
