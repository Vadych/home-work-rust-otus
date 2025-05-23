use std::io;

use smart_home::{SmartSocket, devices::SmartDeviceConnect};

const SOCKET_IP: &str = "127.0.0.1:4331";
fn main() {
    let client = SmartSocket::connect(SOCKET_IP).expect("Can't connect");

    loop {
        show_menu();
        let input = read_input();
        match input.as_ref() {
            "1" => {
                client.switch().unwrap();
            }
            "2" => {
                println!("Power: {}", client.get_power().unwrap());
            }
            "3" => {
                println!("Enabled: {}", client.is_on().unwrap());
            }
            _ => {
                println!("Exit...");
                break;
            }
        };

        println!("------------------\n");
    }
}

fn show_menu() {
    println!();
    println!("------------------");
    println!("Select action:");
    println!("1) switch");
    println!("2) power");
    println!("3) is enabled");
    println!("_) exit");
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
