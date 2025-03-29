use log::{debug, trace};

use crate::token::{Token, TokenType};

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
    pub tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    /// Init a lexer.
    ///* `in_src` - input source string.
    pub fn new(in_src: &'a String) -> Self {
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

    /// Starts lexer
    pub fn start(&mut self) {
        debug!("start scanning tokens.");
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
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
            '-' => self.add_basic_token(TokenType::Minus),
            '+' => self.add_basic_token(TokenType::Plus),
            ';' => self.add_basic_token(TokenType::Semicolon),
            '*' => self.add_basic_token(TokenType::Star),
            '%' => self.add_basic_token(TokenType::Mod),

            '\n' => {
                self.line += 1;
            }

            ' ' | '\t' | '\r' => {}

            '/' => {
                if self.match_char('/') {
                    // a line starting with '//'
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

            '"' => {}
            _ => {}
        }
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
        let token = Token {
            ttype,
            lexeme: self.in_src[self.start..self.current].to_string(),
            line: self.line,
            literal: crate::token::TokenLiteral::Null,
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
}
