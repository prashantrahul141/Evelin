mod expr;
pub mod parser;
mod utils;

pub type ParserResult<T> = Result<T, anyhow::Error>;
