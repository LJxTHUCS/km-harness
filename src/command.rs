/// Command executed on kernel.
pub trait Command: Sized {
    /// Deserialize a command from byte slice.
    fn from_bytes(buf: &[u8]) -> Self;

    /// Execute the command on kernel.
    ///
    /// - isize return value.
    /// - Write additional output to `output`.
    fn execute(&self, output: &mut [u8]) -> isize;
}

/// Command parser and executor.
pub trait Executor {
    /// Parse a command from byte slice and execute it.
    /// 
    /// - isize return value.
    /// - Parse command from `cmd_buf`.
    /// - Write additional output to `output`.
    fn parse_and_execute(&self, cmd_buf: &[u8], output: &mut [u8]) -> isize;
}
