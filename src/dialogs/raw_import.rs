use relm4::{
    gtk::{
        self,
        glib::clone,
        prelude::{BoxExt, ButtonExt, DialogExt, FileChooserExt, GtkWindowExt, WidgetExt},
    },
    ComponentSender,
};

use crate::{messages::Message, ApplicationModel};

pub fn create_raw_import(sender: ComponentSender<ApplicationModel>) -> gtk::Window {
    let import_dialog = gtk::Window::builder()
        .default_width(300)
        .default_height(200)
        .modal(true)
        .visible(false)
        .title("Add raw volume")
        .build();
    let container = gtk::Box::new(gtk::Orientation::Vertical, 5);

    let file_dialog = gtk::FileChooserDialog::new(
        Some("Choose file"),
        Some(&import_dialog),
        gtk::FileChooserAction::Open,
        &[
            ("Open", gtk::ResponseType::Accept),
            ("Cancel", gtk::ResponseType::Cancel),
        ],
    );
    let file_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    file_box.set_hexpand(true);
    let file_input = gtk::Entry::new();
    file_input.set_hexpand(true);
    let file_button = gtk::Button::with_label("Choose file");
    file_box.append(&file_input);
    file_box.append(&file_button);

    let button_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let ok_button = gtk::Button::with_label("Ok");
    let cancel_button = gtk::Button::with_label("Cancel");
    button_box.append(&ok_button);
    button_box.append(&cancel_button);

    container.append(&file_box);
    container.append(&button_box);
    import_dialog.set_child(Some(&container));

    ok_button.connect_clicked(clone!(@strong sender => move |_| {
        sender.input(Message::ImportOk);
    }));
    cancel_button.connect_clicked(clone!(@strong sender => move |_| {
        sender.input(Message::CloseImportDialog);
    }));

    file_dialog.connect_response(clone!(@strong sender => move |chooser, response| {
        match response {
            gtk::ResponseType::Accept => {
                println!("{}", chooser.file().unwrap());
                chooser.close();
            },
            gtk::ResponseType::Cancel | gtk::ResponseType::Close => {
                chooser.close();
            },
            _ => ()
        }
    }));

    file_button.connect_clicked(clone!(@strong sender => move |_| {
        println!("Aha!");
        file_dialog.show();
    }));

    import_dialog
}
