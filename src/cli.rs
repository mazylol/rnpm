use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn handle() -> Result<Self, Box<dyn std::error::Error>> {
        let cli = Cli::parse();

        Ok(cli)
    }
}

#[derive(Subcommand)]
pub enum Commands {
    Test {
        #[arg(short, long)]
        text: String,
    }
}