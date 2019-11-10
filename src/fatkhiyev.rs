use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

mod gui;
mod i2c;
mod protocol;
mod shared;

use protocol::{IncomingMsg, OutgoingMsg};
use shared::{Event, SerialEvent};

pub fn launch(tx: Sender<Event>, rx: Receiver<SerialEvent>) {
    let lister = TcpListener::bind("0.0.0.0:6969").unwrap();

    let (stream_rx, _addr) = lister.accept().unwrap();
    let mut stream_tx = stream_rx.try_clone().unwrap();

    thread::spawn(move || {
        for message in rx.iter() {
            stream_tx
                .write_all(
                    serde_json::to_string(&OutgoingMsg::from(message))
                        .unwrap()
                        .as_bytes(),
                )
                .unwrap();
        }
    });

    let mut buf_reader = BufReader::new(stream_rx);

    loop {
        let mut content = String::new();

        match buf_reader.read_line(&mut content) {
            Ok(_) => {
                let message: IncomingMsg = serde_json::from_str(content.as_ref()).unwrap();
                tx.send(Event::from(message)).unwrap();
            }

            Err(e) => {
                let error = format!("{:?}", e);
                tx.send(Event::Gui(gui::GuiEvent::Log(error))).unwrap();
                break;
            }
        }
    }
}

fn main() {
    shared::start(launch);
}