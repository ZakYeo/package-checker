use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use dotenv::dotenv;
use reqwest::Error;
use serde_json::Value;

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

struct PackageJsonHandler {
    parsed_package_json: PackageJson,
    api_key: String
}

impl PackageJsonHandler {
    pub fn new(file_location: String) -> PackageJsonHandler {
        dotenv().ok();

        let api_key = env::var("API_KEY").expect("API_KEY must be set");


        let contents = fs::read_to_string(file_location)
                .expect("Should have been able to read the file");

        let parsed = match serde_json::from_str::<PackageJson>(&contents) {
            Ok(pkg) => pkg,
            Err(e) => {
                panic!("Unable to parse package.json: {:?}", e);
            }
        };
        PackageJsonHandler{
            parsed_package_json: parsed,
            api_key
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

    async fn get_npm_package_info(&self, package_name: String) -> Result<NpmPackage, Error> {

        let encoded_package_name = urlencoding::encode(&package_name);

        let url = format!(
                "https://libraries.io/api/npm/{}?api_key={}",
                encoded_package_name, self.api_key
        );
        let response = reqwest::get(&url).await?;
        let package: NpmPackage = response.json().await?;
        Ok(package)
    }

}

#[tokio::main]
async fn main() {
    let file_location = "./package.json"; 


    let pkg_handler = PackageJsonHandler::new(file_location.to_string());
    let package_name = "@notifee/react-native";

    match pkg_handler.get_npm_package_info("@notifee/react-native".to_string()).await {
        Ok(package) => {
            println!("Package: {:?}", package);
        }
        Err(e) => {
            println!("Error fetching package info: {:?}", e);
        }
    }

}

#[derive(Deserialize, Debug)]
struct NpmPackage {
    name: String,
    description: Option<String>,
    repository_url: Option<String>,
    homepage: Option<String>,
    #[serde(rename = "type")]
    package_type: Option<String>,
    contributions_count: u32,
    dependent_repos_count: u32,
    dependents_count: u32,
    deprecation_reason: Option<String>,
    forks: u32,
    latest_download_url: Option<String>,
    latest_release_number: Option<String>,
    latest_release_published_at: Option<String>,
    latest_stable_release_number: Option<String>,
    latest_stable_release_published_at: Option<String>,
    license_normalized: bool,
    licenses: Option<String>,
    normalized_licenses: Option<Vec<String>>,
    package_manager_url: Option<String>,
    platform: String,
    rank: u32,
    repository_license: Option<String>,
    repository_status: Option<String>,
    stars: u32,
    status: Option<String>,
    //versions: Option<Vec<Version>>,
}

#[derive(Deserialize, Debug)]
struct Version {
    number: String,
    //original_license: Option<String>,
    published_at: Option<String>,
    //repository_sources: Option<Vec<String>>,
    //researched_at: Option<String>,
    //spdx_expression: Option<String>,
}
