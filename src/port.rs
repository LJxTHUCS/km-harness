/// Communicate with checker: receive command and send result.
pub trait HarnessPort {
    /// Receive command from checker
    fn get_command(&mut self) -> &[u8];
    /// Send result to checker
    fn send_result(&mut self, result: &[u8]);
}

/// Simple port using bytes buffer to communicate with checker.
pub struct BufferPort<'a> {
    in_buf: &'a [u8],
    out_buf: &'a mut [u8],
}

impl<'a> BufferPort<'a> {
    /// Create a new `BufferPort` with given input and output buffers.
    pub fn new(in_buf: &'a [u8], out_buf: &'a mut [u8]) -> Self {
        Self { in_buf, out_buf }
    }
}

impl<'a> HarnessPort for BufferPort<'a> {
    fn get_command(&mut self) -> &[u8] {
        self.in_buf
    }
    fn send_result(&mut self, result: &[u8]) {
        self.out_buf[..result.len()].copy_from_slice(result);
    }
}
