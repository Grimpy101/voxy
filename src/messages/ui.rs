use fltk::prelude::BrowserExt;

use crate::{ui::Widgets, State};

#[derive(Clone)]
pub enum UIMessage {
    VolumeListUpdate,
    SelectedVolumesRemove,
    SelectedProcessesRemove,
}

impl UIMessage {
    pub fn handle(&self, widgets: &mut Widgets, state: &mut State) {
        match self {
            UIMessage::VolumeListUpdate => {
                let volume_browser = &mut widgets.volume_browser;
                volume_browser.clear();
                for volume_specification in state.volumes().iter() {
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
            UIMessage::SelectedProcessesRemove => {}
        }
    }
}
