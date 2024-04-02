mod cli;
mod fs;

use cli::{Cli, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::handle().unwrap();

    match &cli.command {
        Some(Commands::Init { name }) => {
            return fs::PackageJson::create_package_json(name.clone());
        }
        Some(Commands::Test { text }) => {
            println!("{:?}", fs::PackageJson::read_package_json().unwrap());
        }
        None => {}
    }

    Ok(())
}
