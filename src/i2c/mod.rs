#![allow(dead_code)]

use std::sync::mpsc::Sender;
use std::{thread, time};

use rppal::gpio::{Gpio, InputPin, Trigger, Level};

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

mod constants;
use constants::*;
//use serde::{Serialize, Deserialize};

use crate::shared::Event;
use crate::shared::SerialEvent;

use crate::protocol::*;

#[derive(Clone, Debug)]

pub enum I2CEvent {
    On(Button, Devices),
    Off(Button, Devices),
    Poll(Devices),
}

pub struct I2CStruct {
    pub tx: Sender<Event>,
    pub input_pin_1: InputPin,
    pub i2c_device_1: LinuxI2CDevice,
    pub button_array: [Button; 8],
}

pub fn initialize(tx: Sender<Event>) -> I2CStruct {
    let gpio = Gpio::new().expect("A new GPIO should have been created");
    let maybe_input_pin_1 = gpio.get(constants::RPI_GPIO_INT_PIN).expect("A new pin should have been created");
    let mut input_pin_1 = maybe_input_pin_1.into_input_pulldown();

    let mut i2c_device_1 = LinuxI2CDevice::new("/dev/i2c-1", constants::I2C_EXPANDER_1).expect("A new i2c device should have been created");
    initialize_i2c_device(&mut i2c_device_1).expect("An i2c device should have been initialied");

    let mut button_array: [Button; 8] = [Button::B0, Button::B1, Button::B2, Button::B3, Button::B4, Button::B5, Button::B6, Button::B7];    

    let tx_clone = tx.clone();

    input_pin_1.set_async_interrupt(Trigger::RisingEdge, move |level: Level|{
        tx_clone.send(Event::I2C(I2CEvent::Poll(Devices::D1)));
    }).unwrap();

    I2CStruct{
        tx,
        input_pin_1,
        i2c_device_1,
        button_array,

    }
}

pub fn handle(message: I2CEvent, i2c_struct: &mut I2CStruct) {
    match message{
        I2CEvent::Poll(dev) => {
            let mut device = {
                match dev{
                    Devices::D1 => {
                        &mut i2c_struct.i2c_device_1
                    }
                    Devices::D2 => {
                        panic!("this does not exist yet");
                    }
                }
            };
            let mut state: [bool; 8] = [false;8];
            let buttons = read_i2c(&mut device, constants::INTFA).expect("the buttons should have been read");
            println!("this is fucking working");
            for x in 0..8{
                let bool_test = 1 & (buttons >> x) == 1;
                if bool_test != state[x]{
                    state[x] = bool_test;
                    i2c_struct.tx.send(Event::I2C(I2CEvent::On(i2c_struct.button_array[x], dev))).unwrap();
                    i2c_struct.tx.send(Event::Serial(SerialEvent::Pressed(i2c_struct.button_array[x], dev))).unwrap();
                    i2c_struct.tx.send(Event::I2C(I2CEvent::Off(i2c_struct.button_array[x], dev))).unwrap();
                }
            }
            device.smbus_write_byte_data(constants::OLATB, 0xff).expect("The LED should be reset");
        }
        I2CEvent::On(button, dev) => {
            let mut device = {
                match dev{
                    Devices::D1 => {
                        &mut i2c_struct.i2c_device_1
                    }

                    Devices::D2 => {
                        panic!("this does not exist yet");
                    }
                }
            };
            let pin_to_read = Button_to_address(button);
            device.smbus_write_byte_data(constants::OLATB, pin_to_read).expect("LEDs should have turned on");
        }
        I2CEvent::Off(button, dev) => {
            thread::sleep(time::Duration::from_secs(2));
            let mut device = {
                match dev{
                    Devices::D1 => {
                        &mut i2c_struct.i2c_device_1
                    }

                    Devices::D2 => {
                        panic!("this does not exist yet");
                    }
                }
            };
            let pin_to_read = Button_to_address(button);
            device.smbus_write_byte_data(constants::OLATB, !pin_to_read).expect("LEDs should have turned on");
        }
        
    }
    
}

fn initialize_i2c_device(dev: &mut LinuxI2CDevice) -> Result<(), LinuxI2CError>{
    dev.smbus_write_byte_data(constants::IODIRB, 0x00)?;
    dev.smbus_write_byte_data(constants::DEFVALA, 0x00)?;
    dev.smbus_write_byte_data(constants::DEFVALB, 0xff)?;
    dev.smbus_write_byte_data(constants::INTCONA, 0xff)?;
    dev.smbus_write_byte_data(constants::IOCON, 0x02)?;
    dev.smbus_write_byte_data(constants::GPINTAEN, 0xff)?;
    Ok(())
}

fn read_i2c(dev: &mut LinuxI2CDevice, register: u8) -> Result<u8, LinuxI2CError>{
    let pin_to_read = dev.smbus_read_byte_data(register)?;
    println!("{}", pin_to_read);
    println!("a button has been pressed");

    Ok(pin_to_read)
}

fn Button_to_address(button: Button) -> u8{
    let mut address = {
        match button{
            Button::B0 => !0x00,
            Button::B1 => !0x01,
            Button::B2 => !0x02,
            Button::B3 => !0x03,
            Button::B4 => !0x04,
            Button::B5 => !0x05,
            Button::B6 => !0x06,
            Button::B7 => !0x07
        }
    };
    return address;
}
