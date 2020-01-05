// Collection of data sturtures that are used to communicate with clients

use serde::{Deserialize, Serialize};

use crate::SerialEvent;

// New window struct
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewWindow {
    pub id: String,
<<<<<<< HEAD
    pub content: String,
    pub message: String,
=======
    pub content: WindowContent,
    pub style: WindowStyle,
>>>>>>> ef72d5766baf7cd45b97e41942256d1a091a2bda
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: i32,
    pub height: i32,
}

//Contents of window.
#[derive(Clone, Debug, Serialize, Deserialize)]
<<<<<<< HEAD
pub enum WindowContent {
    Text,
    List,
    Scoreboard,
    ProgressBar,
=======
pub struct WindowContent {
    pub text: Option<String>,
    pub list: Option<Vec<String>>,
    pub progressBar: Option<(i32, i32)>,
    /*pub Table: Option<Vec>,*/
}

// Decorate contents of window with different ncurses characteristics
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WindowStyle {
    Bold,
    Invert,
    Flashing,
>>>>>>> ef72d5766baf7cd45b97e41942256d1a091a2bda
}

// Represents a device in the system
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Device {
    D0,
}

// Represents a port on a device in the system
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Port {
    P0 = 0,
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
}

// Represents a message coming from the client
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IncomingMsg {
    CreateWindow { window: NewWindow },
    DestroyWindow { id: String },
    On { device: Device, port: Port },
    Off { device: Device, port: Port },
}

// Represents a message that will be sent to the client
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OutgoingMsg {
    Pressed { device: Device, port: Port },
    Released { device: Device, port: Port },
}

impl From<SerialEvent> for OutgoingMsg {
    fn from(event: SerialEvent) -> Self {
        match event {
            SerialEvent::Pressed(device, port) => OutgoingMsg::Pressed { device, port },
            SerialEvent::Released(device, port) => OutgoingMsg::Released { device, port },
        }
    }
}
