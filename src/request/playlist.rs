use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaylistSrc {
    src: String,
}

impl PlaylistSrc {
    pub fn src(&self) -> &String {
        &self.src
    }
}
