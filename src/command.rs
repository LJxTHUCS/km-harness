/// Command executed on kernel.
pub trait Command {
    /// Execute the command on kernel.
    fn execute(&self) -> isize;
}

/// Command deserializer and executor.
pub trait Executor {
    /// Deserialize a command from byte slice and execute it.
    fn deser_and_exec(&self, buf: &[u8]) -> Result<isize, ()>;
}
