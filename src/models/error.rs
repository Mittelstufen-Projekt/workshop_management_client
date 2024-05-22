/*

    Author: Justin Kosten
    Description: This file contains the model for the Error object. This object is used to represent an error in the actix-web server.

*/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    pub code: i32,
}

impl Error {
    pub fn new(message: String, code: i32) -> Error {
        Error {
            message,
            code
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

impl Default for Error {
    fn default() -> Error {
        Error {
            message: "An error occurred".to_string(),
            code: 500
        }
    }
}