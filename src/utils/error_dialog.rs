/*

    Author: Justin
    Description: Open an error dialog for the user to see an error message

*/

use error_window::dialog::{self, DialogBox}; 

pub fn show_error(title: String, message: String) {
    dialog::Message::new(message)
        .title(title)
        .show()
        .expect("Could not display dialog box");
}