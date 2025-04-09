use evelin::ast::{BinOp, Expr, FnDecl, LiteralValue, Stmt, StructDecl, TokenType};
use evelin::lexer::Lexer;
use evelin::parser::Parser;

fn tokenize<T: Into<String>>(input: T) -> Vec<evelin::lexer::Token> {
    let l = input.into();
    let mut lexer = Lexer::from(&l);
    lexer.start();
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
    let parser = parser_struct("struct Point { x, y }");

    assert_eq!(parser.len(), 1);
    let s = &parser[0];
    assert_eq!(s.name, "Point");
    assert_eq!(s.fields, vec!["x", "y"]);
}

#[test]
fn parses_function_without_param() {
    let parser = parse_fn("fn main() { return 42; }");

    assert_eq!(parser.len(), 1);
    let f = &parser[0];
    assert_eq!(f.name, "main");
    assert!(f.parameter.is_none());
    assert!(!f.body.is_empty());
}

#[test]
fn parses_function_with_param() {
    let parser = parse_fn("fn inc(x: i32) { return x; }");

    assert_eq!(parser.len(), 1);
    let f = &parser[0];
    assert_eq!(f.name, "inc");
    assert_eq!(
        f.parameter.as_ref(),
        Some(&("x".to_owned(), TokenType::TypeI64))
    );
}

#[test]
fn parses_if_else_statement() {
    let parser = parse_fn("fn test() { if (true) { print 1; } else { print 2; } }");

    let body = &parser[0].body;
    assert!(matches!(body[0], Stmt::If(_)));
}

#[test]
fn parses_literal_expression() {
    let parser = parse_fn("fn test() { return 123; }");

    if let Stmt::Return(ret_stmt) = &parser[0].body[0] {
        match &ret_stmt.value {
            Expr::Literal(lit) => {
                matches!(lit.value, LiteralValue::NumberInt(123));
            }
            _ => panic!("Expected literal int"),
        }
    } else {
        panic!("Expected return stmt");
    }
}

#[test]
fn parses_binary_expression() {
    let parser = parse_fn("fn test() { return 1 + 2 * 3; }");

    if let Stmt::Return(ret_stmt) = &parser[0].body[0] {
        match &ret_stmt.value {
            Expr::Binary(bin) => {
                matches!(bin.op, BinOp::Add);
            }
            _ => panic!("Expected binary expression"),
        }
    } else {
        panic!("Expected return stmt");
    }
}

#[test]
fn parses_nested_blocks() {
    let parser = parse_fn("fn test() { { { print 1; } } }");

    assert_eq!(parser.len(), 1);
    let outer_block = &parser[0].body[0];
    assert!(matches!(outer_block, Stmt::Block(_)));
}
