use std::{thread, io};
use std::sync::mpsc::{self, Receiver, Sender};

pub mod gui;
pub mod i2c;
pub mod protocol;

use protocol::{Port, Device, IncomingMsg, OutgoingMsg};
use serialport::prelude::*;
use std::time::Duration;
use std::io::{BufReader, BufRead, Write};
use std::net::TcpListener;

// Represents all messages sent between modules
#[derive(Clone, Debug)]
pub enum Event {
    I2C(i2c::I2CEvent),
    Serial(SerialEvent),
    Gui(gui::GuiEvent),
}

impl From<IncomingMsg> for Event {
    fn from(message: IncomingMsg) -> Self {
        match message {
            IncomingMsg::CreateWindow(new_window) => {
                Event::Gui(gui::GuiEvent::CreateWindow(new_window))
            }
            IncomingMsg::DestroyWindow(id) => Event::Gui(gui::GuiEvent::DestroyWindow(id)),
            IncomingMsg::On(button, device) => Event::I2C(i2c::I2CEvent::On(button, device)),
            IncomingMsg::Off(button, device) => Event::I2C(i2c::I2CEvent::Off(button, device)),
        }
    }
}

// Represents a message sent to the interface module
// These messages are usually sent to the client
#[derive(Clone, Debug)]
pub enum SerialEvent {
    Pressed(Device, Port),
    Released(Device, Port),
}

// Default serial port location on the raspberry pi
const PORT: &str = "/dev/ttyGS0";

pub trait AltctrlInterface {
    fn launch(&self, sender: Sender<Event>, serial_receiver: Receiver<SerialEvent>);
}

pub struct Chungo;

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
            serialport::open_with_settings(PORT, &s).expect("Failed to open serialport");
        let serial_read = serial_write.try_clone().unwrap();

        // Spawn a thread for sending OutgoingMsg to the client over serial
        thread::spawn(move || {
            for message in serial_receiver.iter() {
                serial_write
                    .write_all(
                        serde_json::to_string(&OutgoingMsg::from(message))
                            .unwrap()
                            .as_bytes(),
                    )
                    .unwrap();
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

pub struct Fatkhiyev;

impl AltctrlInterface for Fatkhiyev {
    fn launch(&self, sender: Sender<Event>, serial_receiver: Receiver<SerialEvent>) {
        // Create listener for a tcp connection of port 6969
        let listener = TcpListener::bind("0.0.0.0:6969").unwrap();

        // Block the thread until a client connects
        let (stream_rx, _addr) = listener.accept().unwrap();
        let mut stream_tx = stream_rx.try_clone().unwrap();

        // Spawn a thread for sending OutgoingMsg to the client over tcp
        thread::spawn(move || {
            for message in serial_receiver.iter() {
                stream_tx.write_all(
                    serde_json::to_string(&OutgoingMsg::from(message))
                        .unwrap()
                        .as_bytes())
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

pub struct Garfanzo;

impl AltctrlInterface for Garfanzo {
    fn launch(&self, sender: Sender<Event>, serial_receiver: Receiver<SerialEvent>) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(command) => {
                    let command = command.split(",").collect::<Vec<&str>>();
                    if command.len() >= 1 {
                        match command[0].as_ref() {
                            "log" => {
                                sender.send(Event::Gui(gui::GuiEvent::Log(command[1].to_string()))).unwrap();
                            },
                            "window" => {
                                match command[1].as_ref() {
                                    "new" => {
                                        if command.len() == 8 {
                                            sender.send(
                                                Event::Gui(
                                                    gui::GuiEvent::Log(
                                                        format!("Creating window, \"{}\"", command[2]).to_string()))).unwrap();
                                            let window = protocol::NewWindow {
                                                id: command[2].to_string(),
                                                content: command[3].to_string(),
                                                x_pos: command[4].parse::<i32>().unwrap(),
                                                y_pos: command[5].parse::<i32>().unwrap(),
                                                width: command[6].parse::<i32>().unwrap(),
                                                height: command[7].parse::<i32>().unwrap()
                                            };
                                            sender.send(Event::Gui(gui::GuiEvent::CreateWindow(window))).unwrap();
                                        }
                                    },
                                    "close" => {
                                        let window = command[2].to_string();
                                        sender.send(Event::Gui(gui::GuiEvent::DestroyWindow(window))).unwrap();
                                    },
                                    "list" => {
                                        sender.send(Event::Gui(gui::GuiEvent::List())).unwrap();
                                    },
                                    _ => {
                                        sender.send(Event::Gui(gui::GuiEvent::Log(format!("Invalid command received. ({}) Please enter a window subcommand. (new, close, list)", command[1]).to_string()))).unwrap();
                                    },
                                }
                            },
                            "clear" => {
                                sender.send(Event::Gui(gui::GuiEvent::Clear())).unwrap();
                            },
                            "help" => {
                                sender.send(Event::Gui(gui::GuiEvent::Log("(log, window(id, content, x_pos, y_pos, width, height), clear, help) Separate arguments with \',\'".to_string()))).unwrap();
                            },
                            _ => {
                                sender.send(Event::Gui(gui::GuiEvent::Log("Invalid command received.".to_string()))).unwrap();
                            },

                        }
                    } else {
                        sender.send(Event::Gui(gui::GuiEvent::Log("Invalid command received.".to_string()))).unwrap();
                    }
                },
                _ => {
                    println!("*** OH F*CK!!! ***");
                }
            }
        }
    }
}
