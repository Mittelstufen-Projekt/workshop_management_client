/*

    Author: Justin, Jasha
    Description: This file contains the model for the material. It is used to store the data of a material.

*/

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Material {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub quantity: i32,
}