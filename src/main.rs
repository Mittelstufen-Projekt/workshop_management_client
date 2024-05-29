/*

    Author: Jasha, Justin
    Description: This is the main file for the workshop management client. It is the entry point for the application.

*/

mod models;
mod utils;

use std::sync::{Arc, Mutex};

use models::client::Client as r_Client;
use models::material::Material as r_Material;
use models::material_type::MaterialType as r_MaterialType;
use models::project::Project as r_Project;
use models::project_material::ProjectMaterial as r_ProjectMaterial;

use slint::Model;
use slint::SharedString;

use crate::utils::keycloak_service::Keycloak;
use crate::utils::workshop_service::WorkshopService;

// Import the slint modules
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // Make the window fullscreen
    // DISABLE ON MACOS (crashes for some reason)
    // std::env::set_var("SLINT_FULLSCREEN", "1");

    let ui = WorkshopClient::new()?;

    // Need to use the services as mutex arcs so that we can move and still edit the memory
    let arc_workshop_service: Arc<Mutex<WorkshopService>> =
        Arc::new(Mutex::new(WorkshopService::new()));
    let arc_keycloak: Arc<Mutex<Keycloak>> = Arc::new(Mutex::new(Keycloak::new()));

    // Set the inital state of the window
    ui.global::<Backend>().set_loginView(true);
    ui.global::<Backend>().set_projectView(false);
    ui.global::<Backend>().set_projectManagementView(false);
    ui.global::<Backend>().set_projectDetailView(false);
    ui.global::<Backend>().set_lagerOverviewView(false);
    ui.global::<Backend>().set_showClientPopUp(false);
    ui.global::<Backend>().set_showMaterialPopUp(false);
    ui.global::<Backend>().set_showMaterialTypePopUp(false);

    // Login action
    ui.global::<Backend>().on_request_login({
        // Get the handlers that we need to manipulate the UI and Keycloak
        let ui_handle = ui.as_weak();
        let keycloak_handle = arc_keycloak.clone();
        move || {
            let ui = ui_handle.unwrap();
            let user = ui.global::<Backend>().get_username();
            let password = ui.global::<Backend>().get_password();

            // Check if the user has entered a username and password
            if user.is_empty() || password.is_empty() {
                ui.global::<Backend>()
                    .set_login_error("Please enter a username and password.".into());
                return;
            }

            // Attempt to login the user and get a token in return
            let token = keycloak_handle.lock().unwrap().login_user(&user, &password);

            // Check if the token was successfully retrieved otherwise handle the error
            match token {
                Err(e) => {
                    ui.global::<Backend>().set_login_error(e.to_string().into());
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
                    ui.global::<Backend>().invoke_route_to_project_management();
                    ui.global::<Backend>().set_login_error("".into());
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
            ui.global::<Backend>().set_loginView(true);
            ui.global::<Backend>().set_projectView(false);
            ui.global::<Backend>().set_username("".into());
            ui.global::<Backend>().set_password("".into());
            keycloak_handle.lock().unwrap().clear();
        }
    });

    /*
        Raw routing functions
    */
    // Route to project view
    ui.global::<Backend>().on_route_to_project_management({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            // TODO: Set vars
            ui.global::<Backend>().set_projectView(false);
            ui.global::<Backend>().set_projectManagementView(true);
            ui.global::<Backend>().set_projectDetailView(false);
            ui.global::<Backend>().set_lagerOverviewView(false);
            ui.global::<Backend>().set_showClientPopUp(false);
            ui.global::<Backend>().set_showMaterialPopUp(false);
            ui.global::<Backend>().set_showMaterialTypePopUp(false);
        }
    });

    // Route to project detail view
    ui.global::<Backend>().on_route_to_project_view({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            // TODO: Set vars
            // in-out property <[ProjectModel]> recentProjects;
            // in-out property <[Material]> materialAlerts;
            ui.global::<Backend>().set_loginView(false);
            ui.global::<Backend>().set_projectView(false);
            ui.global::<Backend>().set_projectManagementView(false);
            ui.global::<Backend>().set_projectDetailView(true);
            ui.global::<Backend>().set_lagerOverviewView(false);
            ui.global::<Backend>().set_showClientPopUp(false);
            ui.global::<Backend>().set_showMaterialPopUp(false);
            ui.global::<Backend>().set_showMaterialTypePopUp(false);
        }
    });

    // Route to lager overview
    ui.global::<Backend>().on_route_to_warehouse_management({
        let ui_handle = ui.as_weak();
        let workshop_handle = arc_workshop_service.clone();
        let keycloak_handle = arc_keycloak.clone();
        move || {
            let ui = ui_handle.unwrap();
            // TODO: Set vars
            // in-out property <[Material]> materials;
            ui.global::<Backend>().set_projectView(false);
            ui.global::<Backend>().set_projectManagementView(false);
            ui.global::<Backend>().set_projectDetailView(false);
            ui.global::<Backend>().set_lagerOverviewView(true);
            ui.global::<Backend>().set_showClientPopUp(false);
            ui.global::<Backend>().set_showMaterialPopUp(false);
            ui.global::<Backend>().set_showMaterialTypePopUp(false);
            // Refresh the token
            let token = keycloak_handle.lock().unwrap().refresh_token();
            let token = match token {
                Ok(token) => token,
                Err(_) => return,
            };
            // Get the materials
            workshop_handle.lock().unwrap().get_materials(&token);
            let materials: Vec<r_Material> = workshop_handle.lock().unwrap().materials.clone();
            let materials: Vec<Material> = materials
                .iter()
                .map(|m| Material {
                    id: m.id,
                    name: m.name.clone().into(),
                    m_type: m.material_type.clone(),
                    description: m.description.clone().into(),
                    price: m.costs,
                    quantity: m.amount,
                    threshold_value: m.threshold_value,
                })
                .collect();
        }
    });

    // Route to project detail view
    ui.global::<Backend>().on_route_to_project_i({
        let ui_handle = ui.as_weak();
        let workshop_handle = arc_workshop_service.clone();
        let keycloak_handle = arc_keycloak.clone();
        move |project_id: i32| {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_projectView(false);
            ui.global::<Backend>().set_projectManagementView(false);
            ui.global::<Backend>().set_projectDetailView(true);
            ui.global::<Backend>().set_lagerOverviewView(false);
            ui.global::<Backend>().set_showClientPopUp(false);
            ui.global::<Backend>().set_showMaterialPopUp(false);
            ui.global::<Backend>().set_showMaterialTypePopUp(false);
            // Refresh the token
            let token = keycloak_handle.lock().unwrap().refresh_token();
            // Check if the token was successfully retrieved otherwise handle the error
            let token = match token {
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
                Ok(token) => token,
            };
            let project = workshop_handle
                .lock()
                .unwrap()
                .get_project_by_id(project_id, &token);
            // Check if the project was successfully retrieved otherwise handle the error
            let project = match project {
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
                Ok(project) => project,
            };
            todo!("Set project details");
        }
    });

    ui.global::<Backend>().on_showAddNewClientPopUp({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_showClientPopUp(true);
        }
    });

    ui.global::<Backend>().on_showAddNewMaterialPopUp({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_showMaterialPopUp(true);
        }
    });

    ui.global::<Backend>().on_showAddNewMaterialTypePopUp({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_showMaterialTypePopUp(true);
        }
    });

    ui.global::<Backend>().on_showAddNewClientPopUp({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_showClientPopUp(true);
        }
    });

    ui.global::<Backend>().on_create_new_project({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_projectView(false);
            ui.global::<Backend>().set_projectManagementView(false);
            ui.global::<Backend>().set_projectDetailView(true);
            ui.global::<Backend>().set_lagerOverviewView(false);
            ui.global::<Backend>().set_showClientPopUp(false);
            ui.global::<Backend>().set_showMaterialPopUp(false);
            ui.global::<Backend>().set_showMaterialTypePopUp(false);
        }
    });

    ui.global::<Backend>().on_save_material_type({
        let ui_handle = ui.as_weak();
        let workshop_handle = arc_workshop_service.clone();
        let keycloak_handle = arc_keycloak.clone();
        move |name: SharedString, description: SharedString| {
            let ui = ui_handle.unwrap();
            // Refresh the token
            let token = keycloak_handle.lock().unwrap().refresh_token();
            // Check if the token was successfully retrieved otherwise handle the error
            let token = match token {
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
                Ok(token) => token,
            };
            let material_type: MaterialType = MaterialType {
                id: 0, // We can ignore it because the database will set it
                name: name.to_string(),
                description: description.to_string(),
            };
            let result = workshop_handle
                .lock()
                .unwrap()
                .create_material_type(&material_type, &token);
            // Check if the material type was successfully created otherwise handle the error
            match result {
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
                Ok(_) => {
                    ui.global::<Backend>().set_showMaterialTypePopUp(false);
                    // Refresh the token
                    let token = keycloak_handle.lock().unwrap().refresh_token();
                    // Check if the token was successfully retrieved otherwise handle the error
                    let token = match token {
                        Err(e) => {
                            todo!("Set UI error message");
                            return;
                        }
                        Ok(token) => token,
                    };
                    let material_types = workshop_handle.lock().unwrap().get_material_types(&token);
                    // Check if the material types were successfully retrieved otherwise handle the error
                    let material_types = match material_types {
                        Err(e) => {
                            todo!("Set UI error message");
                            return;
                        }
                        Ok(material_types) => material_types,
                    };
                    todo!("Set material types");
                }
            }
        }
    });

    ui.global::<Backend>().on_search_material({
        let ui_handle = ui.as_weak();
        move |search: SharedString| {
            let ui = ui_handle.unwrap();
            let materials = ui.global::<Backend>().get_materials();
            materials.filter(|m| {
                m.name.to_lowercase().contains(&search.to_lowercase()) || m.description.to_lowercase().contains(&search.to_lowercase())
            });
            ui.global::<Backend>().set_materials(materials);
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
