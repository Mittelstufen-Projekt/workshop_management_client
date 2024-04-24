/*

    Author: Justin
    Description: Open a file dialog for the user to select a file

*/

use tinyfiledialogs::open_file_dialog;

pub fn open_browse_dialog() -> Option<String> {
    let dialog = open_file_dialog("Select a file", ".", None);
    let path = match dialog {
        Some(path) => path,
        None => return None,
    };
    Some(path)
}