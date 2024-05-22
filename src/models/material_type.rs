/*

    Author: Justin Kosten
    Description: This file contains the model for the Project object. This object is used to represent a project in the database.

*/

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MaterialType {
    pub id: i32,
    pub name: String,
    pub description: String,
}
