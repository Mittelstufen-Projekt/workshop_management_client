/*

    Author: Justin, Jasha
    Description: This file contains the model for the project. It is used to store the data of a project.

*/

use serde::{Serialize, Deserialize};

use super::material_model::Material;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub name: String,
    pub client: String,
    pub email: String,
    pub tel: String,
    pub project_start: String,
    pub project_deadline: String,
    pub calculated_costs: String,
    pub current_costs: String,
    pub mats: Vec<Material>,
}