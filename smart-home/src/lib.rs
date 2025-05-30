pub mod devices;
pub mod homes;
pub mod report;
pub mod rooms;

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
