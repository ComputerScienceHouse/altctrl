#![allow(dead_code)]

use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::serial::types::OutgoingMsg;

pub mod types;

use types::{I2CMsg, Button};

pub fn launch(i2c_rx: Receiver<I2CMsg>, outgoing_tx: Sender<OutgoingMsg>) {
    thread::spawn(move || {
        for message in i2c_rx.iter() {
            match message {
                I2CMsg::On(button) => {
                    //turn on that button
                }

                I2CMsg::Off(button) => {
                    //turn off that button
                }
            }
        }
    });

    //handle events over i2c and send them over sender
}
