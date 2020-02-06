#[macro_use]
extern crate lazy_static;

use std::sync::mpsc::{Receiver, Sender};

pub mod gui;
pub mod i2c;
pub mod protocol;
pub mod garfanzo;
pub mod fatkhiyev;
pub mod chungo;

pub use garfanzo::Garfanzo;
pub use fatkhiyev::Fatkhiyev;
pub use chungo::Chungo;

use protocol::{Device, IncomingMsg, Port};

pub trait AltctrlInterface {
    fn launch(&self, sender: Sender<Event>, serial_receiver: Receiver<SerialEvent>);
}

// Represents all messages sent between modules
#[derive(Clone, Debug)]
pub enum Event {
    I2C(i2c::I2CEvent),
    Serial(SerialEvent),
    Gui(gui::GuiEvent),
}

impl From<IncomingMsg> for Event {
    fn from(message: IncomingMsg) -> Self {
        match message {
            IncomingMsg::CreateWindow { window } => Event::Gui(gui::GuiEvent::CreateWindow(window)),
            IncomingMsg::DestroyWindow { id } => Event::Gui(gui::GuiEvent::DestroyWindow(id)),
            IncomingMsg::On { device, port } => Event::I2C(i2c::I2CEvent::On(device, port)),
            IncomingMsg::Off { device, port } => Event::I2C(i2c::I2CEvent::Off(device, port)),
        }
    }
}

// Represents a message sent to the interface module
// These messages are usually sent to the client
#[derive(Clone, Debug)]
pub enum SerialEvent {
    Pressed(Device, Port),
    Released(Device, Port),
}
