extern crate sysfs_gpio;

use self::sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

/// An opaque wrapper around a gpio object
pub struct Gpio {
    led_pin: sysfs_gpio::Pin,
}

impl Gpio {
    pub fn new() -> Gpio {
        println!("Gpio::new()");
        let io = Gpio {
            led_pin: Pin::new(4),
        };

        io.led_pin.export().unwrap();
        io.led_pin.set_direction(Direction::Out).unwrap();
        io.led_pin.set_value(1).unwrap();

        io
    }

    pub fn led_on(&self) {
        self.led_pin.set_value(1).unwrap();
    }

    pub fn led_off(&self) {
        self.led_pin.set_value(0).unwrap();
    }

    pub fn led_toggle(&self, duration: Duration) {
        loop {
            self.led_on();
            sleep(duration);
            self.led_off();
            sleep(duration);
        }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(2 + 2, 4);
    }
}
