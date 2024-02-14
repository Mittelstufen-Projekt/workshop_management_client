/*

    Author: Jasha, Justin
    Description: This is the main file for the workshop management client. It is the entry point for the application.

*/

mod utils;
mod models;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = WorkshopClient::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()
}
