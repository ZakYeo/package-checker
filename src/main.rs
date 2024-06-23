use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct PackageJson {
    name: String,
    version: String,
    description: Option<String>,
    main: Option<String>,
    scripts: Option<HashMap<String, String>>,
    dependencies: Option<HashMap<String, String>>,
    dev_dependencies: Option<HashMap<String, String>>,
    keywords: Option<Vec<String>>,
    author: Option<String>,
    license: Option<String>,
    repository: Option<Repository>,
}

struct PackageJsonHandler {
    parsed_package_json: PackageJson 
}

impl PackageJsonHandler {
    pub fn new(file_location: String) -> PackageJsonHandler {
        let contents = fs::read_to_string(file_location)
                .expect("Should have been able to read the file");

        let parsed = match serde_json::from_str::<PackageJson>(&contents) {
            Ok(pkg) => pkg,
            Err(e) => {
                panic!("Unable to parse package.json: {:?}", e);
            }
        };
        PackageJsonHandler{
            parsed_package_json: parsed
        }
    }

    pub fn dependencies(&self) -> HashMap<String, String> {
        match &self.parsed_package_json.dependencies {
            None => {
                println!("No dependencies found in package.json");
                HashMap::new()
            },
            Some(deps) => deps.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Repository {
    #[serde(rename = "type")]
    repo_type: String,
    url: String
}

fn main() {
    let file_location = "./package.json"; 


    let pkg_handler = PackageJsonHandler::new(file_location.to_string());

    println!("{:?}", pkg_handler.dependencies());
}
