//                    GNU GENERAL PUBLIC LICENSE
//                       Version 2, June 1991
//
// Copyright (C) 1989, 1991 Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
// Everyone is permitted to copy and distribute verbatim copies
// of this license document, but changing it is not allowed.

use std::{
  fs,
  io,
  path::Path
};

static MAIN_RS: &str=r#"#![no_std]
#![no_main]

use panic_halt as _;
use esp8266_hal::prelude::*;
use esp8266_hal::target::Peripherals;

#[entry]
fn main()-> ! {
  let dp=Peripherals::take().unwrap();
  let pins=dp.GPIO.split();
  let mut led=pins.gpio2.into_push_pull_output();
  let (mut timer1,_)=dp.TIMER.timers();

  led.set_high().unwrap();

  loop {
    timer1.delay_ms(500);
    led.toggle().unwrap();
  }
}
"#;

pub fn write_main<P: AsRef<Path>>(project_path: P)-> io::Result<()> {
  let path=project_path.as_ref().join("src/main.rs");
  fs::write(path,MAIN_RS)
}



