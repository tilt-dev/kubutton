extern crate reqwest;

use serialport::prelude::*;
use std::env;
use std::io::{self, Write};
use std::str;
use std::time::Duration;
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    println!("kubotton must takes two arguments: kubutton <path/to/serial/port> <http://url/to/hit/when/pressed>");
    process::exit(1);
  }
  let port_name = &args[1];
  let snack_url = &args[2];

  let mut settings: SerialPortSettings = Default::default();
  settings.timeout = Duration::from_millis(10);

  let baud_rate = 9600;
  settings.baud_rate = baud_rate;
  match serialport::open_with_settings(&port_name, &settings) {
    Ok(mut port) => {
      let mut serial_buf: Vec<u8> = vec![0; 1000];
      println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
      loop {
        match port.read(serial_buf.as_mut_slice()) {
          Ok(t) => {
            io::stdout().write_all(&serial_buf[..t]).unwrap();
            match str::from_utf8(&serial_buf) {
              Ok(t) => {
                if t.starts_with("1") {
                  hit_snack(snack_url);
                }
              }
              Err(e) => eprintln!("error parsing UTF string: {:?}", e),
            }
          }
          Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
          Err(e) => eprintln!("{:?}", e),
        }
      }
    }
    Err(e) => {
      eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
      ::std::process::exit(1);
    }
  }
}

fn hit_snack(snack_url: &String) {
  println!("Hitting snack");
  match reqwest::get(snack_url) {
    Ok(_t) => println!("Hit snack!"),
    Err(e) => eprintln!("error hitting snack: {:?}", e),
  }
}
