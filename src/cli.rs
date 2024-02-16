use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets a config file
    #[arg(short, long, value_name = "FILE")]
    pub config: PathBuf,

    /// Sets a ngrok config file
    #[arg(short, long, value_name = "FILE")]
    pub ngrok_config: PathBuf,
}
