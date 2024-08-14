use crate::{command::Executor, port::HarnessPort};
use libafl_qemu_cmd::backdoor;

/// Harness for kernel model check.
pub struct Harness<P, E, const BUF_SIZE: usize> {
    /// Port to communicate with checker.
    port: P,
    /// Command parser and executor.
    executor: E,
    /// Data buffer.
    buf: [u8; BUF_SIZE],
}

impl<P, E, const BUF_SIZE: usize> Harness<P, E, BUF_SIZE>
where
    P: HarnessPort,
    E: Executor,
{
    /// Creates a new harness instance.
    pub fn new(port: P, executor: E) -> Self {
        Harness {
            port,
            executor,
            buf: [0u8; BUF_SIZE],
        }
    }

    /// A normal test step of harness.
    ///
    /// 1. Recieve a command from input buffer.
    /// 2. Execute the command.
    /// 3. Serialize the return value to retv buffer.
    /// 4. Write extra data to data buffer.
    pub fn step(&mut self) {
        backdoor(); // Checker: Command
        let command = self.port.receive_command();
        let retv = self.executor.parse_and_execute(command, &mut self.buf);
        self.port.send_retv(retv);
        self.port.send_extra_data(&self.buf);
    }
}
