use smart_home::{
    Room, SmartDevice, SmartThermometer, devices::SmartDeviceConnect, report::Reporter,
};

fn main() {
    let room = Room::default();

    let thermo1 = SmartThermometer::connect("127.0.0.1:4321").unwrap();
    let thermo2 = SmartThermometer::connect("127.0.0.1:4322").unwrap();
    let device = SmartDevice::from(SmartThermometer::connect("127.0.0.1:4323").unwrap());

    Reporter::new()
        .add_for_report(&room)
        .add_for_report(&device)
        .add_for_report(&thermo1)
        .add_for_report(&thermo2)
        .report();
}
