use crate::SmartDevice;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Default)]
pub struct Room {
    devices: HashMap<String, SmartDevice>,
}

impl Room {
    pub fn new_with_devices(devices: HashMap<String, SmartDevice>) -> Room {
        Room { devices }
    }
    pub fn get_device(&self, name: &str) -> Option<&SmartDevice> {
        self.devices.get(name)
    }

    pub fn get_device_mut(&mut self, name: &str) -> Option<&mut SmartDevice> {
        self.devices.get_mut(name)
    }
    pub fn add_device(&mut self, name: String, device: SmartDevice) {
        self.devices.insert(name, device);
    }
    pub fn remove_device(&mut self, name: &str) {
        self.devices.remove(name);
    }
}

#[macro_export]
macro_rules! room {
    ($(($name: expr, $device: ty)), +) => {
        {
            let room = Room::new_with_devices( [
            $(($name.to_string(), SmartDevice::from(<$device>::default())),
            )+].into());
            room
        }
    };
}

impl<'a> IntoIterator for &'a Room {
    type Item = (&'a String, &'a SmartDevice);
    type IntoIter = std::collections::hash_map::Iter<'a, String, SmartDevice>;
    fn into_iter(self) -> Self::IntoIter {
        self.devices.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::{SmartSocket, SmartThermometer};

    use super::*;

    #[test]
    fn test_new_room() {
        let room = Room::default();
        assert!(room.devices.is_empty());
        assert!(room.devices.keys().count() == 0);
    }

    #[test]
    fn test_new_room_with_devices() {
        let mut devices = HashMap::new();
        devices.insert(
            "Socket".to_string(),
            SmartDevice::SmartSocket(SmartSocket::default()),
        );
        devices.insert(
            "Termometer".to_string(),
            SmartDevice::SmartThermometer(SmartThermometer::default()),
        );

        let room = Room::new_with_devices(devices);

        assert_eq!(room.devices.len(), 2);
        assert!(room.devices.contains_key("Socket"));
        assert!(room.devices.contains_key("Termometer"));
    }

    #[test]
    fn test_get_device() {
        let mut room = Room::default();
        let device = SmartDevice::from(SmartThermometer::default());
        room.add_device("Termometer".to_string(), device);
        let device = room.get_device("Termometer");
        assert!(device.is_some());
        let device = room.get_device("Socket");
        assert!(device.is_none());
    }
    #[test]
    fn test_get_device_mut() {
        let mut room = Room::default();
        let device = SmartDevice::from(SmartThermometer::default());
        room.add_device("Termometer".to_string(), device);
        let device = room.get_device_mut("Termometer");
        assert!(device.is_some());
        let device = room.get_device_mut("Socket");
        assert!(device.is_none());
    }

    #[test]
    fn test_add_device() {
        let mut room = Room::default();
        let device = SmartDevice::from(SmartThermometer::default());
        room.add_device("Termometer".to_string(), device);
        assert!(room.devices.contains_key("Termometer"));
    }

    #[test]
    fn test_remove_device() {
        let mut room = Room::default();
        let device = SmartDevice::from(SmartThermometer::default());
        room.add_device("Termometer".to_string(), device);
        assert!(room.devices.contains_key("Termometer"));
        room.remove_device("Termometer");
        assert!(!room.devices.contains_key("Termometer"));
    }

    #[test]
    fn test_macro_room() {
        let room = room![("Socket", SmartSocket), ("Termometer", SmartThermometer)];
        assert!(room.devices.contains_key("Socket"));
        assert!(room.devices.contains_key("Termometer"));

        assert_eq!(
            room.get_device("Socket").unwrap(),
            &SmartDevice::from(SmartSocket::default())
        );

        assert_eq!(
            room.get_device("Termometer").unwrap(),
            &SmartDevice::from(SmartThermometer::default())
        )
    }
}
