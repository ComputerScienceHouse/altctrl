#![allow(dead_code)]

use std::sync::mpsc;
use std::thread;

mod gui;
mod i2c;
mod serial;

fn main() {
    let (i2c_tx, i2c_rx) = mpsc::channel();
    let (gui_tx, gui_rx) = mpsc::channel();

    let (outgoing_tx, outgoing_rx) = mpsc::channel();

    thread::spawn(move || {
        i2c::launch(i2c_rx, outgoing_tx);
    });

    thread::spawn(move || {
        serial::launch(i2c_tx, gui_tx, outgoing_rx);
    });

    gui::launch(gui_rx, outgoing_tx);
}
