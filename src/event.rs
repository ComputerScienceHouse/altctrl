use crate::i2c;
use crate::gui;
use crate::serial;

#[derive(Clone, Debug)]
pub enum Event {
    I2C(i2c::I2CEvent),
    Serial(serial::SerialEvent),
    Gui(gui::GuiEvent),
}