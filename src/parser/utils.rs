use super::parser::Parser;

use log::{error, trace};

use crate::{
    die,
    token::{Token, TokenType},
};

impl<'a> Parser<'a> {
    /// Consumes current token if it matches the given token type.
    /// * `expected_type` - type of token to match with.
    /// * `message` - error message for when token doesn't match the expected type
    pub(super) fn consume(&mut self, expected_type: TokenType, message: &str) -> Option<&Token> {
        if self.match_current(&expected_type) {
            return Some(self.advance());
        }

        self.report_parser_error(message);
        die!("{}", message);
    }

    /// Checks whether the current token is
    /// present in the given expected vec of tokens
    /// * `expected` - expected vec of tokens.
    pub(super) fn match_token(&mut self, expected: &[TokenType]) -> bool {
        for ttype in expected {
            if self.match_current(ttype) {
                self.advance();
                return true;
            }
        }

        false
    }

    /// Checks whether the current token is of given type
    /// * `expected_type` - the type to match with.
    fn match_current(&self, expected_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        expected_type == &self.current().ttype
    }

    /// Consumes current token and returns it.
    /// if reached the end, keeps returning the last token.
    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        let prev = self.previous();
        trace!("Parser::advance self.previous() = {}", &prev);
        prev
    }

    /// Returns current token.
    pub fn current(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Returns previous token.
    pub fn previous(&self) -> &Token {
        if self.current == 0 {
            die!("Parser::previous self.current = 0");
        }
        &self.tokens[self.current - 1]
    }

    /// Checks whether the end of the token stream has been reached.
    pub fn is_at_end(&self) -> bool {
        self.current().ttype == TokenType::Eof
    }

    /// Sets parser's error flag, then logs the given error message.
    /// * `message` - error message.
    pub fn report_parser_error<T: Into<String>>(&mut self, message: T) -> String {
        self.has_errors = true;
        let value: String = message.into();
        error!(
            "Parsing error: at line {}: {:?}",
            self.previous().line,
            value
        );

        value
    }
}
