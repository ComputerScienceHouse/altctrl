extern crate serialport;

use std::io;
use std::io::{Write, BufRead, BufReader};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::i2c::types::I2CMsg;
use crate::gui::types::{GuiMsg, NewWindow};

pub mod types;

use types::{IncomingMsg, OutgoingMsg};

const PORT: &str = "/dev/serial0";

pub fn launch(
    i2c_tx: Sender<I2CMsg>,
    gui_tx: Sender<GuiMsg>,
    outgoing_rx: Receiver<OutgoingMsg>,
) {
    match serialport::open(PORT) {
        Ok(mut serial_tx) => {
            let mut serial_rx = BufReader::new(serial_tx.try_clone().unwrap());

            thread::spawn(move || loop {
                let mut line = String::new();

                match serial_rx.read_line(&mut line) {
                    Ok(_) => {
                        line.retain(|c| !c.is_whitespace());

                        //match I2CMsg::deserialize(&line[..2], &line[2..]) {
                        //   Ok(message) => i2c_tx.send(message).unwrap(),

                        //    Err(()) => eprintln!("Invalid formated message: {}", line),
                        //}
                    }

                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),

                    Err(e) => eprintln!("{:?}", e),
                }
            });

            thread::spawn(move || {
                for message in outgoing_rx.iter() {
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
