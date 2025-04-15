use smart_home::{room, Home, Report, Room, SmartDevice, SmartSocket, SmartThermometer};
fn main() {
    let room1 = room! {
        ("Socket1", SmartSocket),
        ("Thermometer1", SmartThermometer)
    };

    let room2 = room! {
        ("Socket2", SmartSocket),
        ("Thermometer2", SmartThermometer)
    };

    let mut home = Home::new("Home".to_string());
    home.add_room("Room1", room1);
    home.add_room("Room2", room2);

    report(home.get_room("Room1").unwrap());
    report(&home);
    println!("Switch Socket1 in Room1");
    if let SmartDevice::SmartSocket(device) = home.get_device("Room1", "Socket1").unwrap() {
        device.switch()
    }
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
