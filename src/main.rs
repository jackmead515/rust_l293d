use std::time::Duration;
use std::thread;

mod controller;


fn main() {
  let mut device = controller::L293D::new()
    .with_lpwm_speed()
    .with_ldirection_pins(3, 4)
    .with_rpwm_speed()
    .with_rdirection_pins(15, 18);

  device.set_rspeed(50.0);
  device.activate_rpwm_speed();

  loop {
    device.rforward();
    thread::sleep(Duration::from_millis(500));
    device.rbackward();
    thread::sleep(Duration::from_millis(500));
  }
}
