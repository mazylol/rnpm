mod cli;
mod fs;
mod request;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::handle().unwrap();

    match &cli.command {
        Some(Commands::Init { name }) => {
            return fs::PackageJson::create_package_json(name.clone());
        }
        Some(Commands::Test {}) => {
            println!(
                "{:?}",
                fs::PackageJson::read_package_json_saveable().unwrap()
            );
        }
        Some(Commands::Add { package }) => {
            let response = request::get_package(package).await?;
            let version = response.dist_tags.get("latest").unwrap();
            let package_version = response.versions.get(version).unwrap();
            let url = package_version.dist.tarball.clone();

            let mut package_json = fs::PackageJson::read_package_json_saveable()?;

            package_json.add_dependency(
                package_version.name.clone(),
                package_version.version.clone(),
            )?;

            fs::PackageJson::save_package_json(package_json)?;

            let _ = request::download_package(package, &url).await;
        }
        Some(Commands::Install {}) => {
            let package_json = fs::PackageJson::read_package_json_cleaned()?;

            for key in package_json.dependencies.keys() {
                let response = request::get_package(key).await?;
                let version = response.dist_tags.get("latest").unwrap();
                let package_version = response.versions.get(version).unwrap();
                let url = package_version.dist.tarball.clone();

                let _ = request::download_package(key, &url).await;
            }
        }
        None => {}
    }

    Ok(())
}
