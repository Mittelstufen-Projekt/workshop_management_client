/*

    Author: Jasha, Justin
    Description: This is the main file for the workshop management client. It is the entry point for the application.

*/

mod utils;
mod models;

use std::sync::{Arc, Mutex};

use crate::utils::keycloak_service::Keycloak;
use crate::utils::error_dialog::show_error;
use crate::utils::workshop_service::WorkshopService;
use crate::models::project_model::ProjectModel;
use crate::models::material_model::Material;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = WorkshopClient::new()?;

    // Need to use the services as mutex arcs so that we can move and still edit the memory
    let arc_workshop_service: Arc<Mutex<WorkshopService>> = Arc::new(Mutex::new(WorkshopService::new()));
    let arc_keycloak: Arc<Mutex<Keycloak>> = Arc::new(Mutex::new(Keycloak::new()));

    ui.set_loginView(true);
    ui.set_projectView(false);
    ui.set_projectManagementView(false);
    ui.set_projectDetailView(false);

    // Login action
    ui.on_request_login({
        let ui_handle = ui.as_weak();
        let keycloak_handle = arc_keycloak.clone();
        move || {
            let ui = ui_handle.unwrap();
            let user = ui.get_username();
            let password = ui.get_password();

            if user.is_empty() || password.is_empty() {
                show_error("Error".to_string(), "Please enter a username and password".to_string());
                return;
            }

            let token = Keycloak::login_user(&user, &password);

            match token {
                Ok(token) => {
                    keycloak_handle.lock().unwrap().set_token(token);
                    ui.set_loginView(false);
                    ui.set_projectView(true);
                }
                Err(_) => {
                    show_error("Login Failed".to_string(), "Invalid username or password".to_string());
                }
            }
        }
    });

    // Logout action
    ui.on_request_logout({
        let ui_handle = ui.as_weak();
        let keycloak_handle = arc_keycloak.clone();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_loginView(true);
            ui.set_projectView(false);
            ui.set_projectManagementView(false);
            ui.set_projectDetailView(false);
            keycloak_handle.lock().unwrap().clear();
        }
    });

    ui.run()
}
