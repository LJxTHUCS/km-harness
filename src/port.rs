/// An interface for receiving commands and sending results to checker.
pub trait HarnessPort {
    /// Receive command from checker.
    fn receive_command(&mut self) -> &[u8];

    /// Send return value of the command to checker.
    fn send_retv(&mut self, retv: isize);

    /// (optional) Send extra data to checker.
    ///
    /// Some commands may return extra data, such as some special structs.
    /// Like `fstat` returns a `stat` structure.
    fn send_extra_data(&mut self, _data: &[u8]) {}
}

/// Facilitates receiving commands and sending results via memory buffer.
pub struct MemPort<'a> {
    cmd_buf: &'a [u8],
    retv_buf: &'a mut [u8; core::mem::size_of::<isize>()],
    output_buf: &'a mut [u8],
}

impl<'a> MemPort<'a> {
    /// Create a new `BufferPort` with given input and output buffers.
    pub fn new(
        cmd_buf: &'a [u8],
        retv_buf: &'a mut [u8; core::mem::size_of::<isize>()],
        output_buf: &'a mut [u8],
    ) -> Self {
        Self {
            cmd_buf,
            retv_buf,
            output_buf,
        }
    }
}

impl<'a> HarnessPort for MemPort<'a> {
    fn receive_command(&mut self) -> &[u8] {
        self.cmd_buf
    }
    fn send_retv(&mut self, retv: isize) {
        self.retv_buf.copy_from_slice(&retv.to_le_bytes());
    }
    fn send_extra_data(&mut self, data: &[u8]) {
        self.output_buf.copy_from_slice(data);
    }
}
