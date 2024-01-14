use fltk::{
    app::Sender,
    browser::MultiBrowser,
    button::Button,
    enums::Shortcut,
    group::{Flex, FlexType},
    menu::{MenuButton, MenuFlag},
    prelude::*,
    window::Window,
};

use crate::messages::{process::ProcessMessage, ui::UIMessage, volume::VolumeMessage};

pub struct Widgets {
    pub volume_browser: MultiBrowser,
    pub process_browser: MultiBrowser,
}

pub fn build_ui(
    ui_sender: Sender<UIMessage>,
    volume_sender: Sender<VolumeMessage>,
    process_sender: Sender<ProcessMessage>,
) -> Widgets {
    let mut main_window = Window::new(500, 500, 500, 300, "Voxy");
    main_window.make_resizable(true);
    main_window.resizable(&main_window);

    let mut container = Flex::default_fill();
    container.set_type(FlexType::Row);
    container.set_spacing(10);

    let volume_browser = build_volume_view(ui_sender.clone(), volume_sender);
    let process_browser = build_process_view(ui_sender, process_sender);

    container.end();

    main_window.end();
    main_window.show();

    Widgets {
        volume_browser,
        process_browser,
    }
}

fn build_volume_view(
    ui_sender: Sender<UIMessage>,
    volume_sender: Sender<VolumeMessage>,
) -> MultiBrowser {
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
        volume_sender.clone(),
        VolumeMessage::AddRaw,
    );
    let mut remove_button = Button::default().with_label("Remove volume");
    remove_button.set_callback({
        let sender = ui_sender.clone();
        move |_| sender.send(UIMessage::SelectedVolumesRemove)
    });

    toolbar.end();

    let browser = MultiBrowser::default();
    split.end();

    browser
}

fn build_process_view(
    ui_sender: Sender<UIMessage>,
    processes_sender: Sender<ProcessMessage>,
) -> MultiBrowser {
    let mut split = Flex::default_fill();
    split.set_type(FlexType::Column);

    let mut toolbar = Flex::default();
    toolbar.set_type(FlexType::Row);
    split.fixed(&toolbar, 30);

    let mut add_button = MenuButton::default().with_label("Add button");
    add_button.add_emit(
        "Export",
        Shortcut::empty(),
        MenuFlag::empty(),
        processes_sender.clone(),
        ProcessMessage::AddExport,
    );
    let mut remove_button = Button::default().with_label("Remove volume");
    remove_button.set_callback({
        let sender = ui_sender.clone();
        move |_| sender.send(UIMessage::SelectedProcessesRemove)
    });

    toolbar.end();

    let browser = MultiBrowser::default();
    split.end();

    browser
}
