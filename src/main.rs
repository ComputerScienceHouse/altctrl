#![allow(dead_code)]

use std::sync::mpsc;
use std::thread;
use event::*;
use gui::GuiEvent;

pub mod protocol;
mod gui;
mod i2c;
mod serial;
mod event;


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

    tx.send(Event::Gui(GuiEvent::Log("Hello there!".to_string()))).unwrap();
    tx.send(Event::Gui(GuiEvent::Log("I am a boy".to_string()))).unwrap();
    tx.send(Event::Gui(GuiEvent::Log("I have ligma".to_string()))).unwrap();
    tx.send(Event::Gui(GuiEvent::Log("And herpes.".to_string()))).unwrap();
    tx.send(Event::Gui(GuiEvent::Log("You might want to exit.".to_string()))).unwrap();
    tx.send(Event::Gui(GuiEvent::Log("Like... NOW!".to_string()))).unwrap();
    tx.send(Event::Gui(GuiEvent::Log("too late...".to_string()))).unwrap(); 
    
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
