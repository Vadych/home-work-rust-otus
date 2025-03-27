pub mod devices;
pub mod homes;
pub mod report;
pub mod rooms;

pub use devices::{SmartSocket, SmartThermometer};
pub use homes::Home;
pub use report::Report;
pub use rooms::Room;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum SmartHomeError {
    DeviceNotFound(String),
    RoomNotFound(String),
}

impl std::fmt::Display for SmartHomeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartHomeError::DeviceNotFound(name) => write!(f, "Device {} not found", name),
            SmartHomeError::RoomNotFound(name) => write!(f, "Room {} not found", name),
        }
    }
}

impl std::error::Error for SmartHomeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_thermometer() {
        let thermometer = SmartThermometer::default();
        let device: SmartDevice = SmartDevice::from(thermometer);
        assert!(matches!(device, SmartDevice::SmartThermometer(_)));
    }

    #[test]
    fn test_from_socket() {
        let socket = SmartSocket::default();
        let device: SmartDevice = SmartDevice::from(socket);
        assert!(matches!(device, SmartDevice::SmartSocket(_)));
    }
}
