use crate::{
    SmartDevice,
    devices::{smartsocket::SmartSocket, termo::SmartThermometer},
    homes::Home,
    rooms::Room,
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
        format!(
            "On: {}\t Power: {:.2}",
            self.is_on().expect("Error reading socket for report"),
            self.get_power().expect("Error reading socket for report")
        )
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
        let mut report = "Room".to_string();
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

pub struct Reporter<'a> {
    reporters: Vec<&'a dyn Report>,
}

impl<'a> Reporter<'a> {
    pub fn new() -> Self {
        Reporter {
            reporters: Vec::new(),
        }
    }
    pub fn add_for_report(mut self, reporter: &'a dyn Report) -> Self {
        self.reporters.push(reporter);
        self
    }
    pub fn report(self) {
        for reporter in self.reporters {
            println!("{}", reporter.report());
        }
    }
}

impl Default for Reporter<'_> {
    fn default() -> Self {
        Self::new()
    }
}
