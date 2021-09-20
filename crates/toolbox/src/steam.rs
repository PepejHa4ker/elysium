use std::io;
use std::process::{Command, Stdio};

pub struct Steam {
    use_apulse: bool,
}

impl Steam {
    pub fn new(use_apulse: bool) -> Self {
        Self { use_apulse }
    }

    pub async fn run_game(&self, game_id: u64) -> io::Result<()> {
        let mut command = if self.use_apulse {
            let mut command = Command::new("apulse");

            command.arg("steam");
            command
        } else {
            Command::new("steam")
        };

        command.arg(format!("steam://rungameid/{}", game_id));

        if self.use_apulse {
            command.env("SDL_AUDIODRIVER", "alsa");
        }

        command.stderr(Stdio::null());
        command.stdin(Stdio::null());
        command.stdout(Stdio::null());

        let mut child = command.spawn()?;

        child.wait()?;

        Ok(())
    }
}
