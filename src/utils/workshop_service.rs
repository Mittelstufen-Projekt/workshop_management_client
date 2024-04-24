/*

    Author: Justin
    Description: This file contains the connection to the API and provides the necessary functions to interact with the workshop management system.

*/

use crate::models::project_model::ProjectModel;
use crate::models::material_model::Material;

//TODO: Change this to the actual API URL
const API_URL: &str = "http://localhost:8000/api";

pub async fn get_projects() -> Result<Vec<ProjectModel>, reqwest::Error> {
    let response = reqwest::get(&format!("{}/projects", API_URL)).await?;
    let projects: Vec<ProjectModel> = response.json().await?;
    Ok(projects)
}

pub async fn get_project(id: i32) -> Result<ProjectModel, reqwest::Error> {
    let response = reqwest::get(&format!("{}/projects/{}", API_URL, id)).await?;
    let project: ProjectModel = response.json().await?;
    Ok(project)
}

pub async fn create_project(project: ProjectModel) -> Result<ProjectModel, reqwest::Error> {
    let response = reqwest::Client::new()
        .post(&format!("{}/projects", API_URL))
        .json(&project)
        .send()
        .await?;
    let project: ProjectModel = response.json().await?;
    Ok(project)
}

pub async fn update_project(id: i32, project: ProjectModel) -> Result<ProjectModel, reqwest::Error> {
    let response = reqwest::Client::new()
        .put(&format!("{}/projects/{}", API_URL, id))
        .json(&project)
        .send()
        .await?;
    let project: ProjectModel = response.json().await?;
    Ok(project)
}

pub async fn delete_project(id: i32) -> Result<(), reqwest::Error> {
    reqwest::Client::new()
        .delete(&format!("{}/projects/{}", API_URL, id))
        .send()
        .await?;
    Ok(())
}
