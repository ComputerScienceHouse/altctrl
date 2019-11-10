#![allow(dead_code)]

use std::io;
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

mod gui;
mod i2c;
mod protocol;
mod shared;

use protocol::{IncomingMsg, OutgoingMsg};
use shared::{Event, SerialEvent};

const PORT: &str = "/dev/serial0";

pub fn launch(tx: Sender<Event>, rx: Receiver<SerialEvent>) {
    let mut serial_tx = serialport::open(PORT).expect("Failed to open serialport");

    let serial_rx = serial_tx.try_clone().unwrap();

    thread::spawn(move || {
        for message in rx.iter() {
            serial_tx
                .write_all(
                    serde_json::to_string(&OutgoingMsg::from(message))
                        .unwrap()
                        .as_bytes(),
                )
                .unwrap();
        }
    });

    let mut buf_reader = BufReader::new(serial_rx);

    loop {
        let mut content = String::new();

        match buf_reader.read_line(&mut content) {
            Ok(_) => {
                let message: IncomingMsg = serde_json::from_str(content.as_ref()).unwrap();
                tx.send(Event::from(message)).unwrap();
            }

            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),

            Err(e) => eprintln!("{:?}", e),
        }
    }
}

fn main() {
    shared::start(launch);
}
