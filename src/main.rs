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
            let mut package_urls = Vec::new();
            let mut package_names = Vec::new();

            let response = request::get_package(package).await?;
            let version = response.dist_tags.get("latest").unwrap();
            let package_version = response.versions.get(version).unwrap();
            package_urls.push(package_version.dist.tarball.to_string());
            package_names.push(response.name);

            match package_version.dependencies.as_ref() {
                Some(dependencies) => {
                    if !dependencies.is_empty() {
                        for dependency in dependencies {
                            let response = request::get_package(dependency.0).await?;

                            let mut fixed_version = dependency.1.clone();

                            if fixed_version.contains('^') {
                                fixed_version = fixed_version.replace('^', "");
                            }

                            if fixed_version.contains('~') {
                                fixed_version = fixed_version.replace('~', "");
                            }

                            if fixed_version.contains('*') {
                                let version = response
                                    .versions
                                    .get(response.dist_tags.get("latest").unwrap());
                                let package_url = &version.unwrap().dist.tarball;

                                package_urls.push(package_url.to_string());
                                package_names.push(response.name.clone());

                                break;
                            }

                            let version = response.versions.get(&fixed_version);
                            let package_url = &version.unwrap().dist.tarball;

                            package_urls.push(package_url.to_string());
                            package_names.push(response.name);
                        }
                    }
                }
                None => {}
            }

            match package_version.dev_dependencies.as_ref() {
                Some(dev_dependencies) => {
                    if !dev_dependencies.is_empty() {
                        for dependency in dev_dependencies {
                            let response = request::get_package(dependency.0).await?;

                            let mut fixed_version = dependency.1.clone();

                            if fixed_version.contains('^') {
                                fixed_version = fixed_version.replace('^', "");
                            }

                            if fixed_version.contains('~') {
                                fixed_version = fixed_version.replace('~', "");
                            }

                            if fixed_version.contains('*') {
                                let version = response
                                    .versions
                                    .get(response.dist_tags.get("latest").unwrap());
                                let package_url = &version.unwrap().dist.tarball;

                                package_urls.push(package_url.to_string());
                                package_names.push(response.name.clone());

                                break;
                            }

                            let version = response.versions.get(&fixed_version);
                            let package_url = &version.unwrap().dist.tarball;

                            package_urls.push(package_url.to_string());
                            package_names.push(response.name);
                        }
                    }
                }
                None => {}
            }

            let mut package_json = fs::PackageJson::read_package_json_saveable()?;

            package_json.add_dependency(
                package_version.name.clone(),
                package_version.version.clone(),
            )?;

            fs::PackageJson::save_package_json(package_json)?;

            for i in 0..package_names.len() {
                println!("{}: {}", package_names[i], package_urls[i]);
                let _ =
                    request::download_package(package_names[i].as_str(), package_urls[i].as_str())
                        .await;
            }
        }
        Some(Commands::Install {}) => {
            let package_json = fs::PackageJson::read_package_json_cleaned()?;

            for key in package_json.dependencies.unwrap().keys() {
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
