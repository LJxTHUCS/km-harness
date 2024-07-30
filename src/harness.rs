use crate::command::Executor;
use libafl_qemu_cmd::backdoor;

/// Harness for kernel model check.
///
/// Provides interfaces for command execution and output serialization.
pub struct Harness<'a, E> {
    in_buf: &'a [u8],
    out_buf: &'a mut [u8],
    executor: E,
}

impl<'a, E> Harness<'a, E>
where
    E: Executor,
{
    /// Creates a new harness instance.
    pub fn new(in_buf: &'a [u8], out_buf: &'a mut [u8], executor: E) -> Self {
        Harness {
            in_buf,
            out_buf,
            executor,
        }
    }

    /// Init step of harness.
    pub fn init(&mut self) {
        backdoor();
    }

    /// A normal test step of harness.
    ///
    /// 1. Recieve a command from input buffer.
    /// 2. Execute the command.
    /// 3. Serialize the return value to output buffer.
    pub fn step(&mut self) -> Result<(), ()> {
        backdoor();
        let retv = self.executor.deser_and_exec(self.in_buf)?;
        let bytes = retv.to_le_bytes();
        self.out_buf[..bytes.len()].copy_from_slice(&bytes);
        backdoor();
        Ok(())
    }
}
