/*

    Author: Justin
    Description: This file contains the API calls to keycloak for authentication and authorization.

*/

// Import the necessary modules
use std::time::{SystemTime, UNIX_EPOCH};

// Import out own Error crate because its easier to handle errors
use crate::models::error::Error;

// Define the constants for the keycloak server
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
    pub async fn login_user(
        &mut self,
        username: &str,
        password: &str,
    ) -> Result<String, Error> {
        // Send a POST request to the keycloak server to get a token
        let response = reqwest::Client::new()
            .post(&format!(
                "{}/realms/{}/protocol/openid-connect/token",
                API_URL, REALM
            ))
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
            .await;
        let response = match response {
            Ok(response) => response,
            // If the request failed return an error
            Err(e) => {
                return Err(Error::new(e.to_string(), 500));
            }
        };
        // Check if the response is an error
        match response.error_for_status_ref().err() {
            Some(err) => {
                // Get the status code and message of the error and turn it into our own error class
                let status = err.status().unwrap().as_u16();
                let message = err.to_string();
                return Err(Error::new(message, status.into()));
            },
            None => {
                let token = response.json().await;
                let token: serde_json::Value = match token {
                    Ok(token) => token,
                    // If the token could not be parsed return an error
                    Err(e) => {
                        return Err(Error::new(e.to_string(), 500));
                    }
                };
                // Set the login timestamp and return the token
                self.login_timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let token = token["access_token"].as_str().unwrap().to_string();
                self.set_token(token.clone());
                Ok(token)
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

    pub fn refresh_token(&mut self) -> Result<String, Error> {
        // Check if the token is empty or if the time since the last login is less than 5 minutes
        let time_since_login = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - self.login_timestamp;
        // User never logged in
        if self.token.is_empty() {
            return Err(Error::new("Token not found".into(), 401));
        }
        // Old token still valid, no need to refresh
        if time_since_login < 300 {
            return Ok(self.token.clone());
        }
        // Attempt to login the user again
        let token = self.login_user(&self.username.clone(), &self.password.clone());
        // Check if the token was successfully retrieved otherwise handle the error
        match token {
            Err(e) => Err(e),
            Ok(token) => {
                // Set the new token and update the login timestamp
                self.login_timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                Ok(token)
            }
        }
    }
}
