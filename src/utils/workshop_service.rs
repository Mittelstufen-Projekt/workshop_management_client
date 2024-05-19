/*

    Author: Justin
    Description: This file contains the connection to the API and provides the necessary functions to interact with the workshop management system.

*/

use crate::models::material_model::Material;
use crate::models::project_model::Project;

//TODO: Change this to the actual API URL
const API_URL: &str = "http://justinrauch.myftp.org:8580/";

#[derive(Clone)]
pub struct WorkshopService {
    pub projects: Vec<Project>,
    pub materials: Vec<Material>,
}

impl WorkshopService {
    pub fn new() -> WorkshopService {
        WorkshopService {
            projects: Vec::new(),
            materials: Vec::new(),
        }
    }

    #[tokio::main]
    pub async fn get_projects(&mut self, token: &str) {
        let response = reqwest::Client::new()
            .get(&format!("{}/projects", API_URL))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to get projects");

        self.projects = response.json().await.expect("Failed to parse projects");
    }

    #[tokio::main]
    pub async fn get_materials(&mut self, token: &str) {
        let response = reqwest::Client::new()
            .get(&format!("{}/materials", API_URL))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to get materials");

        self.materials = response.json().await.expect("Failed to parse materials");
    }

    #[tokio::main]
    pub async fn add_project(&mut self, project: Project, token: &str) {
        let response = reqwest::Client::new()
            .post(&format!("{}/projects", API_URL))
            .bearer_auth(token)
            .json(&project)
            .send()
            .await
            .expect("Failed to add project");

        self.projects
            .push(response.json().await.expect("Failed to parse project"));
    }

    #[tokio::main]
    pub async fn add_material(&mut self, material: Material, token: &str) {
        let response = reqwest::Client::new()
            .post(&format!("{}/materials", API_URL))
            .bearer_auth(token)
            .json(&material)
            .send()
            .await
            .expect("Failed to add material");

        self.materials
            .push(response.json().await.expect("Failed to parse material"));
    }

    #[tokio::main]
    pub async fn update_project(&mut self, project: Project, token: &str) {
        let response = reqwest::Client::new()
            .put(&format!("{}/projects/{}", API_URL, project.id))
            .bearer_auth(token)
            .json(&project)
            .send()
            .await
            .expect("Failed to update project");

        self.projects
            .push(response.json().await.expect("Failed to parse project"));
    }

    #[tokio::main]
    pub async fn update_material(&mut self, material: Material, token: &str) {
        let response = reqwest::Client::new()
            .put(&format!("{}/materials/{}", API_URL, material.id))
            .bearer_auth(token)
            .json(&material)
            .send()
            .await
            .expect("Failed to update material");

        self.materials
            .push(response.json().await.expect("Failed to parse material"));
    }

    #[tokio::main]
    pub async fn delete_project(&mut self, project: Project, token: &str) {
        let response = reqwest::Client::new()
            .delete(&format!("{}/projects/{}", API_URL, project.id))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to delete project");

        self.projects
            .push(response.json().await.expect("Failed to parse project"));
    }

    #[tokio::main]
    pub async fn delete_material(&mut self, material: Material, token: &str) {
        let response = reqwest::Client::new()
            .delete(&format!("{}/materials/{}", API_URL, material.id))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to delete material");

        self.materials
            .push(response.json().await.expect("Failed to parse material"));
    }
}
