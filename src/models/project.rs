/*

    Author: Justin Kosten
    Description: This file contains the model for the Project object. This object is used to represent a project in the database.

*/

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub client_id: i32,
    pub description: String,
    pub startpoint: String,
    pub endpoint: String,
    pub estimated_costs: f32,
    pub estimated_hours: f32,
    pub costs: f32,
}
