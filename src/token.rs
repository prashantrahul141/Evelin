/// All types of token.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // single-character tokens.
    LeftParen,  // (
    RightParen, // )
    LeftBrace,  // {
    RightBrace, // }
    Comma,      // ,
    Dot,        // .
    Minus,      // -
    Plus,       // +
    Semicolon,  // ;
    Slash,      // /
    Star,       // *
    Mod,        // %

    // one or two character tokens.
    Bang,         // !
    BangEqual,    // !=
    Equal,        // =
    EqualEqual,   // ==
    Greater,      // >
    GreaterEqual, // >=
    Less,         // <
    LessEqual,    // <=

    // literals.
    Identifier,  // variables, function names, class names.
    String,      // Strings.
    NumberInt,   // numbers : integers.
    NumberFloat, // numbers : floats.

    // keywords
    True,   // true
    False,  // false
    Null,   // null
    And,    // and
    Or,     // or
    Let,    // let
    Fn,     // fn
    Return, // return
    If,     // if
    Else,   // else
    Print,  // print

    Eof, // end of file.
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Some tokens contains values with them.
#[derive(Debug)]
pub enum TokenLiteral {
    NumberFloat(f64),
    NumberInt(i64),
    String(String),
    Boolean(bool),
    Null,
}

// The token type.
#[derive(Debug)]
pub struct Token {
    // type of the token.
    pub ttype: TokenType,

    // literal present as is in the source code.
    pub lexeme: String,

    // parsed literal
    pub literal: TokenLiteral,

    // at which line number in the source.
    pub line: usize,
}

static RESERVED_KEYWORDS_KEYS: [&'static str; 11] = [
    "true", "false", "null", "and", "or", "let", "fn", "return", "if", "else", "print",
];

static RESERVED_KEYWORDS_TYPES: [TokenType; 11] = [
    TokenType::True,
    TokenType::False,
    TokenType::And,
    TokenType::Null,
    TokenType::Or,
    TokenType::Let,
    TokenType::Fn,
    TokenType::Return,
    TokenType::If,
    TokenType::Else,
    TokenType::Print,
];

pub fn is_reserved(target: &str) -> bool {
    RESERVED_KEYWORDS_KEYS.contains(&target)
}

/// Get TokenType for reserved keyword.
pub fn get_type_from_reserved(target: &str) -> Option<TokenType> {
    return if is_reserved(target) {
        Some(
            RESERVED_KEYWORDS_TYPES[RESERVED_KEYWORDS_KEYS
                .iter()
                .position(|&x| x == target)
                .unwrap()]
            .clone(),
        )
    } else {
        None
    };
}
