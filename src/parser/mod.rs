mod expr;
pub mod parser;
mod stmt;
mod top_level;
mod utils;

pub const MAX_NATIVE_FUNCTION_ARITY: usize = 127;

pub type ParserResult<T> = Result<T, anyhow::Error>;
