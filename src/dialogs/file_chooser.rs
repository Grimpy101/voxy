pub fn create_file_chooser() {
    let file_dialog = gtk::FileChooserDialog::new(
        Some("Choose file"),
        Some(&import_dialog),
        gtk::FileChooserAction::Open,
        &[
            ("Open", gtk::ResponseType::Accept),
            ("Cancel", gtk::ResponseType::Cancel),
        ],
    );
}
