use std::command::Command;
use std::io;
use std::path::{Path, PathBuf};

pub struct Injector {
    path: PathBuf,
}

impl Injector {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().into(),
        }
    }

    pub fn inject(&self, library: impl AsRef<Path>) -> io::Result<()> {
        let library = library.as_ref();
        let mut command = Command::new(&self.injector);

        command.arg("-n");
        command.arg("csgo_linux64");
        command.arg(library);

        let mut child = command.spawn()?;

        child.wait()?;

        Ok(())
    }
}
