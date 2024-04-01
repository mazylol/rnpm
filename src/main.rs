mod cli;
mod fs;

use cli::{Cli, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::handle().unwrap();

    match &cli.command {
        Some(Commands::Init { name }) => {
            return fs::create_package_json(name.clone());
        },
        Some(Commands::Test { text }) => {
            println!("{}", text);
        },
        None => {},
    }

    Ok(())
}
