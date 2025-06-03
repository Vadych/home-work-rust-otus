use smart_home::{Room, SmartThermometer, devices::SmartDeviceConnect, subscrib::Subscriber};

fn main() {
    #[derive(Debug, Default)]
    struct MyObserver {
        count: u32,
    }
    impl Subscriber for MyObserver {
        fn on_add(&mut self, name: &str) {
            self.count += 1;
            println!("Added device # {}, name: {}", self.count, name);
        }
    }

    let mut room = Room::default();
    room.subscribe(MyObserver::default());
    let thermo1 = SmartThermometer::connect("127.0.0.1:4321").unwrap();
    let thermo2 = SmartThermometer::connect("127.0.0.1:4322").unwrap();
    room.add_device("Termo 1", thermo1.into());
    room.subscribe(|name: &str| println!("Added new device {}", name));
    room.add_device("Termo 2", thermo2.into());
}
