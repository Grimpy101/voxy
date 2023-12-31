use fltk::{
    app::{self, Sender},
    browser::HoldBrowser,
    enums::Shortcut,
    group::{Flex, FlexType},
    menu::{MenuButton, MenuFlag},
    prelude::{GroupExt, MenuExt, WidgetBase, WidgetExt},
    window::Window,
};
use messages::UIMessage;
use volumes::Volume;

mod messages;
mod table;
mod volumes;

struct State {
    volumes: Vec<Volume>,
}

impl State {
    pub fn new() -> Self {
        Self {
            volumes: Vec::new(),
        }
    }
}

fn main() {
    let mut state = State::new();
    let application = app::App::default().with_scheme(app::Scheme::Gtk);
    let (sender, receiver) = app::channel::<UIMessage>();
    build_ui(sender);
    while application.wait() {
        if let Some(message) = receiver.recv() {
            match message {
                UIMessage::SelectRawImport => {
                    println!("Dela!");
                }
            }
        }
    }
}

fn build_ui(sender: Sender<UIMessage>) {
    let mut main_window = Window::new(500, 500, 500, 300, "Voxy");
    main_window.make_resizable(true);
    main_window.resizable(&main_window);

    let mut container = Flex::default_fill();
    container.set_type(FlexType::Row);
    container.set_spacing(10);

    build_volume_view(sender);

    container.end();

    main_window.end();
    main_window.show();
}

fn build_volume_view(sender: Sender<UIMessage>) {
    let mut split = Flex::default_fill();
    split.set_type(FlexType::Column);

    let mut toolbar = Flex::default();
    toolbar.set_type(FlexType::Row);
    split.fixed(&toolbar, 30);

    let mut import_button = MenuButton::default().with_label("Import volume");
    import_button.add_emit(
        "Raw volume",
        Shortcut::empty(),
        MenuFlag::empty(),
        sender,
        UIMessage::SelectRawImport,
    );

    toolbar.end();

    let browser = HoldBrowser::default();

    split.end();
}
