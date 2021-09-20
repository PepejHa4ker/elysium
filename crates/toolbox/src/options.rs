use clap::Clap;
use std::path::PathBuf;

#[derive(Clap, Debug)]
pub struct Options {
    /// Compile the cheat.
    #[clap(long, short)]
    pub build: bool,

    /// Compile the cheat in debug mode.
    #[clap(long, short)]
    pub build_debug: bool,

    /// Inject the cheat.
    #[clap(long, short)]
    pub load: bool,

    /// Unload the cheat.
    #[clap(long, short)]
    pub unload: bool,

    /// Setuid helper.
    #[clap(default_value = "sudo", long)]
    pub setuid_helper: PathBuf,

    /// Silence annoying missing libraries for ALSA users.
    #[clap(long)]
    pub use_apulse: bool,
}

impl Options {
    pub fn parse() -> Self {
        <Self as Clap>::parse()
    }
}
