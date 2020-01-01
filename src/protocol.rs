use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewWindow {
    pub id: String,
    pub content: String, //Contents?
    pub x_pos: i32,
    pub y_pos: i32,
    pub width: i32,
    pub height: i32,
}

//Contents of window. Text? Progress bar? List? (Needs titles for stuff as well as text decoration such as bold, invert, and flashing)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WindowContent {
    Text,
    List,
    Chart,
    ProgressBar,
}


// Represents a port on a device in the system
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Button {
    B1,
    B2,
    B3,
    B4,
}

// Serial stuff... well, it's all serial stuff.
#[derive(Serialize, Deserialize, Debug)]
enum IncomingMsg {
    CreateWindow(NewWindow),
    DestroyWindow(u32),
    On(Button),
    Off(Button),
}

#[derive(Serialize, Debug)]
enum OutgoingMsg {
    Pressed(Button),
    Released(Button),
}
