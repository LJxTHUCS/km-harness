/// Trait for receiving commands and sending return values to checker.
pub trait CommandChannel {
    /// Receive command from checker
    fn receive_command(&mut self) -> &[u8];

    /// Send return value of the command to checker
    fn send_retv(&mut self, retv: isize);
}

/// Trait for abstracting state and sending it to checker.
pub trait StateChannel {
    /// Start the process of state sending.
    fn start_state_sending(&mut self);

    /// Fetch state data and send it to checker.
    /// 
    /// Return `true` if checker needs no more data, then `finish_state_sending` 
    /// should be called.
    fn send_state_data(&mut self) -> bool;

    /// Complete the state sending process.
    fn finish_state_sending(&mut self);
}

/// A unified interface for interacting with checker.
///
/// 1. `CommandChannel`: Receive commands and send return values.
/// 2. `StateChannel`: Send state to checker.
pub trait HarnessPort: CommandChannel + StateChannel {}

/// Facilitates receiving commands and sending results via memory buffer.
pub struct MemCommandChannnel<'a> {
    in_buf: &'a [u8],
    out_buf: &'a mut [u8],
}

impl<'a> MemCommandChannnel<'a> {
    /// Create a new `BufferPort` with given input and output buffers.
    pub fn new(in_buf: &'a [u8], out_buf: &'a mut [u8]) -> Self {
        Self { in_buf, out_buf }
    }
}

impl<'a> CommandChannel for MemCommandChannnel<'a> {
    fn receive_command(&mut self) -> &[u8] {
        self.in_buf
    }
    fn send_retv(&mut self, retv: isize) {
        self.out_buf[..core::mem::size_of::<isize>()].copy_from_slice(&retv.to_le_bytes());
    }
}
