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
    Struct, // struct
    Extern, // extern

    Eof, // end of file.
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Some tokens contains values with them.
#[derive(Debug, Clone)]
pub enum LiteralValue {
    NumberFloat(f64),
    NumberInt(i64),
    String(String),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::NumberFloat(v) => write!(f, "{}", v),
            LiteralValue::NumberInt(v) => write!(f, "{}", v),
            LiteralValue::String(v) => write!(f, "{}", v),
            LiteralValue::Boolean(v) => write!(f, "{}", v),
            LiteralValue::Null => write!(f, "null"),
        }
    }
}

// The token type.
#[derive(Debug, Clone)]
pub struct Token {
    // type of the token.
    pub ttype: TokenType,

    // literal present as is in the source code.
    pub lexeme: String,

    // parsed literal
    pub literal: LiteralValue,

    // at which line number in the source.
    pub line: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// static array of all reserved keywords.
static RESERVED_KEYWORDS_KEYS: [&'static str; 13] = [
    "true", "false", "null", "and", "or", "let", "fn", "return", "if", "else", "print", "struct",
    "extern",
];

/// TokenTypes which are reserved keywords,
/// THIS HAS TO BE IN SAME ORDER AS RESERVED_KEYWORDS_KEYS
static RESERVED_KEYWORDS_TYPES: [TokenType; 13] = [
    TokenType::True,
    TokenType::False,
    TokenType::Null,
    TokenType::And,
    TokenType::Or,
    TokenType::Let,
    TokenType::Fn,
    TokenType::Return,
    TokenType::If,
    TokenType::Else,
    TokenType::Print,
    TokenType::Struct,
    TokenType::Extern,
];

/// Checks whether given &str is a reserved keyword or not
pub fn is_reserved(target: &str) -> bool {
    RESERVED_KEYWORDS_KEYS.contains(&target)
}

/// converts &str to reserved keyword type if it is infact reserved.
impl<'a> TryFrom<&'a str> for TokenType {
    type Error = anyhow::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        return if is_reserved(value) {
            Ok(RESERVED_KEYWORDS_TYPES[RESERVED_KEYWORDS_KEYS
                .iter()
                .position(|&x| x == value)
                .unwrap()]
            .clone())
        } else {
            Err(anyhow::Error::msg(format!(
                "Not a reserved keyword: '{}'.",
                value
            )))
        };
    }
}
