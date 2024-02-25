use std::io::{Result, Write};

pub struct BitWriter<W: Write> {
    writer: W,
    buffer: u8,
    count: u8,
}

impl<W: Write> BitWriter<W> {
    pub fn new(writer: W) -> BitWriter<W> {
        BitWriter {
            writer,
            buffer: 0,
            count: 0,
        }
    }

    pub fn write(&mut self, bit: u8) -> Result<()> {
        self.buffer = self.buffer << 1 | bit;
        self.count += 1;
        if self.count == 8 {
            let buffer = self.buffer;
            self.buffer = 0;
            self.count = 0;
            self.writer.write_all(&[buffer])
        } else {
            Ok(())
        }
    }

    pub fn flush(&mut self) -> Result<()> {
        if self.count > 0 {
            self.writer.write_all(&[self.buffer << (8 - self.count)])?;
            self.buffer = 0;
            self.count = 0;
        }
        self.writer.flush()
    }
}

impl<W: Write> Drop for BitWriter<W> {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}