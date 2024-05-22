/*

    Author: Justin Kosten
    Description: This file contains the model for the Message object. This object is used to represent a message in the database.

*/

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Client {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub phone: String,
}
