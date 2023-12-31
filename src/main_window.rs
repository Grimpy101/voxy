use relm4::{
    gtk::{
        self,
        glib::clone,
        prelude::{BoxExt, ButtonExt, GtkWindowExt, PopoverExt, WidgetExt},
    },
    ComponentSender, RelmWidgetExt,
};

use crate::{messages::Message, ApplicationModel};

pub fn init_main_window(root: &gtk::ApplicationWindow, sender: ComponentSender<ApplicationModel>) {
    let container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    container.set_margin_all(10);
    container.set_vexpand(true);
    container.set_hexpand(true);

    init_volume_view(&container, sender);
    init_operation_view(&container);

    root.set_child(Some(&container));
}

fn init_volume_view(container: &gtk::Box, sender: ComponentSender<ApplicationModel>) {
    let volume_container = gtk::Box::new(gtk::Orientation::Vertical, 10);
    volume_container.set_vexpand(true);
    init_volume_toolbar(&volume_container, sender);
    init_volume_list(&volume_container);
    container.append(&volume_container);
}

fn init_volume_toolbar(container: &gtk::Box, sender: ComponentSender<ApplicationModel>) {
    let toolbar = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    toolbar.set_hexpand(true);

    let import_volume_button = gtk::MenuButton::builder().label("Import volume").build();
    import_volume_button.set_label("Import volume");
    let import_popover = gtk::Popover::new();
    import_popover.set_position(gtk::PositionType::Left);
    let import_popover_container = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let import_raw = gtk::Button::with_label("Raw");
    let import_bvp = gtk::Button::with_label("BVP");

    import_popover_container.append(&import_raw);
    import_popover_container.append(&import_bvp);

    import_popover.set_child(Some(&import_popover_container));
    import_volume_button.set_popover(Some(&import_popover));
    toolbar.append(&import_volume_button);
    container.append(&toolbar);

    import_raw.connect_clicked(clone!(@strong sender => move |_| {
        import_popover.set_visible(false);
        sender.input(Message::ShowImportDialog);
    }));
}

fn init_volume_list(container: &gtk::Box) {
    let list_box = gtk::ListBox::new();
    let scrolled_window = gtk::ScrolledWindow::builder()
        .child(&list_box)
        .vexpand(true)
        .build();
    container.append(&scrolled_window);
}

fn init_operation_view(container: &gtk::Box) {
    let operation_container = gtk::Box::new(gtk::Orientation::Vertical, 10);
    operation_container.set_vexpand(true);
    init_operation_toolbar(&operation_container);
    init_operation_list(&operation_container);
    container.append(&operation_container);
}

fn init_operation_toolbar(container: &gtk::Box) {
    let toolbar = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    toolbar.set_hexpand(true);

    let import_volume_button = gtk::MenuButton::builder().label("Import volume").build();
    import_volume_button.set_label("Import volume");
    let import_popover = gtk::Popover::new();
    import_popover.set_position(gtk::PositionType::Left);
    let import_popover_container = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let import_raw = gtk::Button::with_label("Export");

    import_popover_container.append(&import_raw);

    import_popover.set_child(Some(&import_popover_container));
    import_volume_button.set_popover(Some(&import_popover));
    toolbar.append(&import_volume_button);
    container.append(&toolbar);
}

fn init_operation_list(container: &gtk::Box) {
    let list_box = gtk::ListBox::new();
    let scrolled_window = gtk::ScrolledWindow::builder()
        .child(&list_box)
        .vexpand(true)
        .build();
    container.append(&scrolled_window);
}
