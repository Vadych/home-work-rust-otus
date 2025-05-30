use crate::SmartHomeError;

use super::SmartDeviceConnect;
use std::{
    io,
    net::{SocketAddr, ToSocketAddrs, UdpSocket},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};
#[derive(Debug)]
pub struct SmartThermometer {
    temperature: Arc<Temperature>,
    finished: Arc<AtomicBool>,
}

impl SmartThermometer {
    pub fn get_temperature(&self) -> f32 {
        self.temperature.get()
    }
}

pub trait UdpLike {
    fn send_to(&mut self, buf: &[u8], target: SocketAddr) -> io::Result<usize>;
    fn recv_from(&mut self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)>;
}

impl UdpLike for UdpSocket {
    fn send_to(&mut self, buf: &[u8], target: SocketAddr) -> io::Result<usize> {
        UdpSocket::send_to(self, buf, target)
    }

    fn recv_from(&mut self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        UdpSocket::recv_from(self, buf)
    }
}

impl SmartThermometer {
    pub fn new(mut stream: impl UdpLike + Send + 'static) -> Self {
        let finished = Arc::new(AtomicBool::new(false));
        let temperature = Arc::new(Temperature::default());

        let finished_clone = finished.clone();
        let temperature_clone = temperature.clone();

        thread::spawn(move || {
            println!("Thread started");
            loop {
                if finished_clone.load(Ordering::SeqCst) {
                    return;
                }

                let mut buf = [0; 4];
                if let Err(err) = stream.recv_from(&mut buf) {
                    println!("can't receive datagram: {err}");
                }

                let val = f32::from_be_bytes(buf);
                temperature_clone.set(val);
                println!("Temperature: {val}");
                thread::sleep(std::time::Duration::from_secs(1));
            }
        });

        Self {
            temperature,
            finished,
        }
    }
}

impl SmartDeviceConnect for SmartThermometer {
    fn connect(address: impl ToSocketAddrs) -> Result<Self, SmartHomeError> {
        let socket = UdpSocket::bind(address)?;
        socket.set_read_timeout(Some(Duration::from_secs(1)))?;
        Ok(SmartThermometer::new(socket))
    }
}
impl Drop for SmartThermometer {
    fn drop(&mut self) {
        self.finished.store(true, Ordering::SeqCst);
    }
}

#[derive(Default, Debug)]
struct Temperature(Mutex<f32>);

impl Temperature {
    pub fn get(&self) -> f32 {
        let guard = match self.0.lock() {
            Ok(g) => g,
            Err(poison_error) => poison_error.into_inner(),
        };
        *guard
    }

    pub fn set(&self, val: f32) {
        let mut guard = match self.0.lock() {
            Ok(g) => g,
            Err(poison_error) => poison_error.into_inner(),
        };
        *guard = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read, Write};

    pub(crate) struct FakeUdpSocket {
        buf: Cursor<Vec<u8>>,
    }

    impl FakeUdpSocket {
        fn new() -> Self {
            Self {
                buf: Cursor::new(Vec::new()),
            }
        }
    }

    impl UdpLike for FakeUdpSocket {
        fn send_to(&mut self, buf: &[u8], _target: SocketAddr) -> io::Result<usize> {
            self.buf.set_position(0);
            self.buf.write(buf)
        }
        fn recv_from(&mut self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
            self.buf.set_position(0);
            let _ = self.buf.read(buf).expect("Error reading buffer");
            Ok((4, SocketAddr::from(([127, 0, 0, 1], 0))))
        }
    }
    #[test]
    fn test_new_thermometer() {
        let mut stream = FakeUdpSocket::new();
        let buf = 23f32.to_be_bytes();
        stream
            .send_to(&buf, SocketAddr::from(([127, 0, 0, 1], 0)))
            .unwrap();
        let termo = SmartThermometer::new(stream);
        thread::sleep(std::time::Duration::from_secs(2));
        assert_eq!(termo.get_temperature(), 23.0);
    }
}
