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
            port: Port::P0,
            device,
            state: false,
        };

        let mut device_button_array: [PortStruct; 8] = [default; 8];
        for i in 0..8 {
            let button_enum_val = match i {
                0 => Port::P0,
                1 => Port::P1,
                2 => Port::P2,
                3 => Port::P3,
                4 => Port::P4,
                5 => Port::P5,
                6 => Port::P6,
                7 => Port::P7,
                _ => panic!("Not possible I promise"),
            };

            device_button_array[i].port = button_enum_val;
        }
        device_button_array
    }

    pub fn get_port(self) -> Port {
        self.port
    }

    pub fn get_device(self) -> Device {
        self.device
    }

    pub fn get_state(self) -> bool {
        self.state
    }

    pub fn set_true(mut self) {
        self.state = true;
    }

    pub fn set_false(mut self) {
        self.state = false;
    }
}
