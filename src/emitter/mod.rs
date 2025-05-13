pub mod qbee;

pub type EmitterResult<T> = anyhow::Result<T>;

pub trait Emitter {
    fn emit_ir(&mut self) -> EmitterResult<String>;
}
