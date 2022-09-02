use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Eq)]
pub struct Package {
    path: String,
    version: u32,
    checksum: String,
    has_installer: bool,
}

impl Package {
    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn checksum(&self) -> &str {
        &self.checksum
    }
    pub fn has_installer(&self) -> bool {
        self.has_installer
    }
}
