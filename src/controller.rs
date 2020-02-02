use std::collections::HashMap;

use crate::gpio::GPIOPin;

const ENA: u8 = 1;
const IN1: u8 = 2;
const OUT1: u8 = 3;
const GND1: u8 = 4;
const GND2: u8 = 5;
const OUT2: u8 = 6;
const IN2: u8 = 7;
const VCC2: u8 = 8;
const ENB: u8 = 9;
const IN3: u8 = 10;
const OUT3: u8 = 11;
const GND3: u8 = 12;
const GND4: u8 = 13;
const OUT4: u8 = 14;
const IN4: u8 = 15;
const VCC1: u8 = 16;

/// ENA  --[==()==]-- VCC1
/// IN1  --[======]-- IN4
/// OUT1 --[==L===]-- OUT4
/// GND1 --[==2===]-- GND4
/// GND2 --[==9===]-- GND3
/// OUT2 --[==3===]-- OUT3
/// IN2  --[==D===]-- IN3
/// VCC2 --[======]-- ENB
pub struct L293D {
  gpios: HashMap<u8, GPIOPin>
}

impl L293D {
  pub fn new() -> Self {
    return L293D {
      gpios: HashMap::new()
    };
  }

  /**
   * Sets the speed control gpio pin of the left hand
   * side of the chip.
   */
  pub fn with_lspeed(mut self, pin: u8) -> Self {
    self.gpios.insert(ENA, GPIOPin::new(pin).unwrap());
    return self;
  }

  /**
   * Sets the direction control pins of the left hand
   * side of the chip
   */
  pub fn with_ldirection(mut self, forward_pin: u8, backward_pin: u8) -> Self {
    self.gpios.insert(IN1, GPIOPin::new(forward_pin).unwrap());
    self.gpios.insert(IN2, GPIOPin::new(backward_pin).unwrap());
    return self;
  }

  /**
   * Sets the speed control pin of the right hand
   * side of the chip
   */
  pub fn with_rspeed(mut self, pin: u8) -> Self {
    self.gpios.insert(ENB, GPIOPin::new(pin).unwrap());
    return self;
  }

  /**
   * Sets the direction control pins of the right hand
   * side of the chip.
   */
  pub fn with_rdirection(mut self, forward_pin: u8, backward_pin: u8) -> Self {
    self.gpios.insert(IN4, GPIOPin::new(forward_pin).unwrap());
    self.gpios.insert(IN3, GPIOPin::new(backward_pin).unwrap());
    return self;
  }

  /**
   * Drives the left hand side motor forward. Use only
   * if connected device is a bi-directional motor.
   */
  pub fn lforward(&mut self) {
    self.gpios.get_mut(&IN1).unwrap().set_low();
    self.gpios.get_mut(&IN2).unwrap().set_high();
  }

  /**
   * Drives the left hand side motor forward. Use only
   * if connected device is a bi-directional motor.
   */
  pub fn lbackward(&mut self) {
    self.gpios.get_mut(&IN1).unwrap().set_high();
    self.gpios.get_mut(&IN2).unwrap().set_low();
  }

  /**
   * Drives the right hand side motor forward. Use only
   * if connected device is a bi-directional motor.
   */
  pub fn rforward(&mut self) {
    self.gpios.get_mut(&IN4).unwrap().set_low();
    self.gpios.get_mut(&IN3).unwrap().set_high();
  }

  /**
   * Drives the right hand side motor forward. Use only
   * if connected device is a bi-directional motor.
   */
  pub fn rbackward(&mut self) {
    self.gpios.get_mut(&IN4).unwrap().set_high();
    self.gpios.get_mut(&IN3).unwrap().set_low();
  }

}

