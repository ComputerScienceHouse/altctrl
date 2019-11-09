#![allow(dead_code)]

use std::sync::mpsc::Sender;

use serde::{Serialize, Deserialize};

use crate::Event;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Button {
    B1,
    B2,
    B3,
    B4,
}

#[derive(Clone, Debug)]
pub enum I2CEvent {
    On(Button),
    Off(Button),
}


pub struct I2CStruct {
    tx: Sender<Event>
}

pub fn initialize(tx: Sender<Event>) -> I2CStruct {
    I2CStruct {
        tx
    }
}

pub fn handle(message: I2CEvent, i2c_struct: &mut I2CStruct) {

}