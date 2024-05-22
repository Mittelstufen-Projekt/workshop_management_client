/*

    Author: Justin Kosten
    Description: This file contains the model for the Project object. This object is used to represent a project in the database.

*/

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectMaterial {
    pub id: i32,
    pub project_id: i32,
    pub material_id: i32,
    pub amount: i32,
}
