#![allow(dead_code)]

use std::sync::mpsc::Sender;

use rppal::gpio::{Gpio, InputPin, Trigger, Level};

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

//use serde::{Serialize, Deserialize};

use crate::shared::Event;

use crate::protocol::*;

#[derive(Clone, Debug)]

pub enum I2CEvent {
    On(Button, Devices),
    Off(Button, Devices),
}

pub struct I2CStruct {
    pub tx: Sender<Event>,
    pub input_pin_1: InputPin,
    pub i2c_device_1: LinuxI2CDevice,
}

pub fn initialize(tx: Sender<Event>) -> I2CStruct {
    let gpio = Gpio::new().expect("A new GPIO should have been created");
    let maybe_input_pin_1 = gpio.get(constants::RPI_GPIO_INT_PIN).expect("A new pin should have been created");
    let input_pin_1 = maybe_input_pin_1.into_input_pulldown();

    let mut i2c_device_1 = LinuxI2CDevice::new("/dev/i2c-1", constants::I2C_EXPANDER_1).expect("A new i2c device should have been created");
    initialize_i2c_device(&mut i2c_device_1).expect("An i2c device should have been initialied");

    I2CStruct{
        tx,
        input_pin_1,
        i2c_device_1,

    }
}

pub fn handle(message: I2CEvent, i2c_struct: &mut I2CStruct) {
    i2c_struct.input_pin_1.set_async_interrupt(Trigger::RisingEdge, move |level: Level|{
        let mut state: [bool; 8] = [false;8];
        let buttons = read_i2c(&mut i2c_struct.i2c_device_1, constants::INTFA).expect("the buttons should have been read");
        println!("this is fucking working");
        for x in 0..8{
            let bool_test = 1 & (buttons >> x) == 1;
            if bool_test != state[x]{
                state[x] = bool_test;
            }
        }
    }).unwrap();
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
