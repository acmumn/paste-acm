use std::io::{Read, Result};

/// An adaptor for Read that enforces a maximum length, returning an error if
/// it is exceeded.
pub struct MaxLength<R: Read>(R, usize);

impl<R: Read> MaxLength<R> {
    /// Creates a new MaxLength with the given reader and maximum length.
    pub fn new(r: R, max_length: usize) -> MaxLength<R> {
        MaxLength(r, max_length)
    }
}

impl<R: Read> Read for MaxLength<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let l = buf.len();
        let buf = if self.1 > l { buf } else { &mut buf[0..l] };
        let n = self.0.read(buf)?;
        self.1 -= n;
        Ok(n)
    }
}
