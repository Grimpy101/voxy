use dialogs::raw_import::RawImportDialog;
use fltk::{
    app::{self, Sender},
    browser::MultiBrowser,
    button::Button,
    enums::Shortcut,
    group::{Flex, FlexType},
    menu::{MenuButton, MenuFlag},
    prelude::{BrowserExt, GroupExt, MenuExt, WidgetBase, WidgetExt},
    window::Window,
};
use messages::UIMessage;
use volumes::Volume;

mod dialogs;
mod messages;
mod table;
mod volumes;

struct Widgets {
    volume_browser: MultiBrowser,
}

struct State {
    volumes: Vec<Volume>,
}

impl State {
    pub fn new() -> Self {
        Self {
            volumes: Vec::new(),
        }
    }

    pub fn add_volume(&mut self, volume_specification: Volume, sender: Sender<UIMessage>) {
        self.volumes.push(volume_specification);
        sender.send(UIMessage::VolumeListUpdate);
    }

    pub fn remove_volume(&mut self, i: usize) {
        self.volumes.remove(i);
    }
}

fn main() {
    let mut state = State::new();
    let application = app::App::default().with_scheme(app::Scheme::Gtk);
    let (sender, receiver) = app::channel::<UIMessage>();

    let mut widgets = build_ui(sender.clone());

    while application.wait() {
        if let Some(message) = receiver.recv() {
            match message {
                UIMessage::RawImportOpen => {
                    let dialog = RawImportDialog::default();
                    if let Some(volume_specification) = dialog.value() {
                        state.add_volume(volume_specification, sender.clone());
                    }
                }
                UIMessage::VolumeListUpdate => {
                    let volume_browser = &mut widgets.volume_browser;
                    volume_browser.clear();
                    for volume_specification in state.volumes.iter() {
                        volume_browser.add(&volume_specification.to_string());
                    }
                }
                UIMessage::SelectedVolumesRemove => {
                    let volume_browser = &mut widgets.volume_browser;
                    let mut selected = volume_browser.selected_items();
                    selected.sort();
                    selected.reverse();
                    for i in selected {
                        volume_browser.remove(i);
                        state.remove_volume((i - 1) as usize);
                    }
                }
            }
        }
    }
}

fn build_ui(sender: Sender<UIMessage>) -> Widgets {
    let mut main_window = Window::new(500, 500, 500, 300, "Voxy");
    main_window.make_resizable(true);
    main_window.resizable(&main_window);

    let mut container = Flex::default_fill();
    container.set_type(FlexType::Row);
    container.set_spacing(10);

    let volume_browser = build_volume_view(sender);

    container.end();

    main_window.end();
    main_window.show();

    Widgets { volume_browser }
}

fn build_volume_view(sender: Sender<UIMessage>) -> MultiBrowser {
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
        sender.clone(),
        UIMessage::RawImportOpen,
    );
    let mut remove_button = Button::default().with_label("Remove volume");
    remove_button.set_callback({
        let sender = sender.clone();
        move |_| sender.send(UIMessage::SelectedVolumesRemove)
    });

    toolbar.end();

    let browser = MultiBrowser::default();

    split.end();

    browser
}
