use smart_home::{Home, Room, SmartDevice, SmartSocket, SmartThermometer};
fn main() {
    let socket1 = SmartDevice::Socket(SmartSocket::new("Socket1".to_string()));
    let term1 = SmartDevice::Thermometer(SmartThermometer::new("Therm1".to_string()));
    let socket2 = SmartDevice::Socket(SmartSocket::new("Socket2".to_string()));
    let term2 = SmartDevice::Thermometer(SmartThermometer::new("Therm2".to_string()));

    let room1 = Room::new("Room1".to_string(), vec![socket1, term1]);
    let room2 = Room::new("Room2".to_string(), vec![socket2, term2]);

    let mut home = Home::new("Home".to_string(), vec![room1, room2]);
    home.print_report();

    if let SmartDevice::Socket(socket) = home.get_room_mut(1).get_device_mut(0) {
        socket.switch();
        println!("***\nSwitching socket {}\n***", socket.name);
    }

    home.print_report();
}
