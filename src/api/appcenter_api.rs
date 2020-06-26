extern crate reqwest;

use super::api_trait::API;
use reqwest::Client;
use std::io::Read;

static API_VERSION: &str = "v0.1";

pub struct AppCenter {
    pub token: String,
    client: Client,
}

impl API for AppCenter {
    fn new(token: String) -> Self {
        AppCenter {
            token,
            client: Client::new(),
        }
    }

    fn latest_version(
        &self,
        organization: String,
        application: String,
    ) -> Result<String, &'static str> {
        let url = format!(
            "https://api.appcenter.ms/{}/apps/{}/{}/recent_releases",
            API_VERSION, organization, application
        );
        let response = self
            .client
            .get(&url)
            .header("X-API-Token", self.token.clone())
            .header("accept", "application/json")
            .send();

        match response {
            Ok(mut response) => {
                let mut json = String::new();
                match response.read_to_string(&mut json) {
                    Ok(_j) => Ok(json),
                    Err(_e) => Err("Failed to read response from API"),
                }
            }
            Err(_e) => Err("Failed to fetch latest versions json"),
        }
    }

    fn crashes_json(
        &self,
        organization: String,
        application: String,
        version: String,
        start_date: String,
    ) -> Result<String, &'static str> {
        let url = format!("https://api.appcenter.ms/{}/apps/{}/{}/errors/errorGroups?start={}&%24version={}&%24orderby=count%20desc&%24top=20", API_VERSION, organization, application, start_date, version);
        let response = self
            .client
            .get(&url)
            .header("X-API-Token", self.token.clone())
            .header("accept", "application/json")
            .send();
        match response {
            Ok(mut response) => {
                let mut json = String::new();
                match response.read_to_string(&mut json) {
                    Ok(_j) => Ok(json),
                    Err(_e) => Err("Failed to read response from API"),
                }
            }
            Err(_e) => Err("Failed to fetch error groups json"),
        }
    }
}
