use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PtraceScope {
    Default,
    Restrict,
    Admin,
    Disabled,
}

impl PtraceScope {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str> {
        use PtraceScope::*;

        if bytes.len() != 1 {
            return Err("invalid scope");
        }

        let scope = match bytes.get(0) {
            Some(b'0') => Default,
            Some(b'1') => Restrict,
            Some(b'2') => Admin,
            Some(b'3') => Disabled,
            _ => return Err("invalid scope"),
        };

        Ok(scope)
    }

    pub fn as_str(&self) -> &str {
        use PtraceScope::*;

        match self {
            Default => "default",
            Restrict => "restrict",
            Admin => "admin",
            Disabled => "disabled",
        }
    }

    fn as_sys_str(&self) -> &str {
        use PtraceScope::*;

        match self {
            Default => "0",
            Restrict => "1",
            Admin => "2",
            Disabled => "3",
        }
    }
}

impl FromStr for PtraceScope {
    type Err = &'static str;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        use PtraceScope::*;

        let scope = match string {
            "default" => Default,
            "restrict" => Restrict,
            "admin" => Admin,
            "disabled" => Disabled,
            _ => return Err("invalid scope"),
        };

        Ok(scope)
    }
}

pub struct Helper {
    setuid_helper: PathBuf,
}

impl Helper {
    pub fn new(setuid_helper: impl AsRef<Path>) -> Self {
        Self {
            setuid_helper: setuid_helper.as_ref().into(),
        }
    }

    pub fn ensure_ready() -> io::Result<()> {
        let mut command = Command::new("cargo");

        command.arg("build");
        command.arg("--release");
        command.arg("--package");
        command.arg("toolbox-helper");

        let mut child = command.spawn().unwrap();
        let status = child.wait().unwrap();

        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "child returned an error",
            ));
        }

        Ok(())
    }

    pub fn set_yama_scope(&self, scope: PtraceScope) -> io::Result<()> {
        Self::ensure_ready()?;

        let mut command = Command::new(&self.setuid_helper);

        command.arg("target/release/toolbox-helper");
        command.arg("--set-yama-scope");
        command.arg(scope.as_str());

        let mut child = command.spawn()?;
        let status = child.wait()?;

        if !status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "child returned an error",
            ));
        }

        Ok(())
    }
}
