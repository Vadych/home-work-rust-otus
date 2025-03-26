#[derive(Debug, PartialEq)]
pub struct SmartThermometer {
    temperature: f32,
}
impl Default for SmartThermometer {
    fn default() -> Self {
        Self { temperature: 0.0 }
    }
}

impl SmartThermometer {
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
}
#[derive(Debug, PartialEq)]
pub struct SmartSocket {
    is_on: bool,
    power: f32,
}
impl Default for SmartSocket {
    fn default() -> Self {
        Self {
            is_on: false,
            power: 0.0,
        }
    }
}

impl SmartSocket {
    pub fn switch(&mut self) {
        self.is_on = !self.is_on;
        self.power = if self.is_on { 1000.0 } else { 0.0 };
    }
    pub fn get_power(&self) -> f32 {
        self.power
    }
    pub fn is_on(&self) -> bool {
        self.is_on
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_thermometer() {
        let result = SmartThermometer::default();
        let therm = SmartThermometer { temperature: 0.0 };
        assert_eq!(result, therm);
    }
    #[test]
    fn test_get_temperature_thermometer() {
        let therm = SmartThermometer::default();
        assert_eq!(therm.get_temperature(), 0.0);
    }
    #[test]
    fn test_new_socket() {
        let result = SmartSocket::default();
        let socket = SmartSocket {
            is_on: false,
            power: 0.0,
        };
        assert_eq!(result, socket);
    }
    #[test]
    fn test_switch_socket() {
        let mut socket = SmartSocket::default();
        assert_eq!(socket.is_on, false);
        socket.switch();
        assert_eq!(socket.is_on, true);
    }
    #[test]
    fn test_get_power_socket() {
        let mut socket = SmartSocket::default();
        assert_eq!(socket.get_power(), 0.0);
        socket.switch();
        assert_eq!(socket.get_power(), 1000.0);
    }
}
