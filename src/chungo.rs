use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use std::{thread, io};
use std::io::{BufRead, BufReader};

use serialport::{SerialPortSettings, DataBits, FlowControl, Parity, StopBits};
use crate::{AltctrlInterface, Event, SerialEvent};
use crate::protocol::{IncomingMsg, OutgoingMsg};

pub struct Chungo {
    port: String,
}

impl Chungo {
    pub fn new<S: Into<String>>(port: S) -> Chungo {
        Chungo {
            port: port.into(),
        }
    }
}

impl AltctrlInterface for Chungo {
    fn launch(&self, sender: Sender<Event>, serial_receiver: Receiver<SerialEvent>) {
        // Open the serial port
        let s = SerialPortSettings {
            baud_rate: 115_200,
            data_bits: DataBits::Eight,
            flow_control: FlowControl::None,
            parity: Parity::None,
            stop_bits: StopBits::One,
            timeout: Duration::from_secs(1),
        };

        let mut serial_write =
            serialport::open_with_settings(&self.port, &s).expect("Failed to open serialport");
        let serial_read = serial_write.try_clone().unwrap();

        // Spawn a thread for sending OutgoingMsg to the client over serial
        thread::spawn(move || {
            for message in serial_receiver.iter() {
                let string = format!(
                    "{}\n",
                    serde_json::to_string(&OutgoingMsg::from(message)).unwrap()
                );

                serial_write.write_all(string.as_bytes()).unwrap();
            }
        });

        let mut buf_reader = BufReader::new(serial_read);

        // Read data over serial and parse that data into IncomingMsg in the system
        loop {
            let mut content = String::new();

            match buf_reader.read_line(&mut content) {
                Ok(_) => {
                    let message: IncomingMsg = serde_json::from_str(content.as_ref()).unwrap();
                    sender.send(Event::from(message)).unwrap();
                }

                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),

                Err(e) => eprintln!("{:?}", e),
            }
        }
    }
}
