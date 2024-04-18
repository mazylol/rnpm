use std::collections::HashMap;

use error_chain::error_chain;
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use tar::Archive;
use tempfile::Builder;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Signature {
    pub keyid: String,
    pub sig: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageVersionDist {
    pub shasum: String,
    pub tarball: String,
    pub integrity: Option<String>,
    pub signatures: Option<Vec<Signature>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageVersion {
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub dist: PackageVersionDist,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistryResponse {
    pub name: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, String>,
    pub versions: HashMap<String, PackageVersion>,
}

pub async fn get_package(
    package_name: &str,
) -> core::result::Result<RegistryResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = client
        .get(format!("https://registry.npmjs.org/{}", package_name))
        .send()
        .await?;

    if body.status().is_success() {
        let response = body.json::<RegistryResponse>().await?;
        Ok(response)
    } else {
        Err(format!("Request failed with status code: {}", body.status()).into())
    }
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub async fn download_package(name: &str, url: &str) -> Result<()> {
    let tarball = reqwest::get(url).await?.bytes().await?;
    let mut archive = Archive::new(GzDecoder::new(&tarball[..]));
    let temp_dir = Builder::new().prefix(&name.replace('/', "")).tempdir().unwrap();
    archive.unpack(temp_dir.path())?;

    std::fs::create_dir_all(format!("node_modules/{}", name))?;

    for entry in std::fs::read_dir(temp_dir.path())? {
        let entry = entry?;
        for entry in std::fs::read_dir(entry.path())? {
            // use fs_extra because filesystem boundaries
            let entry = entry?;
            fs_extra::copy_items(
                &vec![entry.path()],
                "node_modules/".to_string() + &name,
                &fs_extra::dir::CopyOptions::new(),
            )
            .unwrap();
        }
    }

    Ok(())
}
