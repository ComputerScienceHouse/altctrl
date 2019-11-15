// Collection of data sturtures that are used to communicate with clients

use serde::{Deserialize, Serialize};

use crate::SerialEvent;

// New window struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewWindow {
    pub id: String,
    pub content: String,
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: i32,
    pub height: i32,
}

// Represents a device in the system
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Device {
    D0,
}

// Represents a port on a device in the system
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Port {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
}

// Represents a message coming from the client
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IncomingMsg {
    CreateWindow(NewWindow),
    DestroyWindow(String),
    On(Device, Port),
    Off(Device, Port),
}

// Represents a message that will be sent to the client
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OutgoingMsg {
    Pressed(Device, Port),
    Released(Device, Port),
}

impl From<SerialEvent> for OutgoingMsg {
    fn from(event: SerialEvent) -> Self {
        match event {
            SerialEvent::Pressed(device, button) => OutgoingMsg::Pressed(device, button),
            SerialEvent::Released(device, button) => OutgoingMsg::Released(device, button),
        }
    }
}
