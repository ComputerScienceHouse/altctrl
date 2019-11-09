use std::io;
use std::io::prelude::*;

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
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("Input received: {}", line.unwrap());
    }
}