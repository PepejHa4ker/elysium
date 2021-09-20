use crate::console::Console;
use parking_lot::{RwLock, RwLockWriteGuard};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Error, Write};
use std::sync::Arc;

struct LoggerRef {
    file: File,
}

#[derive(Clone)]
pub struct Logger(Arc<RwLock<LoggerRef>>);

impl Logger {
    pub fn new() -> Self {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .read(true)
            .write(true)
            .open(concat!(env!("CARGO_MANIFEST_DIR"), "/log"))
            .unwrap();

        Self(Arc::new(RwLock::new(LoggerRef { file })))
    }

    fn lock(&self) -> RwLockWriteGuard<'_, LoggerRef> {
        self.0.write()
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut lock = self.lock();
        let written = lock.file.write(buf)?;

        lock.file.flush()?;

        Ok(written)
    }
}

impl Write for Logger {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
