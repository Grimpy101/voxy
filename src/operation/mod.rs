use std::sync::Arc;

use crate::volume::Volume;

pub enum Operation {}

impl Operation {
    pub fn get_volumes(&self) -> Vec<Arc<Volume>> {
        Vec::new()
    }

    pub fn uses_volume(&self, volume: &Volume) -> bool {
        let operation_volumes = self.get_volumes();
        for operation_volume_rc in operation_volumes.iter() {
            let operation_volume = operation_volume_rc.as_ref();
            if operation_volume == volume {
                return true;
            }
        }
        return false;
    }
}
