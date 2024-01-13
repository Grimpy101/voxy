use std::fmt::Display;

use self::raw::RawVolume;

pub mod raw;

trait VolumeSpec {
    fn to_string(&self) -> String;
}

pub enum Volume {
    Raw(raw::RawVolume),
}

impl Volume {
    pub fn create_raw(parameters: RawVolume) -> Self {
        Self::Raw(parameters)
    }
}

impl Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Volume::Raw(spec) => {
                write!(f, "{}", spec.to_string())
            }
        }
    }
}
