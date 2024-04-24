/*

    Author: Justin
    Description: This file contains the connection to the API and provides the necessary functions to interact with the workshop management system.

*/

use crate::models::project_model::ProjectModel;
use crate::models::material_model::Material;

//TODO: Change this to the actual API URL
const API_URL: &str = "http://localhost:8000/api";

#[derive(Clone)]
pub struct WorkshopService {
    pub projects: Vec<ProjectModel>,
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
    pub async fn get_projects(&mut self) {
        let response = reqwest::Client::new()
            .get(&format!("{}/projects", API_URL))
            .send()
            .await
            .expect("Failed to get projects");

        self.projects = response.json().await.expect("Failed to parse projects");
    }

    #[tokio::main]
    pub async fn get_materials(&mut self) {
        let response = reqwest::Client::new()
            .get(&format!("{}/materials", API_URL))
            .send()
            .await
            .expect("Failed to get materials");

        self.materials = response.json().await.expect("Failed to parse materials");
    }

    #[tokio::main]
    pub async fn add_project(&mut self, project: ProjectModel) {
        let response = reqwest::Client::new()
            .post(&format!("{}/projects", API_URL))
            .json(&project)
            .send()
            .await
            .expect("Failed to add project");

        self.projects.push(response.json().await.expect("Failed to parse project"));
    }

    #[tokio::main]
    pub async fn add_material(&mut self, material: Material) {
        let response = reqwest::Client::new()
            .post(&format!("{}/materials", API_URL))
            .json(&material)
            .send()
            .await
            .expect("Failed to add material");

        self.materials.push(response.json().await.expect("Failed to parse material"));
    }
}