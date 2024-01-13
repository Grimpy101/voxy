use super::VolumeSpec;

pub struct RawVolume {
    pub filepath: String,
    pub dimensions: [u32; 3],
    pub name: Option<String>,
    pub description: Option<String>,
    pub semantic_type: Option<String>,
    pub volume_size: Option<[f64; 3]>,
    pub voxel_size: Option<[f64; 3]>,
}

impl VolumeSpec for RawVolume {
    fn to_string(&self) -> String {
        if let Some(name) = &self.name {
            format!("{} (raw)", name)
        } else {
            format!("{} (raw)", self.filepath)
        }
    }
}
