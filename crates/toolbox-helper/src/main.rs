#![allow(dead_code)]
#![feature(array_methods)]

use crate::options::Options;
use crate::yama::{PtraceScope, Yama};

mod options;
mod yama;

fn main() {
    let _options = Options::parse();

    if unsafe { libc::geteuid() } != 0 {
        println!("Root is required to run this command.");

        return;
    }

    let yama = match Yama::new() {
        Ok(yama) => yama,
        Err(_) => {
            println!("Yama security module is missing.");
            println!(
                "Either your kernel has no support, you didn't enable it, or you need to load it."
            );

            return;
        }
    };

    let scope = yama.get_scope().unwrap();

    if scope != PtraceScope::Restrict {
        println!("Setting Yama's ptrace scope to restricted.");

        yama.set_scope(PtraceScope::Restrict).unwrap();
    }
}
