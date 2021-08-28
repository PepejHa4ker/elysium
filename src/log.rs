use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Error, Write};

pub struct Logger {
    file: File,
}

impl Logger {
    pub fn new() -> Self {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .read(true)
            .write(true)
            .open(concat!(env!("CARGO_MANIFEST_DIR"), "/log"))
            .unwrap();

        Self { file }
    }
}

impl Write for Logger {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let written = self.file.write(buf)?;

        self.file.flush()?;

        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
