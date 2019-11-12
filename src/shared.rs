use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::gui;
use crate::i2c;
use crate::protocol::{Button, Device, IncomingMsg};

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
    Pressed(Device, Button),
    Released(Device, Button),
}

// Main thread of altctrl
// Handles the setup of all the modules in the project
// Handles message schedule
pub fn start(interface: fn(Sender<Event>, Receiver<SerialEvent>)) {
    //Outcoming message channel for all modules
    let (tx, rx) = mpsc::channel();

    //Incoming message channels for specific modules
    let (gui_tx, gui_rx) = mpsc::channel();
    let (serial_tx, serial_rx) = mpsc::channel();

    let clone_tx = tx.clone();

    //Launch the interface module
    thread::spawn(move || {
        interface(clone_tx, serial_rx);
    });

    let clone_tx = tx.clone();

    //Launch the gui module
    thread::spawn(move || {
        gui::launch(clone_tx, gui_rx);
    });

    //Initalizes the data structure for the i2c module
    let mut i2c_struct = i2c::initialize(tx.clone());

    //Handles message scheduling in a loop
    loop {
        for event in rx.iter() {
            match event {
                Event::I2C(i2c_event) => i2c::handle(i2c_event, &mut i2c_struct),
                Event::Serial(serial_event) => serial_tx.send(serial_event).unwrap(),
                Event::Gui(gui_event) => gui_tx.send(gui_event).unwrap(),
            }
        }
    }
}
