#![allow(dead_code)]

use serde::{Serialize, Deserialize};

use crate::i2c::types::{Button};
use crate::gui::types::{NewWindow};

#[derive(Deserialize, Debug)]
pub enum IncomingMsg {
    CreateWindow(NewWindow),
    DestroyWindow(u32),
    On(Button),
    Off(Button),
}

#[derive(Serialize, Debug)]
pub enum OutgoingMsg {
    Pressed(Button),
    Released(Button),
}