#![allow(dead_code)]

use std::sync::mpsc::Sender;

//use serde::{Serialize, Deserialize};

use crate::Event;

use crate::protocol::*;

pub fn initialize(tx: Sender<Event>) -> I2CStruct {
    I2CStruct {
        tx
    }
}

pub fn handle(message: I2CEvent, i2c_struct: &mut I2CStruct) {

}