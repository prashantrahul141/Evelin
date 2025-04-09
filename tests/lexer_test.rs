use evelin::lexer::{Lexer, LiteralValue, TokenType};

fn tokenize<T: Into<String>>(input: T) -> Vec<evelin::lexer::Token> {
    let l = input.into();
    let mut lexer = Lexer::from(&l);
    lexer.start();
    let t = lexer.tokens();
    (*t).clone()
}

#[test]
fn test_single_char_tokens() {
    let input = "(){},.-+:;*%";
    let tokens = tokenize(input);
    let expected = vec![
        TokenType::LeftParen,
        TokenType::RightParen,
        TokenType::LeftBrace,
        TokenType::RightBrace,
        TokenType::Comma,
        TokenType::Dot,
        TokenType::Minus,
        TokenType::Plus,
        TokenType::Colon,
        TokenType::Semicolon,
        TokenType::Star,
        TokenType::Mod,
        TokenType::Eof,
    ];
    let actual: Vec<_> = tokens.iter().map(|t| t.ttype.clone()).collect();
    assert_eq!(expected, actual);
}

#[test]
fn test_operators() {
    let input = "! != = == < <= > >=";
    let tokens = tokenize(input);
    let expected = vec![
        TokenType::Bang,
        TokenType::BangEqual,
        TokenType::Equal,
        TokenType::EqualEqual,
        TokenType::Less,
        TokenType::LessEqual,
        TokenType::Greater,
        TokenType::GreaterEqual,
        TokenType::Eof,
    ];
    let actual: Vec<_> = tokens.iter().map(|t| t.ttype.clone()).collect();
    assert_eq!(expected, actual);
}

#[test]
fn test_integer() {
    let tokens = tokenize("12345");
    assert_eq!(tokens[0].ttype, TokenType::NumberInt);
    matches!(tokens[0].literal, LiteralValue::NumberInt(12345));
}

#[test]
fn test_float() {
    let tokens = tokenize("3.14");
    assert_eq!(tokens[0].ttype, TokenType::NumberFloat);
    matches!(tokens[0].literal, LiteralValue::NumberFloat(3.14));
}

#[test]
fn test_string() {
    let tokens = tokenize("\"hello\"");
    let _l = LiteralValue::String("hello".to_owned());
    assert_eq!(tokens[0].ttype, TokenType::String);
    matches!(tokens[0].literal.clone(), _l);
}

#[test]
fn test_identifier() {
    let tokens = tokenize("fooBar");
    assert_eq!(tokens[0].ttype, TokenType::Identifier);
    assert_eq!(tokens[0].lexeme, "fooBar");
}

#[test]
fn test_keywords() {
    let keywords = vec![
        ("true", TokenType::True),
        ("false", TokenType::False),
        ("null", TokenType::Null),
        ("and", TokenType::And),
        ("or", TokenType::Or),
        ("let", TokenType::Let),
        ("fn", TokenType::Fn),
        ("return", TokenType::Return),
        ("if", TokenType::If),
        ("else", TokenType::Else),
        ("print", TokenType::Print),
        ("struct", TokenType::Struct),
        ("extern", TokenType::Extern),
    ];

    for (kw_str, expected_type) in keywords {
        let tokens = tokenize(kw_str);
        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens[0].ttype, expected_type,
            "Failed for keyword: {}",
            kw_str
        );
    }
}

#[test]
fn test_comment_skipping() {
    let tokens = tokenize("// comment\n+");
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].ttype, TokenType::Plus);
}

#[test]
#[should_panic(expected = "Illegal character")]
fn test_illegal_char() {
    tokenize("@");
}

#[test]
#[should_panic(expected = "Non terminated string.")]
fn test_unterminated_string() {
    tokenize("\"unclosed");
}
