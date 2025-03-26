use crate::{
    devices::{SmartSocket, SmartThermometer},
    homes::Home,
    rooms::Room,
    SmartDevice,
};

pub trait Report {
    fn report(&self) -> String;
}

impl Report for SmartThermometer {
    fn report(&self) -> String {
        format!("Temperature: {:.2}", self.get_temperature())
    }
}

impl Report for SmartSocket {
    fn report(&self) -> String {
        format!("On: {}\t Power: {:.2}", self.is_on(), self.get_power())
    }
}

impl Report for SmartDevice {
    fn report(&self) -> String {
        match self {
            SmartDevice::SmartThermometer(thermometer) => thermometer.report(),
            SmartDevice::SmartSocket(socket) => socket.report(),
        }
    }
}

impl Report for Room {
    fn report(&self) -> String {
        let mut report = "".to_string();
        for (name, device) in self.into_iter() {
            report.push_str(&format!("- {:20}: {}\n", name, device.report()));
        }
        report
    }
}

impl Report for Home {
    fn report(&self) -> String {
        let mut report = format!("Home: {}\n", self.name);
        for (name, room) in self.into_iter() {
            report.push_str("***********\n");
            report.push_str(&format!("Room: {}\n", name));
            report.push_str(&room.report());
        }
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_thermometer() {
        let thermometer = SmartThermometer::default();
        let report = thermometer.report();
        assert!(report.contains("Temperature: 0.00"));
    }

    #[test]
    fn test_report_socket() {
        let mut socket = SmartSocket::default();
        let report = socket.report();
        assert!(report.contains("On: false\t Power: 0.00"));

        socket.switch();
        let report = socket.report();
        assert!(report.contains("On: true\t Power: 1000.00"));
    }

    #[test]
    fn report_smart_device() {
        let socket = SmartDevice::from(SmartSocket::default());
        let report = socket.report();
        assert!(report.contains("On: false\t Power: 0.00"));

        let thermometer = SmartDevice::from(SmartThermometer::default());
        let report = thermometer.report();
        assert!(report.contains("Temperature: 0.00"));
    }
    #[test]
    fn test_report_room() {
        let mut room = Room::default();
        let device = SmartDevice::from(SmartThermometer::default());
        room.add_device("Thermometer".to_string(), device);
        let device = SmartDevice::from(SmartSocket::default());
        room.add_device("Socket".to_string(), device);
        let report = room.report();
        assert!(report.contains("Thermometer"));
        assert!(report.contains("Temperature: 0.00"));
        assert!(report.contains("Socket"));
        assert!(report.contains("On: false\t Power: 0.00"));
    }
    #[test]
    fn test_report_home() {
        let mut home = Home::new("Home".to_string());
        let mut room = Room::default();
        let device = SmartDevice::from(SmartThermometer::default());
        room.add_device("Thermometer".to_string(), device);
        home.add_room("Room1".to_string(), room);

        let mut room = Room::default();
        let device = SmartDevice::from(SmartSocket::default());
        room.add_device("Socket".to_string(), device);
        home.add_room("Room2".to_string(), room);

        let report = home.report();
        assert!(report.contains("Home: Home"));
        assert!(report.contains("Room: Room1"));
        assert!(report.contains("Thermometer"));
        assert!(report.contains("Temperature: 0.00"));
        assert!(report.contains("Room: Room2"));
        assert!(report.contains("Socket"));
        assert!(report.contains("On: false\t Power: 0.00"));
    }
}
