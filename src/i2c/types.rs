#![allow(dead_code)]

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Button {
    B1,
    B2,
    B3,
    B4,
}

pub enum I2CMsg {
    On(Button),
    Off(Button),
}