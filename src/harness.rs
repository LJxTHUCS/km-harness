use crate::{command::Executor, port::HarnessPort};
use libafl_qemu_cmd::backdoor;

/// Harness for kernel model check.
pub struct Harness<P, E> {
    /// Port to communicate with checker.
    port: P,
    /// Command parser and executor.
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

    /// Init step of harness. Sends initial state to checker.
    pub fn init(&mut self) {
        self.port.start_state_sending();
        backdoor(); // Checker: Start
        while !self.port.send_state_data() {
            backdoor(); // Checker: GetState
        }
        self.port.finish_state_sending();
    }

    /// A normal test step of harness.
    ///
    /// 1. Recieve a command from input buffer.
    /// 2. Execute the command.
    /// 3. Serialize the return value to output buffer.
    pub fn step(&mut self) {
        backdoor(); // Checker: Command
        let command = self.port.receive_command();
        let retv = self.executor.parse_and_execute(command);
        self.port.send_retv(retv);
        backdoor(); // Checker: CheckRetv
        self.port.start_state_sending();
        backdoor(); // Checker: GetState
        while !self.port.send_state_data() {
            backdoor(); // Checker: GetState
        }
        self.port.finish_state_sending();
        backdoor(); // Checker: CheckState
    }
}
