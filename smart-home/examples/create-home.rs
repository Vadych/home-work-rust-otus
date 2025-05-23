use std::thread;

use smart_home::devices::SmartDeviceConnect;
use smart_home::{Home, Report, Room, SmartDevice, SmartSocket, SmartThermometer, room};

const TERMO_IP: &str = "127.0.0.1:4321";
const SOCKET_IP: &str = "127.0.0.1:4331";

fn main() {
    let room1 = room! {
        ("Socket1", SmartSocket, SOCKET_IP)
    };

    let room2 = room! {
        ("Thermometer2", SmartThermometer, TERMO_IP)
    };

    let mut home = Home::new("Home".to_string());
    home.add_room("Room1", room1);
    home.add_room("Room2", room2);
    thread::sleep(std::time::Duration::from_secs(1)); // Что бы термометр успел обновиться

    report(home.get_room("Room1").expect("Room1 not found"));
    report(&home);
    println!("Switch Socket1 in Room1");
    if let SmartDevice::SmartSocket(device) = home
        .get_device("Room1", "Socket1")
        .expect("Socket1 not found")
    {
        device.switch().expect("Can't switch Socket1");
    }

    if let SmartDevice::SmartThermometer(d) = home.get_device("Room2", "Thermometer2").unwrap() {
        println!("***Temperature: {}", d.get_temperature());
    }

    thread::sleep(std::time::Duration::from_secs(1)); // Что бы термометр успел обновиться

    report(home.get_device("Room1", "Socket1").unwrap());
    report(&home);
    match home.get_device("Room3", "Socket1") {
        Ok(device) => {
            report(device);
        }
        Err(err) => println!("{}", err),
    }
    match home.get_device("Room1", "Socket2") {
        Ok(device) => {
            report(device);
        }
        Err(err) => println!("{}", err),
    }
}

fn report<T: Report>(obj: &T) {
    println!("{}", obj.report())
}
