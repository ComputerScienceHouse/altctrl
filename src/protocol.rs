use serde::{Serialize, Deserialize};
// use serde::Serialize;

// use crate::i2c;
use crate::serial;
use std::sync::mpsc::Sender;


// main stuff
#[derive(Clone, Debug)]
pub enum Event {
    I2C(I2CEvent),
    Serial(serial::SerialEvent),
    Gui(GuiEvent),
}

// gui stuff
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewWindow {
    pub id: String,
    pub content: String,
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug)]
pub enum GuiEvent{
    CreateWindow(NewWindow),
    DestroyWindow(String),
    Log(String),
}


// i2c stuff
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
    pub tx: Sender<Event>
}