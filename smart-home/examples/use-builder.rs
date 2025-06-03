use smart_home::{HomeBuilder, Report, SmartThermometer, devices::SmartDeviceConnect};
fn main() {
    let home = HomeBuilder::new("Home")
        .add_room("Room 1")
        .add_device(
            "Termo 1",
            SmartThermometer::connect("127.0.0.1:4321").unwrap().into(),
        )
        .add_device(
            "Termo 2",
            SmartThermometer::connect("127.0.0.1:4322").unwrap().into(),
        )
        .add_room("Room 2")
        .add_device(
            "Termo 3",
            SmartThermometer::connect("127.0.0.1:4323").unwrap().into(),
        )
        .add_device(
            "Termo 4",
            SmartThermometer::connect("127.0.0.1:4324").unwrap().into(),
        )
        .build();

    println!("{}", home.report())
}
