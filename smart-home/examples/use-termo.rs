use std::{thread, time::Duration};

use smart_home::{SmartThermometer, devices::SmartDeviceConnect};

fn main() {
    let termo = SmartThermometer::connect("127.0.0.1:4321").expect("Can't connect");

    for _ in 0..10 {
        println!("Temperature: {}", termo.get_temperature());
        thread::sleep(Duration::from_secs(1));
    }
}
