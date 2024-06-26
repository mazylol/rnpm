use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::Write,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub main: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
    pub keywords: Option<Vec<String>>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
}

impl Default for PackageJson {
    fn default() -> Self {
        Self {
            name: Some(String::from("")),
            version: Some(String::from("1.0.0")),
            description: Some(String::from("")),
            main: Some(String::from("index.js")),
            scripts: Some(HashMap::new()),
            keywords: Some(Vec::new()),
            author: Some(String::from("")),
            license: Some(String::from("ISC")),
            dependencies: Some(HashMap::new()),
            dev_dependencies: Some(HashMap::new()),
        }
    }
}

impl PackageJson {
    pub fn create_package_json(mut name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::create_new("package.json").expect("Failed to create package.json");

        let package_json = PackageJson {
            name: if name.is_none() {
                name = Some(
                    env::current_dir()
                        .expect("Failed to get current directory")
                        .file_name()
                        .expect("Failed to get file name")
                        .to_str()
                        .expect("Failed to convert to string")
                        .to_string(),
                );
                name.clone()
            } else {
                name.clone()
            },
            ..Default::default()
        };

        let package_json_str =
            serde_json::to_string_pretty(&package_json).expect("Failed to serialize package.json");

        file.write_all(package_json_str.as_bytes())
            .expect("Failed to write package.json");

        Ok(())
    }

    pub fn read_package_json_saveable() -> Result<PackageJson, Box<dyn std::error::Error>> {
        let package_json_contents =
            fs::read_to_string("package.json").expect("Failed to read package.json");

        let package_json: PackageJson = serde_json::from_str(&package_json_contents)
            .expect("Failed to deserialize package.json");

        Ok(package_json)
    }

    #[allow(dead_code)]
    pub fn read_package_json_cleaned() -> Result<PackageJson, Box<dyn std::error::Error>> {
        let package_json_contents =
            fs::read_to_string("package.json").expect("Failed to read package.json");

        let mut package_json: PackageJson = serde_json::from_str(&package_json_contents)
            .expect("Failed to deserialize package.json");

        for key in package_json.clone().dependencies.unwrap().keys() {
            let val = String::from(package_json.dependencies.clone().unwrap().get(key).unwrap());

            if val.contains('^') {
                package_json
                    .dependencies
                    .as_mut()
                    .unwrap()
                    .insert(key.to_string(), val.replace('^', ""));
            }

            if val.contains('~') {
                package_json
                    .clone()
                    .dependencies
                    .as_mut()
                    .unwrap()
                    .insert(key.to_string(), val.replace('~', ""));
            }
        }

        Ok(package_json)
    }

    pub fn save_package_json(package: PackageJson) -> Result<(), Box<dyn std::error::Error>> {
        let package_json_str =
            serde_json::to_string_pretty(&package).expect("Failed to serialize package.json");

        std::fs::remove_file("package.json")?;

        let mut file = File::create_new("package.json").expect("Failed to create package.json");

        file.write_all(package_json_str.as_bytes())
            .expect("Failed to write package.json");

        Ok(())
    }

    pub fn add_dependency(
        &mut self,
        name: String,
        version: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.dependencies.as_mut().unwrap().insert(name, version);

        Ok(())
    }
}
