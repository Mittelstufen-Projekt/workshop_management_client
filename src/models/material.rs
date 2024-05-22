/*

    Author: Justin Kosten
    Description: This file contains the model for the Material object. This object is used to represent a material in the database.

*/

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Material {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub type_id: i32,
    pub amount: i32,
    pub costs: f32,
    pub threshold_value: i32,
}
