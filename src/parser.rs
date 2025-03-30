use crate::token::{Token, TokenType};

pub struct Parser<'a> {
    /// Vec of tokens to parse.
    tokens: &'a Vec<Token>,

    /// Current token position.
    current: usize,

    /// flag to be set if any error occurs during parsing.
    has_errors: bool,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            has_errors: false,
        }
    }

    pub fn parse(&mut self) {}

    /// Consumes current token if it matches the given token type.
    /// * `expected_type` - type of token to match with.
    /// * `message` - error message for when token doesn't match the expected type
    fn consume(&mut self, expected_type: TokenType, message: &str) {
        if self.match_current(expected_type) {
            self.advance();
        }

        self.report_parser_error(message);
    }

    /// Checks whether the current token is
    /// present in the given expected vec of tokens
    /// * `expected` - expected vec of tokens.
    fn match_token(&self, expected: &[Token]) -> bool {
        let current = self.current();
        for token in expected {
            if token.ttype == current.ttype {
                return true;
            }
        }

        false
    }

    /// Checks whether the current token is of given type
    /// * `expected_type` - the type to match with.
    fn match_current(&self, expected_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.current().ttype == expected_type
    }

    /// Consumes current token and returns it.
    /// if reached the end, keeps returning the last token.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    /// Returns current token.
    fn current(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Returns previous token.
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    /// Checks whether the end of the token stream has been reached.
    fn is_at_end(&self) -> bool {
        self.tokens.len() >= self.current
    }

    /// Sets parser's error flag, then logs the given error message.
    /// * `message` - error message.
    fn report_parser_error(&mut self, message: &str) {
        self.has_errors = true;
        println!("Parser Error at line {} : {}", self.current().line, message);
    }
}
