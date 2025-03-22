use std::path::PathBuf;

use clap::ArgAction::Append;
use clap::Parser;

#[derive(Parser)]
#[command(version, name = "ssh-remote-exec", bin_name = "ssh-remote-exec")]
pub struct Cli {
    #[arg(long, short = 'H', help = "Required - Hosts", required = true, action = Append)]
    pub hosts: Vec<String>,

    #[arg(long, short = 'U', help = "Required - Username", required = true)]
    pub username: String,

    #[arg(long, short = 'P', help = "Required - Public key", required = true)]
    pub public_key: PathBuf,

    #[arg(long, short = 'K', help = "Required - Private key", required = true)]
    pub private_key: PathBuf,
}

impl Cli {
    /// Short command to avoid importing clap in other modules
    pub fn load() -> Self {
        Cli::parse()
    }
}
