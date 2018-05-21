use gpio::Hardware;

pub struct MockHardware {
    led_state: bool,
}

impl Hardware for MockHardware {
    fn led_on(&mut self) {
        println!("LED on\n");
        self.led_state = true;
    }

    fn led_off(&mut self) {
        println!("LED off\n");
        self.led_state = false;
    }
}

impl MockHardware {
    pub fn new() -> MockHardware {
        let io = MockHardware {
            led_state: false,
        };

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
