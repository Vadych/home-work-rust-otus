use std::{
    fs,
    net::{SocketAddr, UdpSocket},
    thread,
    time::Duration,
};

const CONFIG: &str = "termo-emulator.cfg";
const TERMO_EMULATOR_IP: &str = "127.0.0.1:4322";
fn main() {
    let ip = fs::read_to_string(CONFIG)
        .expect("Can't read config file")
        .parse::<SocketAddr>()
        .expect("invalid socket address");

    let socket = UdpSocket::bind(TERMO_EMULATOR_IP).unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(1)))
        .unwrap();
    loop {
        let new_temp = rand::random_range(10.0..30.0) as f32;
        socket.send_to(&new_temp.to_be_bytes(), ip).unwrap();
        println!("New temperature: {}", new_temp);
        thread::sleep(Duration::from_secs(1));
    }
}
