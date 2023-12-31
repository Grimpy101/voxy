use crate::{operation::Operation, volume::Volume};

pub struct State {
    volumes: Vec<Volume>,
    operations: Vec<Operation>,
}

impl State {
    pub fn new() -> Self {
        Self {
            volumes: Vec::new(),
            operations: Vec::new(),
        }
    }

    pub fn add_volume(&mut self, volume: Volume) {
        self.volumes.push(volume);
    }

    pub fn remove_volume(&mut self, index: usize) {
        let volume_query = self.volumes.get(index);
        if let Some(volume) = volume_query {
            for (i, operation) in self.operations.iter().enumerate() {
                if operation.uses_volume(volume) {
                    self.operations.remove(i);
                }
            }
            self.volumes.remove(index);
        }
    }

    pub fn swap_volumes(&mut self, first_index: usize, second_index: usize) {
        self.volumes.swap(first_index, second_index);
    }

    pub fn add_operation(&mut self, operation: Operation) {
        self.operations.push(operation);
    }

    pub fn remove_operation(&mut self, index: usize) {
        self.operations.remove(index);
    }

    pub fn swap_operations(&mut self, first_index: usize, second_index: usize) {
        self.operations.swap(first_index, second_index);
    }
}
