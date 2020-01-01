use crate::protocol::*;

#[derive(Copy, Clone, Debug)]
pub struct PortStruct {
    pub port: Port,
    pub device: Device,
    pub state: bool,
}

impl PortStruct {
    ///The initialize device function creates an array of PortStructs that holds the Buton,
    /// Device and state of the the button. It defaults the state to zero. This function
    /// returns the array.
    pub fn initialize_device_buttons(device: Device) -> [PortStruct; 8] {
        let default = PortStruct {
            port: Port::B0,
            device: device,
            state: false,
        };

        let mut device_button_array: [PortStruct; 8] = [default; 8];
        for i in 0..8 {
            let button_enum_val = match i {
                0 => Port::B0,
                1 => Port::B1,
                2 => Port::B2,
                3 => Port::B3,
                4 => Port::B4,
                5 => Port::B5,
                6 => Port::B6,
                7 => Port::B7,
                _ => panic!("Not possible I promise"),
            };

            device_button_array[i].port = button_enum_val;
        }
        device_button_array
    }

    pub fn get_port(&self) -> Port {
        self.port
    }

    pub fn get_device(&self) -> Device {
        self.device
    }

    pub fn get_state(&self) -> bool {
        self.state
    }

    pub fn set_true(mut self) {
        self.state = true;
    }

    pub fn set_false(mut self) {
        self.state = false;
    }
}
