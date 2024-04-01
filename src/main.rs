mod cli;

use cli::{Cli, Commands};

fn main() {
    let cli = Cli::handle().unwrap();

    match &cli.command {
        Some(Commands::Test { text }) => {
            println!("{}", text);
        },
        None => {},
    }
}
