use std::time::Duration;
use std::thread;

mod gpio;
mod controller;


fn main() {
  let mut device = controller::L293D::new()
    .with_lspeed(2)
    .with_ldirection(3, 4)
    .with_rspeed(14)
    .with_rdirection(15, 18);

  loop {
    device.lforward();
    thread::sleep(Duration::from_millis(500));
    device.lbackward();
    thread::sleep(Duration::from_millis(500));
  }
}
