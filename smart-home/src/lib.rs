pub mod devices;
pub mod homes;
pub mod report;
pub mod rooms;
pub mod subscrib;

use std::collections::HashMap;

pub use devices::smartsocket::SmartSocket;
pub use devices::termo::SmartThermometer;
pub use homes::Home;
pub use report::Report;
pub use rooms::Room;

#[derive(Debug)]
pub enum SmartDevice {
    SmartThermometer(SmartThermometer),
    SmartSocket(SmartSocket),
}

impl From<SmartSocket> for SmartDevice {
    fn from(socket: SmartSocket) -> Self {
        SmartDevice::SmartSocket(socket)
    }
}

impl From<SmartThermometer> for SmartDevice {
    fn from(thermometer: SmartThermometer) -> Self {
        SmartDevice::SmartThermometer(thermometer)
    }
}

#[derive(Debug)]
pub enum SmartHomeError {
    DeviceNotFound(String),
    RoomNotFound(String),
    ConnectionError(std::io::Error),
}

impl std::fmt::Display for SmartHomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartHomeError::DeviceNotFound(name) => write!(f, "Device {} not found", name),
            SmartHomeError::RoomNotFound(name) => write!(f, "Room {} not found", name),
            SmartHomeError::ConnectionError(err) => write!(f, "Connection error: {err}"),
        }
    }
}

impl std::error::Error for SmartHomeError {}

impl From<std::io::Error> for SmartHomeError {
    fn from(err: std::io::Error) -> Self {
        SmartHomeError::ConnectionError(err)
    }
}

pub struct HomeWithRoom;
pub struct HomeWithOutRoom;

#[derive(Default)]
pub struct HomeBuilder<T> {
    name: String,
    rooms: HashMap<String, Room>,
    last_room: Option<String>,
    _marker: std::marker::PhantomData<T>,
}
impl HomeBuilder<HomeWithOutRoom> {
    pub fn new(name: &str) -> HomeBuilder<HomeWithOutRoom> {
        HomeBuilder {
            name: name.to_string(),
            rooms: HashMap::new(),
            last_room: None,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> HomeBuilder<T> {
    pub fn add_room<N>(mut self, name: N) -> HomeBuilder<HomeWithRoom>
    where
        N: Into<String>,
    {
        let name: String = name.into();
        self.rooms.insert(name.clone(), Room::default());
        HomeBuilder {
            name: self.name,
            rooms: self.rooms,
            last_room: Some(name),
            _marker: std::marker::PhantomData,
        }
    }
}

impl HomeBuilder<HomeWithRoom> {
    pub fn add_device<N>(mut self, name: N, device: SmartDevice) -> Self
    where
        N: Into<String>,
    {
        let last = self.last_room.as_ref().unwrap().as_str();
        self.rooms.get_mut(last).unwrap().add_device(name, device);
        self
    }
    pub fn build(self) -> Home {
        Home::new_with_rooms(self.name, self.rooms)
    }
}

#[cfg(test)]
mod tests {

    use std::io::Cursor;

    use super::*;

    #[test]
    fn test_home_builder() {
        let home = HomeBuilder::new("Home")
            .add_room("Room 1")
            .add_device(
                "Device 1",
                SmartDevice::from(SmartSocket::new(Cursor::new(Vec::new()))),
            )
            .add_room("Room 2")
            .add_device(
                "Device 2",
                SmartDevice::from(SmartSocket::new(Cursor::new(Vec::new()))),
            )
            .build();
        let room1 = home.get_room("Room 1").unwrap();
        let _device = room1.get_device("Device 1").unwrap();
        let room2 = home.get_room("Room 2").unwrap();
        let _device = room2.get_device("Device 2").unwrap();
    }
}
