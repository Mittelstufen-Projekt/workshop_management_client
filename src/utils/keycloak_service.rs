/*

    Author: Justin
    Description: This file contains the API calls to keycloak for authentication and authorization.

*/

use std::time::{SystemTime, UNIX_EPOCH};

const API_URL: &str = "http://justinrauch.myftp.org:8480/";
const REALM: &str = "WMS";
const CLIENT_ID: &str = "workshop_client";
const CLIENT_SECRET: &str = "Ip7GUqM8mRuIHMcq3tOuuHCaejSwSk3S";

#[derive(Clone)]
pub struct Keycloak {
    token: String,
    username: String,
    password: String,
    login_timestamp: u64,
}

impl Keycloak {
    pub fn new() -> Keycloak {
        Keycloak {
            token: String::new(),
            username: String::new(),
            password: String::new(),
            login_timestamp: 0,
        }
    }

    #[tokio::main]
    pub async fn login_user(&mut self, username: &str, password: &str) -> Result<String, reqwest::Error> {
        // Send a POST request to the keycloak server to get a token
        let response = reqwest::Client::new()
            .post(&format!("{}/realms/{}/protocol/openid-connect/token", API_URL, REALM))
            .form(&[
                ("client_id", CLIENT_ID),
                ("client_secret", CLIENT_SECRET),
                ("username", username),
                ("password", password),
                ("grant_type", "password"),
            ])
            // Set a timeout of 5 seconds
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await?;
        // Check if the response is an error
        match response.error_for_status_ref().err() {
            Some(err) => Err(err),
            None => {
                let token: serde_json::Value = response.json().await?;
                self.login_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                Ok(token["access_token"].as_str().unwrap().to_string())
            }
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }

    pub fn clear(&mut self) {
        self.token.clear();
        self.username.clear();
        self.password.clear();
        self.login_timestamp = 0;
    }

    pub fn refresh_token(&mut self) {
        // Check if the token is empty or if the time since the last login is less than 5 minutes
        let time_since_login = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - self.login_timestamp;
        if self.token.is_empty() || time_since_login < 300 {
            return;
        }
        // Attempt to login the user again
        let token = self.login_user(&self.username.clone(), &self.password.clone());
        // Check if the token was successfully retrieved otherwise handle the error
        match token {
            Err(e) => {
                println!("Error: {}", e);
            }
            Ok(token) => {
                // Set the new token and update the login timestamp
                self.login_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
                self.set_token(token);
            }
        }
    }
}