use crate::token::Token;

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
}
