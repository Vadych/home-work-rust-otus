use crate::SmartDevice;
use std::collections::HashMap;
#[derive(Debug, Default)]
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
    pub fn add_device<T>(&mut self, name: T, device: SmartDevice)
    where
        T: Into<String>,
    {
        self.devices.insert(name.into(), device);
    }
    pub fn remove_device(&mut self, name: &str) {
        self.devices.remove(name);
    }
}

#[macro_export]
macro_rules! room {
    ($(($name: expr, $device: ty, $ip: expr)), +) => {
        {
            let room = Room::new_with_devices( [
            $(($name.to_string(), SmartDevice::from(<$device>::connect($ip).expect("Failed to connect"))),
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
    use std::io::Cursor;

    use crate::SmartSocket;

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
            SmartDevice::SmartSocket(SmartSocket::new(Cursor::new(Vec::new()))),
        );

        let room = Room::new_with_devices(devices);

        assert_eq!(room.devices.len(), 1);
        assert!(room.devices.contains_key("Socket"));
    }

    #[test]
    fn test_get_device() {
        let mut room = Room::default();
        let device = SmartDevice::from(SmartSocket::new(Cursor::new(Vec::new())));
        room.add_device("Socket", device);
        let device = room.get_device("Socket");
        assert!(device.is_some());
        let device = room.get_device("Termometer");
        assert!(device.is_none());
    }
    #[test]
    fn test_get_device_mut() {
        let mut room = Room::default();
        let device = SmartDevice::from(SmartSocket::new(Cursor::new(Vec::new())));
        room.add_device("Socket", device);
        let device = room.get_device_mut("Socket");
        assert!(device.is_some());
        let device = room.get_device_mut("Termometer");
        assert!(device.is_none());
    }

    #[test]
    fn test_add_device() {
        let mut room = Room::default();
        let device = SmartDevice::from(SmartSocket::new(Cursor::new(Vec::new())));
        room.add_device("Socket", device);
        assert!(room.devices.contains_key("Socket"));
    }

    #[test]
    fn test_remove_device() {
        let mut room = Room::default();
        let device = SmartSocket::new(Cursor::new(Vec::new())).into();
        room.add_device("Socket", device);
        assert!(room.devices.contains_key("Socket"));
        room.remove_device("Socket");
        assert!(!room.devices.contains_key("Socket"));
    }
}
