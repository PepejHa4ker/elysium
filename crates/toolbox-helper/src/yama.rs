use std::str::FromStr;
use std::{fs, io};

const YAMA_PTRACE: &str = "/proc/sys/kernel/yama/ptrace_scope";

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

pub struct Yama {}

impl Yama {
    pub fn new() -> io::Result<Self> {
        fs::metadata(YAMA_PTRACE)?;

        Ok(Yama {})
    }

    pub fn set_scope(&self, scope: PtraceScope) -> io::Result<()> {
        fs::write(YAMA_PTRACE, scope.as_sys_str().as_bytes())
    }

    pub fn get_scope(&self) -> io::Result<PtraceScope> {
        use PtraceScope::*;

        let bytes = fs::read(YAMA_PTRACE)?;

        let scope = match bytes.get(0) {
            Some(b'0') => Default,
            Some(b'1') => Restrict,
            Some(b'2') => Admin,
            Some(b'3') => Disabled,
            _ => return Err(io::Error::new(io::ErrorKind::Other, "invalid value")),
        };

        Ok(scope)
    }
}
