mod expr;
pub mod parser;
mod stmt;
mod top_level;
mod utils;

pub const MAX_NATIVE_FUNCTION_ARITY: usize = 256;

pub type ParserResult<T> = Result<T, anyhow::Error>;
