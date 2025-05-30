use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener},
};

use smart_home::devices::smartsocket::{SocketCommand, SocketResponse};

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();

    let server_address = args
        .next()
        .expect("Need ip address")
        .parse::<SocketAddr>()
        .expect("invalid socket address");
    let listener = TcpListener::bind(server_address).expect("can't bind tcp listener");
    let mut smart_socket = MockScoket::new();
    while let Some(connection) = listener.incoming().next() {
        let mut stream = match connection {
            Ok(conn) => conn,
            Err(err) => {
                println!("can't receive connection: {err}");
                continue;
            }
        };

        let peer = stream
            .peer_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| "unknown".into());
        println!("Peer '{peer}' connected");

        let mut in_buffer = [0u8];
        while stream.read_exact(&mut in_buffer).is_ok() {
            let response = smart_socket.process_command(in_buffer[0].into());
            let response_buf: [u8; 5] = response.into();
            if stream.write_all(&response_buf).is_err() {
                break;
            };
        }

        println!("Connection with {peer} lost. Waiting for new connections...");
    }
}

struct MockScoket {
    is_on: bool,
    power: f32,
}

impl MockScoket {
    pub fn new() -> Self {
        Self {
            is_on: false,
            power: 0.0,
        }
    }
    pub fn process_command(&mut self, command: SocketCommand) -> SocketResponse {
        match command {
            SocketCommand::Switch => {
                self.is_on = !self.is_on;
                self.power = if self.is_on { 1000. } else { 0.0 };
                SocketResponse::On(self.is_on)
            }
            SocketCommand::GetPower => SocketResponse::Power(self.power),
            SocketCommand::IsOn => SocketResponse::On(self.is_on),
            _ => SocketResponse::Unknown,
        }
    }
}
