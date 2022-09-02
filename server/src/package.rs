use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Package {
    path: String,
    version: u32,
    crc: u32,
    has_installer: bool,
}

impl Package {
    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn checksum(&self) -> u32 {
        self.crc
    }
    pub fn has_installer(&self) -> bool {
        self.has_installer
    }
}
