use std::time::Duration;
use std::thread;

mod gpio;

use gpio::GPIOPin;

struct Stepper {
  gpio1: GPIOPin,
  gpio2: GPIOPin,
  gpio3: GPIOPin,
  gpio4: GPIOPin
}

impl Stepper {

  pub fn new(pin1: u8, pin2: u8, pin3: u8, pin4: u8) -> Self {
    let mut gpio1 = GPIOPin::new(pin1).unwrap();
    let mut gpio2 = GPIOPin::new(pin2).unwrap();
    let mut gpio3 = GPIOPin::new(pin3).unwrap();
    let mut gpio4 = GPIOPin::new(pin4).unwrap();
    gpio1.set_output();
    gpio2.set_output();
    gpio3.set_output();
    gpio4.set_output();

    return Stepper {
      gpio1,
      gpio2,
      gpio3,
      gpio4
    }
  }

  pub fn step_left(&mut self) {
    self.gpio1.set_high();
    self.gpio2.set_low();
    self.gpio3.set_low();
    self.gpio4.set_low();
  }

  pub fn step_right(&mut self) {
    self.gpio1.set_low();
    self.gpio2.set_high();
    self.gpio3.set_low();
    self.gpio4.set_low();
  }

}

fn main() {
  let mut stepper = Stepper::new(2, 3, 4, 14);

  loop {
    stepper.step_left();
    thread::sleep(Duration::from_millis(500));
  }
}
