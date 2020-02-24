extern crate sysfs_gpio;

use self::sysfs_gpio::{Direction, Pin};
use gpio::Hardware;

/// An opaque wrapper around a gpio object
pub struct Gpio {
    led_pin: sysfs_gpio::Pin,
}

impl Hardware for Gpio {
    fn led_on(&mut self) {
        self.led_pin.set_value(1).unwrap();
    }

    fn led_off(&mut self) {
        self.led_pin.set_value(0).unwrap();
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(2 + 2, 4);
    }
}
