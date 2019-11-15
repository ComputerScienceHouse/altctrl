// Binary file using a tcp communication protocol to the client

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use altctrl::gui;
use altctrl::{Event, SerialEvent};
use altctrl::protocol::{IncomingMsg, OutgoingMsg};

// Launch function for an interface module using i2c for communication
pub fn launch(tx: Sender<Event>, rx: Receiver<SerialEvent>) {
    // Create listener for a tcp connection of port 6969
    let listener = TcpListener::bind("0.0.0.0:6969").unwrap();

    // Block the thread until a client connects
    let (stream_rx, _addr) = listener.accept().unwrap();
    let mut stream_tx = stream_rx.try_clone().unwrap();

    // Spawn a thread for sending OutgoingMsg to the client over tcp
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

    // Read data over tcp and parse that data into IncomingMsg in the system
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

// Launch the main thread using a tcp interface
fn main() {
    altctrl::start(launch);
}
