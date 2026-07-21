use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct DeviceId(Arc<Mutex<Option<String>>>);

impl DeviceId {
    pub fn get_cloned(&self) -> Option<String> {
        self.0.lock().unwrap().clone()
    }

    pub fn set(&self, device_id: Option<String>) {
        *self.0.lock().unwrap() = device_id
    }
}

impl PartialEq for DeviceId {
    fn eq(&self, other: &Self) -> bool {
        self.get_cloned() == other.get_cloned()
    }
}

impl Eq for DeviceId {}

#[cfg(test)]
mod test {
    use std::thread;

    use super::*;

    #[test]
    fn default_device_id_is_empty() {
        let device_id = DeviceId::default();

        assert_eq!(device_id.get_cloned(), None);
    }

    #[test]
    fn device_id_can_be_set_and_cleared() {
        let device_id = DeviceId::default();

        device_id.set(Some("device-123".to_string()));
        assert_eq!(device_id.get_cloned().as_deref(), Some("device-123"));

        device_id.set(None);
        assert_eq!(device_id.get_cloned(), None);
    }

    #[test]
    fn cloned_device_ids_share_the_same_value() {
        let first = DeviceId::default();
        let second = first.clone();

        first.set(Some("device-from-first".to_string()));
        assert_eq!(second.get_cloned().as_deref(), Some("device-from-first"));

        second.set(Some("device-from-second".to_string()));
        assert_eq!(first.get_cloned().as_deref(), Some("device-from-second"));
        assert_eq!(first, second);
    }

    #[test]
    fn device_id_is_shared_across_threads() {
        let device_id = DeviceId::default();
        let worker_device_id = device_id.clone();

        thread::spawn(move || {
            worker_device_id.set(Some("device-from-worker".to_string()));
        })
        .join()
        .expect("device ID worker should finish successfully");

        assert_eq!(
            device_id.get_cloned().as_deref(),
            Some("device-from-worker")
        );
    }

    #[test]
    fn independently_created_device_ids_compare_by_value() {
        let first = DeviceId::default();
        let second = DeviceId::default();

        assert_eq!(first, second);

        first.set(Some("device-123".to_string()));
        assert_ne!(first, second);

        second.set(Some("device-123".to_string()));
        assert_eq!(first, second);
    }
}
