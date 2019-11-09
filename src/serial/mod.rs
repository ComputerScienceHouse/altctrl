extern crate serialport;

use std::io;
use std::io::{Write, BufRead, BufReader};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::event::Event;
use crate::protocol::{NewWindow,Button};

#[derive(Clone, Debug)]
pub enum SerialEvent {
    Pressed(Button),
    Released(Button),
}

const PORT: &str = "/dev/zero";

pub fn launch(tx: Sender<Event>, rx: Receiver<SerialEvent>) {
    match serialport::open(PORT) {
        Ok(mut serial_tx) => {
            let mut serial_rx = BufReader::new(serial_tx.try_clone().unwrap());

            thread::spawn(move || loop {
                let mut line = String::new();

                match serial_rx.read_line(&mut line) {
                    Ok(_) => {
                        // Handle new messages next
                    }

                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),

                    Err(e) => eprintln!("{:?}", e),
                }
            });

            thread::spawn(move || {
                for message in rx.iter() {
                    //serial_tx.write_all(message.serialize().as_bytes()).unwrap();
                }
            });
        }

        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", PORT, e);
            ::std::process::exit(1);
        }
    }
}
