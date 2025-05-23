use crate::SmartHomeError;

use super::SmartDeviceConnect;
use std::{
    cell::RefCell,
    fmt::Debug,
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
};
pub enum SocketCommand {
    Switch,
    GetPower,
    IsOn,
    Unknown,
}

impl From<SocketCommand> for u8 {
    fn from(cmd: SocketCommand) -> Self {
        match cmd {
            SocketCommand::Switch => 0,
            SocketCommand::IsOn => 1,
            SocketCommand::GetPower => 2,
            SocketCommand::Unknown => 255,
        }
    }
}

impl From<u8> for SocketCommand {
    fn from(value: u8) -> Self {
        match value {
            0 => SocketCommand::Switch,
            1 => SocketCommand::IsOn,
            2 => SocketCommand::GetPower,
            _ => SocketCommand::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SocketResponse {
    On(bool),
    Power(f32),
    Unknown,
}

impl From<SocketResponse> for [u8; 5] {
    fn from(response: SocketResponse) -> Self {
        let mut buffer = [0u8; 5];
        match response {
            SocketResponse::On(is_on) => {
                buffer[0] = 0;
                buffer[1] = is_on as u8;
            }
            SocketResponse::Power(power) => {
                buffer[0] = 1;
                buffer[1..].copy_from_slice(&power.to_be_bytes())
            }
            SocketResponse::Unknown => buffer[0] = 255,
        }
        buffer
    }
}

impl From<[u8; 5]> for SocketResponse {
    fn from(bytes: [u8; 5]) -> Self {
        match bytes {
            [0, ..] => Self::On(bytes[1] != 0u8),
            [1, ..] => {
                let mut buf = [0u8; 4];
                buf.copy_from_slice(&bytes[1..]);
                Self::Power(f32::from_be_bytes(buf))
            }
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct SmartSocket {
    stream: RefCell<Box<dyn ReadWrite>>,
}
impl SmartSocket {
    pub fn new(stream: impl ReadWrite + 'static) -> Self {
        Self {
            stream: RefCell::new(Box::new(stream)),
        }
    }

    pub(crate) fn run_command(
        &self,
        command: SocketCommand,
    ) -> Result<SocketResponse, SmartHomeError> {
        self.stream.borrow_mut().write_all(&[command.into()])?;
        let mut buffer = [0u8; 5];
        self.stream.borrow_mut().read_exact(&mut buffer)?;
        Ok(buffer.into())
    }
}
impl SmartDeviceConnect for SmartSocket {
    fn connect(address: impl ToSocketAddrs) -> Result<Self, SmartHomeError> {
        let stream = TcpStream::connect(address)?;
        Ok(SmartSocket::new(stream))
    }
}

impl SmartSocket {
    pub fn switch(&self) -> Result<(), SmartHomeError> {
        match self.run_command(SocketCommand::Switch) {
            Ok(SocketResponse::On(_)) => Ok(()),
            Err(e) => Err(e),
            _ => unreachable!("Invalid response for switch command"),
        }
    }
    pub fn get_power(&self) -> Result<f32, SmartHomeError> {
        match self.run_command(SocketCommand::GetPower) {
            Ok(SocketResponse::Power(power)) => Ok(power),
            Err(e) => Err(e),
            _ => unreachable!("Invalid response for power command"),
        }
    }
    pub fn is_on(&self) -> Result<bool, SmartHomeError> {
        match self.run_command(SocketCommand::IsOn) {
            Ok(SocketResponse::On(is_on)) => Ok(is_on),
            Err(e) => Err(e),
            _ => unreachable!("Invalid response for IsOn command"),
        }
    }
}

pub trait ReadWrite: Read + Write + Debug {}

impl<T: Read + Write + Debug> ReadWrite for T {}

#[cfg(test)]
mod tests {

    use super::*;
    #[derive(Debug)]
    pub(crate) struct FakeSocket {
        response: SocketResponse,
    }

    impl Write for FakeSocket {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl Read for FakeSocket {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let resp: [u8; 5] = self.response.into();
            buf.copy_from_slice(&resp);
            Ok(5)
        }
    }
    impl Default for FakeSocket {
        fn default() -> Self {
            Self {
                response: SocketResponse::On(true),
            }
        }
    }

    #[test]
    fn test_socket_command() {
        let smart_socket = SmartSocket::new(FakeSocket::default());
        let response = smart_socket.run_command(SocketCommand::Switch).unwrap();
        assert_eq!(response, SocketResponse::On(true));
    }
}
