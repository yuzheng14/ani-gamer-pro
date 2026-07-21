use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceId {
    #[serde(rename = "deviceid")]
    device_id: String,
}

impl DeviceId {
    pub fn into_device_id(self) -> String {
        self.device_id
    }
}
