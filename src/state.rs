use fltk::app::Sender;

use crate::{messages::ui::UIMessage, volumes::Volume};

pub struct State {
    volumes: Vec<Volume>,
}

impl State {
    pub fn new() -> Self {
        Self {
            volumes: Vec::new(),
        }
    }

    pub fn volumes(&self) -> &Vec<Volume> {
        &self.volumes
    }

    pub fn add_volume(&mut self, volume_specification: Volume, sender: Sender<UIMessage>) {
        self.volumes.push(volume_specification);
        sender.send(UIMessage::VolumeListUpdate);
    }

    pub fn remove_volume(&mut self, i: usize) {
        self.volumes.remove(i);
    }
}
