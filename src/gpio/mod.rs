use std::thread::sleep;
use std::time::Duration;

// TODO(smklein): ... there has to be a better way to do
// mocking. TL;DR: I want to use "real hardware" on
// my ARM targets, but mock hardware on my development
// machine.

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
mod gpio;
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub fn get_hardware() -> gpio::Gpio {
    gpio::Gpio::new()
}

#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
mod mock;
#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
pub fn get_hardware() -> mock::MockHardware {
    mock::MockHardware::new()
}

pub trait Hardware {
    fn led_on(&mut self);
    fn led_off(&mut self);

    fn led_toggle(&mut self, duration: Duration) {
        loop {
            self.led_on();
            sleep(duration);
            self.led_off();
            sleep(duration);
        }
    }
}
