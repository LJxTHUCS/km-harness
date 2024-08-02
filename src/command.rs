/// Command executed on kernel.
pub trait Command: Sized {
    /// Deserialize a command from byte slice.
    fn from_bytes(buf: &[u8]) -> Option<Self>;
    /// Execute the command on kernel.
    fn execute(&self) -> isize;
}

/// Command parser and executor.
pub trait Executor {
    /// Parse a command from byte slice and execute it.
    fn parse_and_execute(&self, buf: &[u8]) -> Result<isize, ()>;
}
