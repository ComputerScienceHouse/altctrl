#![allow(dead_code)]

pub enum Button {
    B1,
    B2,
    B3,
    B4,
}

impl Button {
    pub fn serialize(&self) -> String {
        match self {
            Button::B1 => String::from("B1"),
            Button::B2 => String::from("B2"),
            Button::B3 => String::from("B3"),
            Button::B4 => String::from("B4"),
        }
    }

    pub fn deserialize(button: &str) -> Result<Button, ()> {
        match button {
            "B1" => Ok(Button::B1),
            "B2" => Ok(Button::B2),
            "B3" => Ok(Button::B3),
            "B4" => Ok(Button::B4),
            _ => Err(()),
        }
    }
}

pub enum I2CMsg {
    On(Button),
    Off(Button),
}

impl I2CMsg {
    pub fn deserialize(token: &str, param: &str) -> Result<I2CMsg, ()> {
        match token {
            "ON" => match Button::deserialize(param) {
                Ok(button) => Ok(I2CMsg::On(button)),
                Err(_) => Err(()),
            },

            "OF" => match Button::deserialize(param) {
                Ok(button) => Ok(I2CMsg::Off(button)),
                Err(_) => Err(()),
            },

            _ => Err(()),
        }
    }
}

pub enum GuiMsg {}

pub enum OutgoingMsg {
    Pressed(Button),
    Released(Button),
}

impl OutgoingMsg {
    pub fn serialize(&self) -> String {
        match self {
            OutgoingMsg::Pressed(button) => format!("DW{}", button.serialize()),
            OutgoingMsg::Released(button) => format!("UP{}", button.serialize()),
        }
    }
}
