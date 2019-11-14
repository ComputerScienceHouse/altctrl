use crate::protocol::*;

#[derive(Copy, Clone, Debug)]
pub struct ButtonStruct{
    pub button: Button,
    pub device: Device,
    pub state: bool,
}

impl ButtonStruct{
    pub fn initialize_device_buttons(device: Device) -> [ButtonStruct; 8]{
        let default = ButtonStruct {
            button: Button::B0,
            device: device,
            state: false,
        };

        let mut device_button_array: [ButtonStruct; 8] = [default; 8];
        for i in 0..8{
            let button_enum_val = match i {
                0 => Button::B0,
                1 => Button::B1,
                2 => Button::B2,
                3 => Button::B3,
                4 => Button::B4,
                5 => Button::B5,
                6 => Button::B6,
                7 => Button::B7,
                _ => panic!("Not possible I promise")
            };

            device_button_array[i].button = button_enum_val;
        }
        device_button_array 
    }
}