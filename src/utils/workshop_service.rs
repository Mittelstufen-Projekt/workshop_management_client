/*

    Author: Justin
    Description: This file contains the connection to the API and provides the necessary functions to interact with the workshop management system.

*/

use crate::models::material::Material;
use crate::models::project::Project;
use crate::models::project_material::ProjectMaterial;
use crate::models::material_type::MaterialType;
use crate::models::client::Client;
use crate::models::error::Error;

const API_URL: &str = "http://justinrauch.myftp.org:8580";

/*
    All the endpoints for the API are as follows:

    - `GET /Projects` - get all projects
    - `GET /Projects/{id}` - get project by id
    - `POST /Projects` - create project
    - `PUT /Projects/{id}` - update project
    - `DELETE /Projects/{id}` - delete project
    - `GET /Materials` - get all materials
    - `GET /Materials/{id}` - get material by id
    - `POST /Materials` - create material
    - `PUT /Materials/{id}` - update material
    - `DELETE /Materials/{id}` - delete material
    - `GET /MaterialTypes` - get all material types
    - `GET /MaterialTypes/{id}` - get material type by id
    - `POST /MaterialTypes` - create material type
    - `PUT /MaterialTypes/{id}` - update material type
    - `DELETE /MaterialTypes/{id}` - delete material type
    - `GET /ProjectMaterials` - get all project materials
    - `GET /ProjectMaterials/{id}` - get project material by id
    - `POST /ProjectMaterials` - create project material
    - `PUT /ProjectMaterials/{id}` - update project material
    - `DELETE /ProjectMaterials/{id}` - delete project material
    - `GET /Clients` - get all clients
    - `GET /Clients/{id}` - get client by id
    - `POST /Clients` - create client
    - `PUT /Clients/{id}` - update client
    - `DELETE /Clients/{id}` - delete client
*/

#[derive(Clone)]
pub struct WorkshopService {
    pub projects: Vec<Project>,
    pub materials: Vec<Material>,
    pub project_materials: Vec<ProjectMaterial>,
    pub material_types: Vec<MaterialType>,
    pub clients: Vec<Client>,
}

impl WorkshopService {
    pub fn new() -> WorkshopService {
        WorkshopService {
            projects: Vec::new(),
            materials: Vec::new(),
            project_materials: Vec::new(),
            material_types: Vec::new(),
            clients: Vec::new(),
        }
    }

    // Get all items endpoints
    #[tokio::main]
    pub async fn get_projects(&mut self, token: &str) -> Result<Vec<Project>, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/Projects", API_URL))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to get projects");

        if response.status().is_success() {
            self.projects = response.json().await.expect("Failed to parse projects");
            Ok(self.projects.clone())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn get_materials(&mut self, token: &str) -> Result<Vec<Material>, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/Materials", API_URL))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to get materials");

        if response.status().is_success() {
            self.materials = response.json().await.expect("Failed to parse materials");
            Ok(self.materials.clone())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn get_project_materials(&mut self, token: &str) -> Result<Vec<ProjectMaterial>, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/ProjectMaterials", API_URL))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to get project materials");

        if response.status().is_success() {
            self.project_materials = response.json().await.expect("Failed to parse project materials");
            Ok(self.project_materials.clone())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn get_material_types(&mut self, token: &str) -> Result<Vec<MaterialType>, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/MaterialTypes", API_URL))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to get material types");

        if response.status().is_success() {
            self.material_types = response.json().await.expect("Failed to parse material types");
            Ok(self.material_types.clone())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn get_clients(&mut self, token: &str) -> Result<Vec<Client>, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/Clients", API_URL))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to get clients");

        if response.status().is_success() {
            self.clients = response.json().await.expect("Failed to parse clients");
            Ok(self.clients.clone())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    // Get item by id endpoints
    #[tokio::main]
    pub async fn get_project_by_id(&self, id: i32, token: &str) -> Result<Project, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/Projects/{}", API_URL, id))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to get project by id");

        if response.status().is_success() {
            Ok(response.json().await.expect("Failed to parse project"))
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn get_material_by_id(&self, id: i32, token: &str) -> Result<Material, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/Materials/{}", API_URL, id))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to get material by id");

        if response.status().is_success() {
            Ok(response.json().await.expect("Failed to parse material"))
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn get_project_material_by_id(&self, id: i32, token: &str) -> Result<ProjectMaterial, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/ProjectMaterials/{}", API_URL, id))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to get project material by id");

        if response.status().is_success() {
            Ok(response.json().await.expect("Failed to parse project material"))
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn get_material_type_by_id(&self, id: i32, token: &str) -> Result<MaterialType, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/MaterialTypes/{}", API_URL, id))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to get material type by id");

        if response.status().is_success() {
            Ok(response.json().await.expect("Failed to parse material type"))
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn get_client_by_id(&self, id: i32, token: &str) -> Result<Client, Error> {
        let response = reqwest::Client::new()
            .get(&format!("{}/Clients/{}", API_URL, id))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to get client by id");

        if response.status().is_success() {
            Ok(response.json().await.expect("Failed to parse client"))
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    // Create item endpoints
    #[tokio::main]
    pub async fn create_project(&mut self, project: Project, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .post(&format!("{}/Projects", API_URL))
            .bearer_auth(token)
            .json(&project)
            .send()
            .await
            .expect("Failed to create project");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn create_material(&mut self, material: Material, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .post(&format!("{}/Materials", API_URL))
            .bearer_auth(token)
            .json(&material)
            .send()
            .await
            .expect("Failed to create material");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn create_project_material(&mut self, project_material: ProjectMaterial, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .post(&format!("{}/ProjectMaterials", API_URL))
            .bearer_auth(token)
            .json(&project_material)
            .send()
            .await
            .expect("Failed to create project material");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn create_client(&mut self, client: Client, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .post(&format!("{}/Clients", API_URL))
            .bearer_auth(token)
            .json(&client)
            .send()
            .await
            .expect("Failed to create client");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn create_material_type(&mut self, material_type: MaterialType, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .post(&format!("{}/MaterialTypes", API_URL))
            .bearer_auth(token)
            .json(&material_type)
            .send()
            .await
            .expect("Failed to create material type");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    // Update item endpoints
    #[tokio::main]
    pub async fn update_project(&mut self, project: Project, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .put(&format!("{}/Projects/{}", API_URL, project.id))
            .bearer_auth(token)
            .json(&project)
            .send()
            .await
            .expect("Failed to update project");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn update_material(&mut self, material: Material, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .put(&format!("{}/Materials/{}", API_URL, material.id))
            .bearer_auth(token)
            .json(&material)
            .send()
            .await
            .expect("Failed to update material");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn update_project_material(&mut self, project_material: ProjectMaterial, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .put(&format!("{}/ProjectMaterials/{}", API_URL, project_material.id))
            .bearer_auth(token)
            .json(&project_material)
            .send()
            .await
            .expect("Failed to update project material");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn update_client(&mut self, client: Client, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .put(&format!("{}/Clients/{}", API_URL, client.id))
            .bearer_auth(token)
            .json(&client)
            .send()
            .await
            .expect("Failed to update client");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    // Delete item endpoints
    #[tokio::main]
    pub async fn delete_project(&mut self, id: i32, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .delete(&format!("{}/Projects/{}", API_URL, id))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to delete project");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn delete_material(&mut self, id: i32, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .delete(&format!("{}/Materials/{}", API_URL, id))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to delete material");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn delete_project_material(&mut self, id: i32, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .delete(&format!("{}/ProjectMaterials/{}", API_URL, id))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to delete project material");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }

    #[tokio::main]
    pub async fn delete_client(&mut self, id: i32, token: &str) -> Result<(), Error> {
        let response = reqwest::Client::new()
            .delete(&format!("{}/Clients/{}", API_URL, id))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to delete client");

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(response.status().to_string(), 500))
        }
    }
}
