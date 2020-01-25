use std::sync::mpsc::{Sender, Receiver};
use std::net::TcpListener;
use std::thread;
use std::io::{BufRead, BufReader, Write};
use crate::protocol::{IncomingMsg, OutgoingMsg};
use crate::{AltctrlInterface, Event, SerialEvent, gui};

pub struct Fatkhiyev {
    address: String,
}

impl Fatkhiyev {
    pub fn new<S: Into<String>>(address: S) -> Fatkhiyev {
        Fatkhiyev {
            address: address.into(),
        }
    }
}

impl AltctrlInterface for Fatkhiyev {
    fn launch(&self, sender: Sender<Event>, serial_receiver: Receiver<SerialEvent>) {
        // Create listener for a tcp connection of port 6969
        let listener = TcpListener::bind(&self.address).unwrap();

        // Block the thread until a client connects
        let (stream_rx, _addr) = listener.accept().unwrap();
        let mut stream_tx = stream_rx.try_clone().unwrap();

        // Spawn a thread for sending OutgoingMsg to the client over tcp
        thread::spawn(move || {
            for message in serial_receiver.iter() {
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
                    sender.send(Event::from(message)).unwrap();
                }

                Err(e) => {
                    let error = format!("{:?}", e);
                    sender.send(Event::Gui(gui::GuiEvent::Log(error))).unwrap();
                    break;
                }
            }
        }
    }
}

