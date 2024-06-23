use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use dotenv::dotenv;

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
        dotenv().ok();
        if !Self::verify_api_key(){
            panic!("Please specify API_KEY in your environment variables")
        }


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

    fn verify_api_key() -> bool{
for (key, value) in env::vars() {
        println!("{}: {}", key, value);
    }
        match env::var("API_KEY" ){
            Ok(_) =>  true,
            Err(_) =>  false
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

    pub fn pretty_dependencies(&self) {
        for (key, value) in self.dependencies().into_iter(){
            println!("{}: {}", key, value)
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

    pkg_handler.pretty_dependencies();

}
