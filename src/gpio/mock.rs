use gpio::Hardware;

#[derive(Default)]
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
        MockHardware { led_state: false }
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
