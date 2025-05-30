use super::Parser;

use anyhow::bail;
use log::{error, trace};

use crate::{
    ast::{Token, TokenType},
    die,
    utils::{ErrorType, MessageType, report_message},
};

impl Parser<'_> {
    /// Consumes current token if it matches the given token type.
    /// * `expected_type` - type of token to match with.
    /// * `message` - error message for when token doesn't match the expected type
    pub(super) fn consume(
        &mut self,
        expected_type: TokenType,
        message: &str,
    ) -> anyhow::Result<&Token> {
        if self.match_current(&expected_type) {
            return Ok(self.advance());
        }

        bail!("{}, line {}", message.to_owned(), self.current().line);
    }

    /// Checks whether the current token is
    /// present in the given expected vec of tokens
    /// and consumes it
    /// * `expected` - expected vec of tokens.
    pub(super) fn match_token(&mut self, expected: &[TokenType]) -> bool {
        expected.iter().any(|ttype| {
            if self.match_current(ttype) {
                self.advance();
                true
            } else {
                false
            }
        })
    }

    /// Checks whether the current token is of given type
    /// * `expected_type` - the type to match with.
    pub(super) fn match_current(&self, expected_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        trace!(
            "expected_type = {expected_type}, current = {}",
            &self.current().ttype
        );
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

    /// Increments parser's error count, reports the error message to user,
    /// then synchronizes to next statement.
    /// * `message` - error.
    /// * `sync` - whether to synchronize or not
    pub fn report_parser_error(&mut self, err: anyhow::Error, sync: bool) {
        self.errors_count += 1;
        report_message(
            format!("at line {}: {:#}.", self.current().line, err),
            MessageType::Error(ErrorType::ParsingError),
        );

        if sync {
            self.synchronize();
        }
    }

    /// Returns the next token without consuming it.
    pub fn peek(&self) -> &Token {
        if self.current + 1 >= self.tokens.len() {
            return self.tokens.last().unwrap();
        }

        &self.tokens[self.current + 1]
    }

    /// Synchronizes: consumes all tokens untill next meaningful statement.
    fn synchronize(&mut self) {
        trace!("trying to synchronize");
        self.advance();

        while !self.is_at_end() {
            if self.previous().ttype == TokenType::Semicolon {
                trace!("found semicolon, ending synchronize");
                return;
            }

            match self.current().ttype {
                TokenType::Struct
                | TokenType::Fn
                | TokenType::Let
                | TokenType::Return
                | TokenType::If
                | TokenType::Print
                | TokenType::Extern => {
                    trace!("Found new statement beginner token ending synchronize");
                    return;
                }
                _ => trace!("didnt match any new statement beginner token."),
            };

            self.advance();
        }
    }

    /// Synchronizes at top level: consumes all tokens untill next fn or struct decl
    pub(super) fn synchronize_toplevel(&mut self) {
        trace!("trying to synchronize at top level");
        self.advance();

        while !self.is_at_end() {
            match self.current().ttype {
                TokenType::Struct | TokenType::Fn => {
                    trace!("Found new fn or struct decl token, ending top level synchronize");
                    return;
                }
                _ => trace!("didnt match any new fn or struct token."),
            };

            self.advance();
        }
    }
}
