use log::{debug, error, trace};

pub use crate::ast::{LiteralValue, Token, TokenType};
use crate::die;
use crate::utils::{is_alpha, is_alphanumeric, is_numeric};

pub struct Lexer<'a> {
    // Input source as String.
    in_src: &'a String,

    // input source string as characters.
    in_chars: Vec<char>,

    // input string length.
    in_length: usize,

    // start position of current scanning token.
    start: usize,

    // current cursor poisiton
    current: usize,

    // line number
    line: usize,

    // scanned tokens
    tokens: Vec<Token>,
}

impl<'a> From<&'a String> for Lexer<'a> {
    fn from(in_src: &'a String) -> Self {
        Self {
            in_src,
            in_chars: in_src.clone().chars().collect(),
            in_length: in_src.len(),
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
        }
    }
}

impl Lexer<'_> {
    /// Starts lexer
    pub fn start(&mut self) {
        debug!("start scanning tokens.");
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_basic_token(TokenType::Eof);
    }

    /// Scans indiviual tokens.
    fn scan_token(&mut self) {
        let current_char = self.advance();
        match current_char {
            '(' => self.add_basic_token(TokenType::LeftParen),
            ')' => self.add_basic_token(TokenType::RightParen),
            '{' => self.add_basic_token(TokenType::LeftBrace),
            '}' => self.add_basic_token(TokenType::RightBrace),
            ',' => self.add_basic_token(TokenType::Comma),
            '.' => self.add_basic_token(TokenType::Dot),
            '+' => self.add_basic_token(TokenType::Plus),
            ':' => self.add_basic_token(TokenType::Colon),
            ';' => self.add_basic_token(TokenType::Semicolon),
            '*' => self.add_basic_token(TokenType::Star),
            '%' => self.add_basic_token(TokenType::Mod),

            '\n' => {
                self.line += 1;
            }

            ' ' | '\t' | '\r' => {}

            '/' => {
                // a line starting with '//'
                if self.match_char('/') {
                    // consuming everything until end.
                    while !self.is_at_end() && self.look_ahead() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_basic_token(TokenType::Slash);
                }
            }

            '!' => {
                if self.match_char('=') {
                    self.add_basic_token(TokenType::BangEqual);
                } else {
                    self.add_basic_token(TokenType::Bang);
                }
            }

            '=' => {
                if self.match_char('=') {
                    self.add_basic_token(TokenType::EqualEqual);
                } else {
                    self.add_basic_token(TokenType::Equal);
                }
            }

            '>' => {
                if self.match_char('=') {
                    self.add_basic_token(TokenType::GreaterEqual);
                } else {
                    self.add_basic_token(TokenType::Greater);
                }
            }

            '<' => {
                if self.match_char('=') {
                    self.add_basic_token(TokenType::LessEqual);
                } else {
                    self.add_basic_token(TokenType::Less);
                }
            }

            '-' => {
                if self.match_char('>') {
                    self.add_basic_token(TokenType::FatArrow);
                } else {
                    self.add_basic_token(TokenType::Minus);
                }
            }

            '"' => {
                self.scan_string();
            }

            // All other we need to either parse
            // 1. numbers
            // 2. identifier (this includes both reserved keywords and user defined identifier)
            // everything else is then illegal and will raise an error if found.
            _ => {
                if is_numeric(current_char) {
                    // 1. parses numbers.
                    self.scan_number();
                } else if is_alpha(current_char) {
                    // 2. parses identifier.
                    self.scan_identifier();
                } else {
                    // 3. everything else is illegal.
                    die!(
                        "Illegal character found at line {} : {}",
                        self.line,
                        current_char
                    );
                }
            }
        }
    }

    /// Scans identifiers, reserved or user defined
    fn scan_identifier(&mut self) {
        trace!("scanning identifier");
        while is_alphanumeric(self.look_ahead()) {
            self.advance();
        }
        let lexeme = self.in_src[self.start..self.current].to_string();

        match TokenType::try_from(lexeme.as_str()) {
            Ok(ttype) => self.add_token(ttype, lexeme, LiteralValue::Null),
            Err(_) => self.add_token(
                TokenType::Identifier,
                lexeme.clone(),
                LiteralValue::String(lexeme),
            ),
        }
    }

    /// Scans a number.
    fn scan_number(&mut self) {
        trace!("scanning number");
        let mut is_float = false;
        while is_numeric(self.look_ahead()) {
            self.advance();
        }

        if self.look_ahead() == '.' && is_numeric(self.look_ahead_twice()) {
            trace!("number is a float.");
            is_float = true;

            // consume the '.'
            self.advance();

            // consume rest of the characters.
            while is_numeric(self.look_ahead()) {
                self.advance();
            }
        }

        // getting the literal.
        let lexeme = self.in_src[self.start..self.current].to_string();

        if is_float {
            let literal = LiteralValue::NumberFloat(lexeme.parse::<f64>().unwrap());
            self.add_token(TokenType::NumberFloat, lexeme, literal);
        } else {
            let literal = LiteralValue::NumberInt(lexeme.parse::<i64>().unwrap());
            self.add_token(TokenType::NumberInt, lexeme, literal);
        }
    }

    /// Scans a string.
    fn scan_string(&mut self) {
        trace!("scanning string");
        // consume until a single-double quote or the stream ends.
        while !self.is_at_end() && self.look_ahead() != '"' {
            if self.look_ahead() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            die!("Non terminated string.");
        }

        // consume "
        self.advance();

        // add token.
        let literal = self.in_src[self.start + 1..self.current - 1].to_string();
        self.add_token(
            TokenType::String,
            literal.clone(),
            LiteralValue::String(literal),
        )
    }

    /// Consumes and returns next character in the input stream.
    fn advance(&mut self) -> char {
        let current_char = self.in_chars[self.current];
        self.current += 1;
        trace!("current char : {}", current_char);
        current_char
    }

    /// Adds a basic token from its type.
    fn add_basic_token(&mut self, ttype: TokenType) {
        self.add_token(
            ttype,
            self.in_src[self.start..self.current].to_string(),
            LiteralValue::Null,
        );
    }

    /// Adds a new token from arguments.
    fn add_token(&mut self, ttype: TokenType, lexeme: String, literal: LiteralValue) {
        let token = Token {
            ttype,
            lexeme,
            line: self.line,
            literal,
        };
        trace!("Added token = {:?}", token);
        self.tokens.push(token);
    }

    /// Checks whether stream of tokens ended.
    fn is_at_end(&self) -> bool {
        self.current >= self.in_length
    }

    /// matches current char with expected, consumes if same.
    /// also returns whether it matches or not.
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.in_chars[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    // like advance but doesn't consume the token.
    fn look_ahead(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.in_chars[self.current]
    }

    // like look_ahead but looks two positions forward.
    fn look_ahead_twice(&self) -> char {
        if self.current + 1 >= self.in_length {
            return '\0';
        }

        self.in_chars[self.current + 1]
    }

    // "getter" method for tokens
    // returns a non mutable references to tokens vec.
    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
