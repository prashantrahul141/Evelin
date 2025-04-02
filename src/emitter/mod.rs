pub mod qbe_emitter;

pub type EmitterResult<T> = Result<T, String>;

pub trait Emitter {
    fn emit_ir(&mut self) -> EmitterResult<String>;
}
