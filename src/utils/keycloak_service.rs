/*

    Author: Justin
    Description: This file contains the API calls to keycloak for authentication and authorization.

*/

// TODO: Change the API_URL, REALM, CLIENT_ID, and CLIENT_SECRET to match your Keycloak configuration
const API_URL: &str = "http://localhost:8000/api";
const REALM: &str = "workshop";
const CLIENT_ID: &str = "workshop-client";
const CLIENT_SECRET: &str = "workshop";

pub async fn get_token(username: &str, password: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::Client::new()
        .post(&format!("{}/realms/{}/protocol/openid-connect/token", API_URL, REALM))
        .form(&[
            ("client_id", CLIENT_ID),
            ("client_secret", CLIENT_SECRET),
            ("username", username),
            ("password", password),
            ("grant_type", "password"),
        ])
        .send()
        .await?;
    let token: serde_json::Value = response.json().await?;
    Ok(token["access_token"].as_str().unwrap().to_string())
}