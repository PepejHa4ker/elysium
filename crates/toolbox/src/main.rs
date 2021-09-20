#![allow(dead_code)]
#![feature(array_methods)]

use crate::helper::{Helper, PtraceScope};
use crate::options::Options;
use crate::steam::Steam;
use tokio::runtime::Builder;

mod helper;
mod options;
mod steam;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = Error> = std::result::Result<T, E>;

async fn entry() -> Result<()> {
    let options = Options::parse();
    let helper = Helper::new(options.setuid_helper);
    let steam = Steam::new(options.use_apulse);

    helper.set_yama_scope(PtraceScope::Restrict)?;
    steam.run_game(730)?;

    Ok(())
}

fn main() -> Result<()> {
    Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(entry())?;

    Ok(())
}
