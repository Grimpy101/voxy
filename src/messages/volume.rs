use fltk::app::Sender;

use crate::{dialogs::raw_import::RawImportDialog, State};

use super::ui::UIMessage;

#[derive(Clone)]
pub enum VolumeMessage {
    AddRaw,
}

impl VolumeMessage {
    pub fn handle(&self, state: &mut State, ui_sender: Sender<UIMessage>) {
        match self {
            VolumeMessage::AddRaw => {
                let dialog = RawImportDialog::default();
                if let Some(volume_specification) = dialog.value() {
                    state.add_volume(volume_specification, ui_sender);
                }
            }
        }
    }
}
