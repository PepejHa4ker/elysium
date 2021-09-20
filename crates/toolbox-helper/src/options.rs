use crate::yama::PtraceScope;
use clap::Clap;

#[derive(Clap, Debug)]
pub struct Options {
    #[clap(long)]
    pub set_yama_scope: PtraceScope,
}

impl Options {
    pub fn parse() -> Self {
        <Self as Clap>::parse()
    }
}
