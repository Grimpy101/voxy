use std::collections::HashMap;

use relm4::{
    gtk::{self},
    ComponentSender,
};

use crate::ApplicationModel;

mod raw_import;

pub enum ImportDialog {
    Raw,
}

pub fn create_dialogs(
    _root: &gtk::ApplicationWindow,
    sender: ComponentSender<ApplicationModel>,
) -> HashMap<String, gtk::Window> {
    let mut dialogs = HashMap::new();
    let raw_import = raw_import::create_raw_import(sender);
    dialogs.insert("raw_import".to_string(), raw_import);
    dialogs
}
