#![allow(dead_code)]

use std::sync::mpsc;
use std::thread;

pub mod gui;
mod i2c;
mod serial;

#[derive(Clone, Debug)]
pub enum Event {
    I2C(i2c::I2CEvent),
    Serial(serial::SerialEvent),
    Gui(gui::GuiEvent),
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let (gui_tx, gui_rx) = mpsc::channel();
    let (serial_tx, serial_rx) = mpsc::channel();

    let clone_tx = tx.clone();

    /*thread::spawn(move || {
        serial::launch(clone_tx, serial_rx);
    });*/

    let clone_tx = tx.clone();

    thread::spawn(move || {
        gui::launch(clone_tx, gui_rx);
    });

    let mut i2c_struct = i2c::initialize(tx.clone());

    tx.send(Event::Gui(gui::GuiEvent::Log("Hello there!".to_string()))).unwrap();

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
