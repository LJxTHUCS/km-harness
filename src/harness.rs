use crate::{command::Executor, port::HarnessPort};
use libafl_qemu_cmd::backdoor;

/// Harness for kernel model check.
///
/// Provides interfaces for command execution and output serialization.
pub struct Harness<P, E> {
    port: P,
    executor: E,
}

impl<P, E> Harness<P, E>
where
    P: HarnessPort,
    E: Executor,
{
    /// Creates a new harness instance.
    pub fn new(port: P, executor: E) -> Self {
        Harness { port, executor }
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
        let command = self.port.get_command();
        let retv = self.executor.parse_and_execute(command)?;
        let bytes = retv.to_le_bytes();
        self.port.send_result(&bytes);
        backdoor();
        Ok(())
    }
}
