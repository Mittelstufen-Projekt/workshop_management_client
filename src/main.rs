/*

    Author: Jasha, Justin
    Description: This is the main file for the workshop management client. It is the entry point for the application.

*/

mod models;
mod utils;

use std::sync::{Arc, Mutex};

use crate::utils::keycloak_service::Keycloak;
use crate::utils::workshop_service::WorkshopService;
// Need to import out models as alias to avoid conflicts with the slint models
use crate::models::material::Material as r_Material;
use crate::models::project::Project as r_Project;
use crate::models::project_material::ProjectMaterial as r_ProjectMaterial;
use crate::models::material_type::MaterialType as r_MaterialType;
use crate::models::client::Client as r_Client;
use crate::models::error::Error;

// Import the slint modules
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // Make the window fullscreen
    // DISABLE ON MACOS (crashes for some reason)
    std::env::set_var("SLINT_FULLSCREEN", "1");

    let ui = WorkshopClient::new()?;

    // Need to use the services as mutex arcs so that we can move and still edit the memory
    let arc_workshop_service: Arc<Mutex<WorkshopService>> =
        Arc::new(Mutex::new(WorkshopService::new()));
    let arc_keycloak: Arc<Mutex<Keycloak>> = Arc::new(Mutex::new(Keycloak::new()));

    // Set the inital state of the window
    ui.set_loginView(true);
    ui.set_projectView(false);
    ui.set_projectManagementView(false);
    ui.set_projectDetailView(false);
    ui.set_lagerOverviewView(false);

    // Login action
    ui.global::<Backend>().on_request_login({
        // Get the handlers that we need to manipulate the UI and Keycloak
        let ui_handle = ui.as_weak();
        let keycloak_handle = arc_keycloak.clone();
        move || {
            let ui = ui_handle.unwrap();
            let user = ui.get_username();
            let password = ui.get_password();

            // Check if the user has entered a username and password
            if user.is_empty() || password.is_empty() {
                ui.set_login_error("Please enter a username and password.".into());
                return;
            }

            // Attempt to login the user and get a token in return
            let token = keycloak_handle.lock().unwrap().login_user(&user, &password);

            // Check if the token was successfully retrieved otherwise handle the error
            match token {
                Err(e) => {
                    ui.set_login_error(e.to_string().into());
                }
                Ok(token) => {
                    keycloak_handle.lock().unwrap().set_token(token);
                    keycloak_handle
                        .lock()
                        .unwrap()
                        .set_username(user.to_string());
                    keycloak_handle
                        .lock()
                        .unwrap()
                        .set_password(password.to_string());
                    ui.set_loginView(false);
                    ui.set_projectView(true);
                    ui.set_login_error("".into());
                }
            }
        }
    });

    // Logout action
    ui.global::<Backend>().on_request_logout({
        // Get the handlers that we need to manipulate the UI and Keycloak
        let ui_handle = ui.as_weak();
        let keycloak_handle = arc_keycloak.clone();
        move || {
            // Basically just route to the login view and clear the token
            let ui = ui_handle.unwrap();
            ui.set_loginView(true);
            ui.set_projectView(false);
            ui.set_projectManagementView(false);
            ui.set_projectDetailView(false);
            ui.set_username("".into());
            ui.set_password("".into());
            keycloak_handle.lock().unwrap().clear();
        }
    });

    // Exit application
    ui.global::<Backend>().on_request_exit({
        move || {
            std::process::exit(0);
        }
    });

    ui.run()
}
