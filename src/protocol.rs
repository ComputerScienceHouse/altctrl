use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewWindow {
    pub id: String,
    pub content: String,
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: i32,
    pub height: i32,
}

// i2c stuff
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Button {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Devices {
    D1,
    D2,
}

// Serial stuff... well, it's all serial stuff.
#[derive(Serialize, Deserialize, Debug)]
pub enum IncomingMsg {
    CreateWindow(NewWindow),
    DestroyWindow(String),
    On(Button),
    Off(Button),
}

#[derive(Serialize, Debug)]
pub enum OutgoingMsg {
    Pressed(Button),
    Released(Button),
}
