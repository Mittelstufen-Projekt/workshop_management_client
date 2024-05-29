/*

    Author: Jasha, Justin
    Description: This is the main file for the workshop management client. It is the entry point for the application.

*/

mod models;
mod utils;

use std::sync::{Arc, Mutex};

use models::client::Client as r_Client;
use models::error::Error;
use models::material::Material as r_Material;
use models::material_type::MaterialType as r_MaterialType;
use models::project::Project as r_Project;
use models::project_material::ProjectMaterial as r_ProjectMaterial;

use slint::SharedString;
use slint::{ModelRc, VecModel};

use crate::utils::keycloak_service::Keycloak;
use crate::utils::workshop_service::WorkshopService;

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
                    ui.global::<Backend>().invoke_route_to_project_view();
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

    // Route to project view
    ui.global::<Backend>().on_route_to_project_management({
        let ui_handle = ui.as_weak();
        let keycloak_handle = arc_keycloak.clone();
        let workshop_handle = arc_workshop_service.clone();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_projectView(false);
            ui.global::<Backend>().set_projectManagementView(true);
            ui.global::<Backend>().set_projectDetailView(false);
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
            let projects = workshop_handle.lock().unwrap().get_projects(&token);
            // Check if the projects were successfully retrieved otherwise handle the error
            let projects = match projects {
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
                Ok(projects) => projects,
            };
            // Get the clients and materials for each project
            let project_materials = match workshop_handle
                .lock()
                .unwrap()
                .get_project_materials(&token)
            {
                Ok(project_materials) => project_materials,
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
            };
            let clients = match workshop_handle.lock().unwrap().get_clients(&token) {
                Ok(clients) => clients,
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
            };
            let materials = match workshop_handle.lock().unwrap().get_materials(&token) {
                Ok(materials) => materials,
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
            };
            let material_types = match workshop_handle.lock().unwrap().get_material_types(&token) {
                Ok(material_types) => material_types,
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
            };

            let mut slint_projects: Vec<ProjectModel> = projects
                .iter()
                .map(|p| {
                    // Get the client for the project
                    let client: Result<r_Client, Error> =
                        match clients.iter().find(|c| c.id == p.client_id) {
                            Some(client) => Ok(client.clone()),
                            None => Err(Error::new("Client not found".to_owned(), 404)),
                        };

                    let client: ClientModel = match client {
                        Ok(client) => ClientModel {
                            id: client.id,
                            firstName: client.firstname.clone().into(),
                            lastName: client.lastname.clone().into(),
                            phone: client.phone.clone().into(),
                        },
                        Err(e) => {
                            todo!("Set UI error message");
                        }
                    };

                    let project_materials = project_materials
                        .iter()
                        .filter(|m| m.project_id == p.id)
                        .collect::<Vec<&r_ProjectMaterial>>();

                    let materials: Vec<Material> = project_materials
                        .iter()
                        .map(|m| {
                            let material: Result<r_Material, Error> =
                                match materials.iter().find(|mat| mat.id == m.material_id) {
                                    Some(material) => Ok(material.clone()),
                                    None => Err(Error::new("Material not found".to_owned(), 404)),
                                };

                            let material = match material {
                                Ok(material) => {
                                    let m_type: Result<r_MaterialType, Error> = match material_types
                                        .iter()
                                        .find(|t| t.id == material.type_id)
                                    {
                                        Some(m_type) => Ok(m_type.clone()),
                                        None => Err(Error::new(
                                            "Material type not found".to_owned(),
                                            404,
                                        )),
                                    };

                                    let m_type = match m_type {
                                        Ok(m_type) => MaterialType {
                                            id: m_type.id,
                                            name: m_type.name.clone().into(),
                                            description: m_type.description.clone().into(),
                                        },
                                        Err(e) => {
                                            todo!("Set UI error message");
                                        }
                                    };

                                    Material {
                                        id: material.id,
                                        name: material.name.clone().into(),
                                        description: material.description.clone().into(),
                                        m_type,
                                        quantity: m.amount,
                                        price: material.costs,
                                        threshold_value: material.threshold_value,
                                    }
                                }
                                Err(_) => {
                                    todo!("Set UI error message");
                                }
                            };
                            material
                        })
                        .collect();

                    ProjectModel {
                        id: p.id,
                        name: p.name.clone().into(),
                        client,
                        mats: ModelRc::new(VecModel::from(materials)),
                        calculated_costs: p.estimated_costs,
                        current_costs: p.costs,
                        project_deadline: p.endpoint.clone().into(),
                        project_start: p.startpoint.clone().into(),
                    }
                })
                .collect();
            // Sort the projects by the deadline
            slint_projects.sort_by(|a, b| a.project_deadline.cmp(&b.project_deadline));

            ui.global::<Backend>()
                .set_allProjects(ModelRc::new(VecModel::from(slint_projects)));
        }
    });

    // Route to project detail view
    ui.global::<Backend>().on_route_to_project_view({
        let ui_handle = ui.as_weak();
        let workshop_handle = arc_workshop_service.clone();
        let keycloak_handle = arc_keycloak.clone();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_loginView(false);
            ui.global::<Backend>().set_projectView(true);
            ui.global::<Backend>().set_projectManagementView(false);
            ui.global::<Backend>().set_projectDetailView(false);
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
            let projects = workshop_handle.lock().unwrap().get_projects(&token);
            // Check if the projects were successfully retrieved otherwise handle the error
            let projects = match projects {
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
                Ok(projects) => projects,
            };
            // Get the clients and materials for each project
            let project_materials = match workshop_handle
                .lock()
                .unwrap()
                .get_project_materials(&token)
            {
                Ok(project_materials) => project_materials,
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
            };
            let clients = match workshop_handle.lock().unwrap().get_clients(&token) {
                Ok(clients) => clients,
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
            };
            let materials = match workshop_handle.lock().unwrap().get_materials(&token) {
                Ok(materials) => materials,
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
            };
            let material_types = match workshop_handle.lock().unwrap().get_material_types(&token) {
                Ok(material_types) => material_types,
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
            };

            let mut slint_projects: Vec<ProjectModel> = projects
                .iter()
                .map(|p| {
                    // Get the client for the project
                    let client: Result<r_Client, Error> =
                        match clients.iter().find(|c| c.id == p.client_id) {
                            Some(client) => Ok(client.clone()),
                            None => Err(Error::new("Client not found".to_owned(), 404)),
                        };

                    let client: ClientModel = match client {
                        Ok(client) => ClientModel {
                            id: client.id,
                            firstName: client.firstname.clone().into(),
                            lastName: client.lastname.clone().into(),
                            phone: client.phone.clone().into(),
                        },
                        Err(e) => {
                            todo!("Set UI error message");
                        }
                    };

                    let project_materials = project_materials
                        .iter()
                        .filter(|m| m.project_id == p.id)
                        .collect::<Vec<&r_ProjectMaterial>>();

                    let materials: Vec<Material> = project_materials
                        .iter()
                        .map(|m| {
                            let material: Result<r_Material, Error> =
                                match materials.iter().find(|mat| mat.id == m.material_id) {
                                    Some(material) => Ok(material.clone()),
                                    None => Err(Error::new("Material not found".to_owned(), 404)),
                                };

                            let material = match material {
                                Ok(material) => {
                                    let m_type: Result<r_MaterialType, Error> = match material_types
                                        .iter()
                                        .find(|t| t.id == material.type_id)
                                    {
                                        Some(m_type) => Ok(m_type.clone()),
                                        None => Err(Error::new(
                                            "Material type not found".to_owned(),
                                            404,
                                        )),
                                    };

                                    let m_type = match m_type {
                                        Ok(m_type) => MaterialType {
                                            id: m_type.id,
                                            name: m_type.name.clone().into(),
                                            description: m_type.description.clone().into(),
                                        },
                                        Err(e) => {
                                            todo!("Set UI error message");
                                        }
                                    };

                                    Material {
                                        id: material.id,
                                        name: material.name.clone().into(),
                                        description: material.description.clone().into(),
                                        m_type,
                                        quantity: m.amount,
                                        price: material.costs,
                                        threshold_value: material.threshold_value,
                                    }
                                }
                                Err(_) => {
                                    todo!("Set UI error message");
                                }
                            };
                            material
                        })
                        .collect();

                    ProjectModel {
                        id: p.id,
                        name: p.name.clone().into(),
                        client,
                        mats: ModelRc::new(VecModel::from(materials)),
                        calculated_costs: p.estimated_costs,
                        current_costs: p.costs,
                        project_deadline: p.endpoint.clone().into(),
                        project_start: p.startpoint.clone().into(),
                    }
                })
                .collect();
            // Sort the projects by the deadline
            slint_projects.sort_by(|a, b| a.project_deadline.cmp(&b.project_deadline));

            // Finnaly set the projects
            ui.global::<Backend>()
                .set_recentProjects(ModelRc::new(VecModel::from(slint_projects)));

            // Set the material alerts
            let mut material_alerts: Vec<Material> = vec![];

            for material in materials.iter() {
                // Check if the material is below the threshold
                if material.amount < material.threshold_value {
                    // Get the material type
                    let m_type: Result<r_MaterialType, Error> =
                        match material_types.iter().find(|t| t.id == material.type_id) {
                            Some(m_type) => Ok(m_type.clone()),
                            None => Err(Error::new("Material type not found".to_owned(), 404)),
                        };
                    let m_type = match m_type {
                        Ok(m_type) => MaterialType {
                            id: m_type.id,
                            name: m_type.name.clone().into(),
                            description: m_type.description.clone().into(),
                        },
                        Err(e) => {
                            todo!("Set UI error message");
                        }
                    };

                    // Add the material to the alerts
                    material_alerts.push(Material {
                        id: material.id,
                        name: material.name.clone().into(),
                        description: material.description.clone().into(),
                        m_type,
                        quantity: material.amount,
                        price: material.costs,
                        threshold_value: material.threshold_value,
                    });
                }
            }

            // Set the material alerts
            ui.global::<Backend>()
                .set_materialAlerts(ModelRc::new(VecModel::from(material_alerts)));
        }
    });

    // Route to lager overview
    ui.global::<Backend>().on_route_to_warehouse_management({
        let ui_handle = ui.as_weak();
        let workshop_handle = arc_workshop_service.clone();
        let keycloak_handle = arc_keycloak.clone();
        move || {
            let ui = ui_handle.unwrap();
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
                Err(_) => {
                    todo!("Set UI error message");
                    return;
                }
            };
            // Get the materials
            let materials: Result<Vec<r_Material>, Error> =
                workshop_handle.lock().unwrap().get_materials(&token);
            let materials: Vec<r_Material> = match materials {
                Ok(materials) => materials,
                Err(_) => {
                    todo!("Set UI error message");
                    return;
                }
            };
            let slint_materials: Vec<Material> = materials
                .iter()
                .map(|m| {
                    let m_type: Result<r_MaterialType, Error> = workshop_handle
                        .lock()
                        .unwrap()
                        .get_material_type_by_id(m.type_id, &token);
                    let m_type: MaterialType = match m_type {
                        Ok(m_type) => MaterialType {
                            id: m_type.id,
                            name: m_type.name.clone().into(),
                            description: m_type.description.clone().into(),
                        },
                        Err(_) => {
                            todo!("Set UI error message");
                        }
                    };

                    Material {
                        id: m.id,
                        name: m.name.clone().into(),
                        description: m.description.clone().into(),
                        m_type,
                        quantity: m.amount,
                        price: m.costs,
                        threshold_value: m.threshold_value,
                    }
                })
                .collect();
            ui.global::<Backend>()
                .set_materials(ModelRc::new(VecModel::from(slint_materials)));
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

            let client = workshop_handle
                .lock()
                .unwrap()
                .get_client_by_id(project.client_id, &token);
            // Check if the client was successfully retrieved otherwise handle the error
            let client = match client {
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
                Ok(client) => client,
            };

            let project_materials = workshop_handle
                .lock()
                .unwrap()
                .get_project_materials(&token);
            // Check if the project materials were successfully retrieved otherwise handle the error
            let project_materials = match project_materials {
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
                Ok(project_materials) => project_materials,
            };

            let materials = workshop_handle.lock().unwrap().get_materials(&token);
            // Check if the materials were successfully retrieved otherwise handle the error
            let materials = match materials {
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
                Ok(materials) => materials,
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

            let materials: Vec<Material> = project_materials
                .iter()
                .map(|m| {
                    let material: Result<r_Material, Error> = materials
                        .iter()
                        .find(|mat| mat.id == m.material_id)
                        .cloned()
                        .ok_or(Error::new("Material not found".to_owned(), 404));
                    let material = match material {
                        Ok(material) => {
                            let m_type: Result<r_MaterialType, Error> = material_types
                                .iter()
                                .find(|t| t.id == material.type_id)
                                .cloned()
                                .ok_or(Error::new("Material type not found".to_owned(), 404));
                            let m_type = match m_type {
                                Ok(m_type) => MaterialType {
                                    id: m_type.id,
                                    name: m_type.name.clone().into(),
                                    description: m_type.description.clone().into(),
                                },
                                Err(e) => {
                                    todo!("Set UI error message");
                                }
                            };
                            Material {
                                id: material.id,
                                name: material.name.clone().into(),
                                description: material.description.clone().into(),
                                m_type,
                                quantity: m.amount,
                                price: material.costs,
                                threshold_value: material.threshold_value,
                            }
                        }
                        Err(_) => {
                            todo!("Set UI error message");
                        }
                    };
                    material
                })
                .collect();

            let project = ProjectModel {
                id: project.id,
                name: project.name.clone().into(),
                client: ClientModel {
                    id: client.id,
                    firstName: client.firstname.clone().into(),
                    lastName: client.lastname.clone().into(),
                    phone: client.phone.clone().into(),
                },
                mats: ModelRc::new(VecModel::from(materials)),
                calculated_costs: project.estimated_costs,
                current_costs: project.costs,
                project_deadline: project.endpoint.clone().into(),
                project_start: project.startpoint.clone().into(),
            };

            ui.global::<Backend>().set_project(project);
        }
    });

    // Navigate to the Material popup
    ui.global::<Backend>().on_showAddNewMaterialPopUp({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_showMaterialPopUp(true);
        }
    });

    // Navigate to the Material Type popup
    ui.global::<Backend>().on_showAddNewMaterialTypePopUp({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_showMaterialTypePopUp(true);
        }
    });

    // Navigate to the Client popup
    ui.global::<Backend>().on_showAddNewClientPopUp({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_showClientPopUp(true);
        }
    });

    // Hide the Material popup
    ui.global::<Backend>().on_hideMaterialPopUp({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_showMaterialPopUp(false);
        }
    });

    // Hide the Material Type popup
    ui.global::<Backend>().on_hideMaterialTypePopUp({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_showMaterialTypePopUp(false);
        }
    });

    // Hide the Client popup
    ui.global::<Backend>().on_hideClientPopUp({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_showClientPopUp(false);
        }
    });

    // Navigate to the project detail view
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

    // Edit a material of a project
    ui.global::<Backend>().on_editMaterial({
        let ui_handle = ui.as_weak();
        let workshop_handle = arc_workshop_service.clone();
        let keycloak_handle = arc_keycloak.clone();
        move |material_id: i32| {
            let ui = ui_handle.unwrap();
            ui.global::<Backend>().set_showMaterialPopUp(true);

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

            let project = ui.global::<Backend>().get_project();
            let materials = workshop_handle.lock().unwrap().get_materials(&token);
            let project_materials = workshop_handle
                .lock()
                .unwrap()
                .get_project_materials(&token);
            let material_types = workshop_handle.lock().unwrap().get_material_types(&token);

            let materials = match materials {
                Ok(materials) => materials,
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
            };

            let project_materials = match project_materials {
                Ok(project_materials) => project_materials,
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
            };

            let material_types = match material_types {
                Ok(material_types) => material_types,
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
            };

            // Get materials of the project and the amount from the project materials and display it
            let material: Material =
                match materials.iter().find(|m: &&r_Material| m.id == material_id) {
                    Some(material) => {
                        let m_type: Result<r_MaterialType, Error> = material_types
                            .iter()
                            .find(|t| t.id == material.type_id)
                            .cloned()
                            .ok_or(Error::new("Material type not found".to_owned(), 404));
                        let m_type = match m_type {
                            Ok(m_type) => MaterialType {
                                id: m_type.id,
                                name: m_type.name.clone().into(),
                                description: m_type.description.clone().into(),
                            },
                            Err(e) => {
                                todo!("Set UI error message");
                            }
                        };
                        let project_material: Option<&r_ProjectMaterial> = project_materials
                            .iter()
                            .find(|m| m.material_id == material_id && m.project_id == project.id);
                        let quantity = match project_material {
                            Some(m) => m.amount,
                            None => 0,
                        };
                        Material {
                            id: material.id,
                            name: material.name.clone().into(),
                            description: material.description.clone().into(),
                            m_type,
                            quantity,
                            price: material.costs,
                            threshold_value: material.threshold_value,
                        }
                    }
                    None => {
                        todo!("Set UI error message");
                    }
                };

            todo!("Need variable to set material");
        }
    });

    // Delete a project
    ui.global::<Backend>().on_delete_project_i({
        let ui_handle = ui.as_weak();
        let workshop_handle = arc_workshop_service.clone();
        let keycloak_handle = arc_keycloak.clone();
        move |project_id: i32| {
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
            // Delete the project
            let result = workshop_handle
                .lock()
                .unwrap()
                .delete_project(project_id, &token);
            match result {
                Ok(_) => {
                    ui.global::<Backend>().invoke_route_to_project_management();
                }
                Err(e) => {
                    todo!("Set UI error message");
                }
            }
        }
    });

    // Delete a material of a project
    ui.global::<Backend>().on_deleteMaterial({
        let ui_handle = ui.as_weak();
        let workshop_handle = arc_workshop_service.clone();
        let keycloak_handle = arc_keycloak.clone();
        move |material_id: i32| {
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
            // Get the project
            let project = ui.global::<Backend>().get_project();
            // Get the project materials
            let project_materials = workshop_handle
                .lock()
                .unwrap()
                .get_project_materials(&token);
            // Check if the project materials were successfully retrieved otherwise handle the error
            let project_materials = match project_materials {
                Ok(project_materials) => project_materials,
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
            };
            // Find the project material that we want to delete
            let project_material: Option<&r_ProjectMaterial> = project_materials
                .iter()
                .find(|m| m.material_id == material_id && m.project_id == project.id);
            // Check if the project material was found otherwise handle the error
            let project_material = match project_material {
                Some(m) => m,
                None => {
                    todo!("Set UI error message");
                    return;
                }
            };
            // Delete the material from the project
            let result = workshop_handle
                .lock()
                .unwrap()
                .delete_project_material(project_material.id, &token);
            // Check if the material was successfully deleted otherwise handle the error
            if let Err(e) = result {
                todo!("Set UI error message");
                return;
            }

            // Set the project again
            ui.global::<Backend>().invoke_route_to_project_i(project.id);
        }
    });

    // Save a new material type
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
            let material_type: r_MaterialType = r_MaterialType {
                id: 0, // We can ignore it because the database will set it
                name: name.to_string(),
                description: description.to_string(),
            };
            let result = workshop_handle
                .lock()
                .unwrap()
                .create_material_type(material_type, &token);
            // Check if the material type was successfully created otherwise handle the error
            match result {
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
                Ok(_) => {
                    ui.global::<Backend>().set_showMaterialTypePopUp(false);
                    let types = workshop_handle.lock().unwrap().get_material_types(&token);
                    // Check if the material types were successfully retrieved otherwise handle the error
                    let types = match types {
                        Err(e) => {
                            todo!("Set UI error message");
                            return;
                        }
                        Ok(types) => types,
                    };
                }
            }
        }
    });

    // Save a new client
    ui.global::<Backend>().on_saveClient({
        let ui_handle = ui.as_weak();
        let workshop_handle = arc_workshop_service.clone();
        let keycloak_handle = arc_keycloak.clone();
        move |first_name: SharedString, last_name: SharedString, phone: SharedString| {
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
            let client: r_Client = r_Client {
                id: 0, // We can ignore it because the database will set it
                firstname: first_name.to_string(),
                lastname: last_name.to_string(),
                phone: phone.to_string(),
            };
            let result = workshop_handle
                .lock()
                .unwrap()
                .create_client(client, &token);
            // Check if the client was successfully created otherwise handle the error
            match result {
                Err(e) => {
                    todo!("Set UI error message");
                    return;
                }
                Ok(_) => {
                    ui.global::<Backend>().set_showClientPopUp(false);
                }
            }
        }
    });

    // Search for a material
    ui.global::<Backend>().on_search_material({
        let ui_handle = ui.as_weak();
        move |search: SharedString| {
            let ui = ui_handle.unwrap();
            let materials = ui.global::<Backend>().get_materials();
            materials.clone().filter(move |m| {
                m.name.to_lowercase().contains(&search.to_lowercase())
                    || m.description
                        .to_lowercase()
                        .contains(&search.to_lowercase())
            });
            ui.global::<Backend>().set_materials(materials);
        }
    });

    // Search for a project
    ui.global::<Backend>().on_search_project({
        let ui_handle = ui.as_weak();
        move |search: SharedString| {
            let ui = ui_handle.unwrap();
            let projects = ui.global::<Backend>().get_allProjects();
            projects
                .clone()
                .filter(move |p| p.name.to_lowercase().contains(&search.to_lowercase()));
            ui.global::<Backend>().set_allProjects(projects);
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
