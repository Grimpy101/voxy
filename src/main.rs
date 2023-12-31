use std::collections::HashMap;

use dialogs::ImportDialog;
use messages::Message;
use relm4::{
    gtk::{self, prelude::WidgetExt},
    ComponentParts, RelmApp, SimpleComponent,
};
use state::State;

mod dialogs;
mod main_window;
mod messages;
mod operation;
mod state;
mod volume;

/// The global state of GUI application
struct ApplicationModel {
    state: State,
    import_dialog: Option<ImportDialog>,
}

/// A struct of widgets that need to be updated
struct ApplicationWidgets {
    dialogs: HashMap<String, gtk::Window>,
    open_dialog: Option<String>,
}

impl SimpleComponent for ApplicationModel {
    type Input = Message; // Received messages
    type Output = (); // Sent messages
    type Init = (); // Type of init data
    type Root = gtk::ApplicationWindow; // Root GTK widget
    type Widgets = ApplicationWidgets;

    fn init_root() -> Self::Root {
        gtk::ApplicationWindow::builder()
            .title("Voxy")
            .modal(true)
            .default_width(840)
            .default_height(480)
            .build()
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::prelude::ComponentSender<Self>,
    ) -> relm4::prelude::ComponentParts<Self> {
        let model = ApplicationModel {
            state: State::new(),
            import_dialog: None,
        };

        main_window::init_main_window(root, sender.clone());
        let dialogs = dialogs::create_dialogs(root, sender);

        let widgets = ApplicationWidgets {
            dialogs,
            open_dialog: None,
        };
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::prelude::ComponentSender<Self>) {
        match message {
            Message::ImportOk => {
                println!("Oui!");
                self.import_dialog = None;
            }
            Message::ShowImportDialog => {
                self.import_dialog = Some(ImportDialog::Raw);
            }
            Message::CloseImportDialog => {
                self.import_dialog = None;
            }
        }
    }

    fn update_view(
        &self,
        widgets: &mut Self::Widgets,
        _sender: relm4::prelude::ComponentSender<Self>,
    ) {
        if let Some(import_dialog) = &self.import_dialog {
            match import_dialog {
                ImportDialog::Raw => {
                    widgets
                        .dialogs
                        .get("raw_import")
                        .expect("Could not get Raw Import dialog")
                        .show();
                    widgets.open_dialog = Some("raw_import".to_string());
                }
            }
        } else if let Some(open_dialog) = &widgets.open_dialog {
            if let Some(dialog) = widgets.dialogs.get(open_dialog) {
                dialog.set_visible(false);
                widgets.open_dialog = None;
            }
        }
    }
}

fn main() {
    let application = RelmApp::new("lgm.grimpy.voxy");
    application.run::<ApplicationModel>(());
}
