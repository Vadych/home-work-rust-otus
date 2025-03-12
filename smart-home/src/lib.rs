pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, PartialEq)]
pub enum SmartDevice {
    Thermometer(SmartThermometer),
    Socket(SmartSocket),
}
impl SmartDevice {
    pub fn print_state(&self) {
        match self {
            SmartDevice::Thermometer(thermometer) => println!("{}", thermometer.get_state()),
            SmartDevice::Socket(socket) => println!("{}", socket.get_state()),
        }
    }
}
#[derive(Debug, PartialEq)]
pub struct SmartThermometer {
    pub name: String,
    temperature: f32,
}

impl SmartThermometer {
    pub fn new(name: String) -> SmartThermometer {
        let temperature = 0.0;
        SmartThermometer { name, temperature }
    }
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
    fn get_state(&self) -> String {
        format!("Name: {}\t Temperature: {:.2}", self.name, self.temperature)
    }
}

#[derive(Debug, PartialEq)]
pub struct SmartSocket {
    pub name: String,
    is_on: bool,
    power: f32,
}

impl SmartSocket {
    pub fn new(name: String) -> SmartSocket {
        let is_on = false;
        let power = 0.0;
        SmartSocket { name, is_on, power }
    }
    pub fn switch(&mut self) {
        self.is_on = !self.is_on;
        self.power = if self.is_on { 1000.0 } else { 0.0 };
    }
    pub fn get_power(&self) -> f32 {
        self.power
    }
    fn get_state(&self) -> String {
        format!(
            "Name: {}\t On: {}\t Power: {:.2}",
            self.name, self.is_on, self.power
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Room {
    pub name: String,
    devices: Vec<SmartDevice>,
}

impl Room {
    pub fn new(name: String, devices: Vec<SmartDevice>) -> Room {
        Room { name, devices }
    }
    pub fn get_device(&self, index: usize) -> &SmartDevice {
        self.devices.get(index).unwrap()
    }
    pub fn get_device_mut(&mut self, index: usize) -> &mut SmartDevice {
        self.devices.get_mut(index).unwrap()
    }
    pub fn print_report(&self) {
        println!("Room: {}, devices:", self.name);
        for device in self.devices.iter() {
            print!("- ");
            device.print_state();
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Home {
    pub name: String,
    rooms: Vec<Room>,
}
impl Home {
    pub fn new(name: String, rooms: Vec<Room>) -> Home {
        Home { name, rooms }
    }
    pub fn get_room(&self, index: usize) -> &Room {
        self.rooms.get(index).unwrap()
    }
    pub fn get_room_mut(&mut self, index: usize) -> &mut Room {
        self.rooms.get_mut(index).unwrap()
    }
    pub fn print_report(&self) {
        println!("Home: {}", self.name);
        for room in self.rooms.iter() {
            println!("***********");
            room.print_report();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_thermometer() {
        let result = SmartThermometer::new("ThermTest".to_string());
        let therm = SmartThermometer {
            name: "ThermTest".to_string(),
            temperature: 0.0,
        };
        assert_eq!(result, therm);
    }
    #[test]
    fn get_temperature_thermometer() {
        let therm = SmartThermometer::new("ThermTest".to_string());
        assert_eq!(therm.get_temperature(), 0.0);
    }
    #[test]
    fn get_state_thermometer() {
        let therm = SmartThermometer::new("ThermTest".to_string());
        assert_eq!(therm.get_state(), "Name: ThermTest\t Temperature: 0.00");
    }

    #[test]
    fn new_socket() {
        let result = SmartSocket::new("SocketTest".to_string());
        let socket = SmartSocket {
            name: "SocketTest".to_string(),
            is_on: false,
            power: 0.0,
        };
        assert_eq!(result, socket);
    }
    #[test]
    fn switch_socket() {
        let mut socket = SmartSocket::new("SocketTest".to_string());
        assert_eq!(socket.is_on, false);
        socket.switch();
        assert_eq!(socket.is_on, true);
    }
    #[test]
    fn get_power_socket() {
        let mut socket = SmartSocket::new("SocketTest".to_string());
        assert_eq!(socket.get_power(), 0.0);
        socket.switch();
        assert_eq!(socket.get_power(), 1000.0);
    }

    #[test]
    fn get_state_socket() {
        let mut socket = SmartSocket::new("SocketTest".to_string());
        assert_eq!(
            socket.get_state(),
            "Name: SocketTest\t On: false\t Power: 0.00"
        );
        socket.switch();
        assert_eq!(
            socket.get_state(),
            "Name: SocketTest\t On: true\t Power: 1000.00"
        );
    }
    #[test]
    fn new_room() {
        let result = Room::new(
            "RoomTest".to_string(),
            vec![SmartDevice::Thermometer(SmartThermometer::new(
                "ThermTest".to_string(),
            ))],
        );
        let room = Room {
            name: "RoomTest".to_string(),
            devices: vec![SmartDevice::Thermometer(SmartThermometer::new(
                "ThermTest".to_string(),
            ))],
        };
        assert_eq!(result, room);
    }
    #[test]
    fn get_device_room() {
        let room = Room::new(
            "RoomTest".to_string(),
            vec![SmartDevice::Thermometer(SmartThermometer::new(
                "ThermTest".to_string(),
            ))],
        );
        assert_eq!(room.get_device(0), room.devices.get(0).unwrap());
    }
    #[test]
    fn get_device_mut_room() {
        // Не знаю, как протестировать этот функционал.
        // Получается нужно сравнить две мутабельные ссылки, которые не могут существовать одновременно
    }
    #[test]
    fn new_home() {
        let result = Home::new(
            "HomeTest".to_string(),
            vec![Room::new(
                "RoomTest".to_string(),
                vec![SmartDevice::Thermometer(SmartThermometer::new(
                    "ThermTest".to_string(),
                ))],
            )],
        );
        let home = Home {
            name: "HomeTest".to_string(),
            rooms: vec![Room::new(
                "RoomTest".to_string(),
                vec![SmartDevice::Thermometer(SmartThermometer::new(
                    "ThermTest".to_string(),
                ))],
            )],
        };
        assert_eq!(result, home);
    }
    #[test]
    pub fn get_room_home() {
        let home = Home::new(
            "HomeTest".to_string(),
            vec![Room::new(
                "RoomTest".to_string(),
                vec![SmartDevice::Thermometer(SmartThermometer::new(
                    "ThermTest".to_string(),
                ))],
            )],
        );
        assert_eq!(home.get_room(0), home.rooms.get(0).unwrap())
    }
}
