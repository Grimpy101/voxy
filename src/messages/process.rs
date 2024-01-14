use fltk::app::Sender;

use crate::state::State;

use super::ui::UIMessage;

#[derive(Clone)]
pub enum ProcessMessage {
    AddExport,
}

impl ProcessMessage {
    pub fn handle(&self, state: &mut State, ui_sender: Sender<UIMessage>) {
        match self {
            ProcessMessage::AddExport => {}
        }
    }
}
