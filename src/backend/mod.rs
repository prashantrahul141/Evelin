pub mod qbe_backend;

/// Trait for Backend
/// A backend's job is to take ir as String
/// and return native asm as String.
pub trait Backend {
    /// Public function to generate asm from ir.
    /// * `ir` - ir for the backend.
    fn generate(&self, ir: String) -> Result<String, anyhow::Error>;
}
