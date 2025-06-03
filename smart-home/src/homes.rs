use crate::SmartHomeError;
use crate::{SmartDevice, rooms::Room};
use std::collections::HashMap;

pub struct Home {
    pub name: String,
    rooms: HashMap<String, Room>,
}
impl Home {
    pub fn new<T>(name: T) -> Home
    where
        T: Into<String>,
    {
        Home {
            name: name.into(),
            rooms: HashMap::new(),
        }
    }
    pub fn new_with_rooms<T>(name: T, rooms: HashMap<String, Room>) -> Home
    where
        T: Into<String>,
    {
        Home {
            name: name.into(),
            rooms,
        }
    }
    pub fn get_room(&self, name: &str) -> Option<&Room> {
        self.rooms.get(name)
    }
    pub fn get_room_mut(&mut self, name: &str) -> Option<&mut Room> {
        self.rooms.get_mut(name)
    }
    pub fn add_room<T>(&mut self, name: T, room: Room)
    where
        T: Into<String>,
    {
        self.rooms.insert(name.into(), room);
    }
    pub fn remove_room(&mut self, name: &str) {
        self.rooms.remove(name);
    }
    pub fn get_device(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> Result<&mut SmartDevice, SmartHomeError> {
        match self.get_room_mut(room_name) {
            Some(room) => match room.get_device_mut(device_name) {
                Some(device) => Ok(device),
                None => Err(SmartHomeError::DeviceNotFound(device_name.to_string())),
            },
            None => Err(SmartHomeError::RoomNotFound(room_name.to_string())),
        }
    }
}

impl<'a> IntoIterator for &'a Home {
    type Item = (&'a String, &'a Room);
    type IntoIter = std::collections::hash_map::Iter<'a, String, Room>;
    fn into_iter(self) -> Self::IntoIter {
        self.rooms.iter()
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::SmartSocket;

    use super::*;

    #[test]
    fn test_new_home() {
        let name = "My Home";
        let home = Home::new(name);
        assert_eq!(home.name, name);
        assert!(home.rooms.is_empty());
    }

    #[test]
    fn test_new_home_with_rooms() {
        let name = "Test Home";
        let mut rooms = HashMap::new();
        rooms.insert("Room 1".to_string(), Room::default());
        rooms.insert("Room 2".to_string(), Room::default());

        let home = Home::new_with_rooms(name, rooms);

        assert_eq!(home.name, name);
        assert_eq!(home.rooms.len(), 2);
        assert!(home.rooms.contains_key("Room 1"));
        assert!(home.rooms.contains_key("Room 2"));
    }
    #[test]
    fn test_get_room() {
        let mut home = Home::new("Test Home");

        let result = home.get_room("Room 1");
        assert!(result.is_none());

        home.rooms.insert("Room 1".to_string(), Room::default());
        let result = home.get_room("Room 1");
        assert!(result.is_some());

        let result = home.get_room("Room 2");
        assert!(result.is_none());
    }
    #[test]
    fn test_get_room_mut() {
        let mut home = Home::new("Test Home");

        let result = home.get_room_mut("Room 1");
        assert!(result.is_none());

        home.rooms.insert("Room 1".to_string(), Room::default());
        let result = home.get_room_mut("Room 1");
        assert!(result.is_some());

        let result = home.get_room_mut("Room 2");
        assert!(result.is_none());
    }
    #[test]
    fn test_add_room() {
        let mut home = Home::new("Test Home");
        home.add_room("Room 1".to_string(), Room::default());
        assert!(home.rooms.contains_key("Room 1"));

        home.add_room("Room 1".to_string(), Room::default());
        assert_eq!(home.rooms.len(), 1);

        home.add_room("Room 2".to_string(), Room::default());
        assert!(home.rooms.contains_key("Room 2"));
    }
    #[test]
    fn test_remove_room() {
        let mut home = Home::new("Test Home");
        home.add_room("Room 1".to_string(), Room::default());
        home.add_room("Room 2".to_string(), Room::default());

        home.remove_room("Room 2");
        assert_eq!(home.rooms.len(), 1);
        assert!(home.rooms.contains_key("Room 1"));
        assert!(!home.rooms.contains_key("Room 2"));
    }
    #[test]
    fn test_get_device() {
        let mut home = Home::new("Test Home");
        let mut room = Room::default();
        room.add_device(
            "Device 1".to_string(),
            SmartDevice::from(SmartSocket::new(Cursor::new(Vec::new()))),
        );
        home.add_room("Room 1".to_string(), room);

        let result = home.get_device("Room 1", "Device 1");
        assert!(result.is_ok());

        let result = home.get_device("Room 1", "Device 2");
        assert!(result.is_err());

        let result = home.get_device("Room 2", "Device 2");
        assert!(result.is_err());
    }
}
