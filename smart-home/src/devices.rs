use std::net::ToSocketAddrs;

use crate::SmartHomeError;

pub mod smartsocket;
pub mod termo;

pub trait SmartDeviceConnect {
    fn connect(address: impl ToSocketAddrs) -> Result<Self, SmartHomeError>
    where
        Self: Sized;
}
