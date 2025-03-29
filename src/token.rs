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
    Identifier, // variables, function names, class names.
    String,     // Strings.
    Number,     // numbers : integers, floats.

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
    While,  // while
    For,    // for
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
    Number(f64),
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
