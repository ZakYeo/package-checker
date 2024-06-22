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


#[derive(Serialize, Deserialize, Debug)]
struct Repository {
    #[serde(rename = "type")]
    repo_type: String,
    url: String
}

fn main() {
    let file_location = "./package.json"; 
    let contents = fs::read_to_string(file_location)
            .expect("Should have been able to read the file");

    let parsed: Result<PackageJson, _> = serde_json::from_str(&contents);
        match parsed {
            Ok(pkg) => println!("Parsed package.json: {:?}", pkg),
            Err(e) => println!("Failed to parse package.json: {}", e),
        }


}
