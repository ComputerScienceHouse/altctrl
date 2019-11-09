use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::gui;
use crate::i2c;
use crate::protocol::{Button, Devices};

#[derive(Clone, Debug)]
pub enum Event {
    I2C(i2c::I2CEvent),
    Serial(SerialEvent),
    Gui(gui::GuiEvent),
}

#[derive(Clone, Debug)]
pub enum SerialEvent {
    Pressed(Button, Devices),
    Released(Button, Devices),
}

pub fn start(interface: fn(Sender<Event>, Receiver<SerialEvent>)) {
    let (tx, rx) = mpsc::channel();
    let (gui_tx, gui_rx) = mpsc::channel();
    let (serial_tx, serial_rx) = mpsc::channel();

    let clone_tx = tx.clone();

    thread::spawn(move || {
        interface(clone_tx, serial_rx);
    });

    let clone_tx = tx.clone();

    thread::spawn(move || {
        gui::launch(clone_tx, gui_rx);
    });

    let mut i2c_struct = i2c::initialize(tx.clone());

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
