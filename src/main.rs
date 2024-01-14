use fltk::app::{self};
use messages::{process::ProcessMessage, ui::UIMessage, volume::VolumeMessage};
use state::State;

mod dialogs;
mod messages;
mod state;
mod ui;
mod volumes;

fn main() {
    let mut state = State::new();
    let application = app::App::default().with_scheme(app::Scheme::Gtk);
    let (ui_sender, ui_receiver) = app::channel::<UIMessage>();
    let (process_sender, process_receiver) = app::channel::<ProcessMessage>();
    let (volume_sender, volume_receiver) = app::channel::<VolumeMessage>();

    let mut widgets = ui::build_ui(
        ui_sender.clone(),
        volume_sender.clone(),
        process_sender.clone(),
    );

    while application.wait() {
        if let Some(message) = ui_receiver.recv() {
            message.handle(&mut widgets, &mut state);
        }

        if let Some(message) = volume_receiver.recv() {
            message.handle(&mut state, ui_sender.clone());
        }

        if let Some(message) = process_receiver.recv() {
            message.handle(&mut state, ui_sender.clone());
        }
    }
}
