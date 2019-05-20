use serialport::prelude::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::str;
use std::time::Duration;

fn main() {
  let args: Vec<String> = env::args().collect();
  let port_name = &args[1];
  let file_name = &args[2];

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
                  create_or_delete_file(file_name);
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

fn create_or_delete_file(file_name: &String) {
  println!("Checking if file exists");
  let p = Path::new(file_name);

  if p.exists() {
    println!("It exists. Deleting it.");
    match fs::remove_file(p) {
      Ok(_) => println!("Deleted file"),
      Err(e) => eprintln!("{:?}", e),
    }
  } else {
    println!("It does not exist. Creating it.");
    match File::create(p) {
      Ok(_) => println!("Created file"),
      Err(e) => eprintln!("{:?}", e),
    }
  }
}
