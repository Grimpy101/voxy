pub enum VolumeType {
    RAW {
        width: u32,
        height: u32,
        depth: u32,
        voxel_size: u32,
    },
    BVP {
        modality_index: usize,
    },
}

pub struct Volume {
    filepath: String,
    volume_type: VolumeType,
}

impl Volume {
    pub fn new(filepath: String, volume_type: VolumeType) -> Self {
        Self {
            filepath,
            volume_type,
        }
    }
}

impl PartialEq for Volume {
    fn eq(&self, other: &Self) -> bool {
        self.filepath == other.filepath
    }
}

impl Eq for Volume {}
