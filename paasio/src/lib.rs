use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    bytes: usize,
    n_reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            bytes: 0,
            n_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.n_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.wrapped.read(buf) {
            Ok(n) => {
                self.bytes += n;
                self.n_reads += 1;
                Ok(n)
            }
            e => e,
        }
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes: usize,
    n_writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            bytes: 0,
            n_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.n_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.wrapped.write(buf) {
            Ok(n) => {
                self.bytes += n;
                self.n_writes += 1;
                Ok(n)
            }
            e => e,
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
