use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::types::*;

pub fn launch(sender: Sender<OutgoingMsg>, receiver: Receiver<I2CMsg>) {
    thread::spawn(move || {
        for message in receiver.iter() {
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
