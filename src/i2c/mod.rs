#![allow(dead_code)]

use std::sync::mpsc::Sender;
use std::{thread, time};

use rppal::gpio::{Gpio, InputPin, Level, Trigger};

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

mod constants;
mod button;

use constants::*;
use button::ButtonStruct;

use crate::shared::{Event, SerialEvent};

use crate::protocol::*;

#[derive(Clone, Debug)]
pub enum I2CEvent {
    On(Device, Button),
    Off(Device, Button),
    Poll(Device),
    ResetState(Device),
}

pub struct I2CStruct {
    pub tx: Sender<Event>,
    pub input_pin_0: InputPin,
    pub i2c_device_0: LinuxI2CDevice,
    pub button_device_0: [button::ButtonStruct; 8],
}

pub fn initialize(tx: Sender<Event>) -> I2CStruct {
    let gpio = Gpio::new().expect("A new GPIO should have been created");
    let maybe_input_pin_0 = gpio
        .get(RPI_GPIO_INT_PIN)
        .expect("A new pin should have been created");
    let mut input_pin_0 = maybe_input_pin_0.into_input_pulldown();

    let mut i2c_device_0 = LinuxI2CDevice::new("/dev/i2c-1", I2C_EXPANDER_0)
        .expect("A new i2c device should have been created");

    initialize_i2c_device(&mut i2c_device_0).expect("An i2c device should have been initialied");

    let mut button_device_0 = ButtonStruct::initialize_device_buttons(Device::D0);

    let tx_clone = tx.clone();
    let tx_clone_1 = tx.clone();

    input_pin_0
        .set_async_interrupt(Trigger::RisingEdge, move |level: Level| {
            tx_clone.send(Event::I2C(I2CEvent::Poll(Device::D0)));
        })
        .expect("The rising edge interrupt function should be configured");

    input_pin_0
        .set_async_interrupt(Trigger::FallingEdge, move |level: Level| {
            tx_clone_1.send(Event::I2C(I2CEvent::ResetState(Device::D0)));
        }).expect("The falling edge interrupt function should be configured");

    I2CStruct {
        tx,
        input_pin_0,
        i2c_device_0,
        button_device_0,
    }
}

pub fn handle(message: I2CEvent, i2c_struct: &mut I2CStruct) {
    match message {
        I2CEvent::Poll(device) => {
            let mut dev = {
                match device {
                    Device::D0 => &mut i2c_struct.i2c_device_0,
                }
            };
            let buttons = read_i2c(&mut dev, INTFA).expect("the buttons should have been read");
            for x in 0..8 {
                let test = 1 & (buttons >> x) == 1;
                if test != i2c_struct.button_device_0[x].state && i2c_struct.button_device_0[x].state == false {
                    i2c_struct.button_device_0[x].state = true;
                    send_serial(i2c_struct.button_device_0[x], i2c_struct)
                }
            }
        }

        I2CEvent::ResetState(device) => {
            let mut dev = {
                match device {
                    Device::D0 => &mut i2c_struct.i2c_device_0,
                }
            };
            let buttons = read_i2c(&mut dev, INTFA).expect("the buttons should have been read");
            for x in 0..8 {
                let test = 1 & (buttons >> x) == 1;
                if test != i2c_struct.button_device_0[x].state && i2c_struct.button_device_0[x].state == false{
                    i2c_struct.button_device_0[x].state = false;
                }
            }
        }

        I2CEvent::On(device, button) => {
            let mut i2c = {
                match device {
                    Device::D0 => &mut i2c_struct.i2c_device_0,
                }
            };
            let pin = button_to_address(button);
            i2c.smbus_write_byte_data(constants::OLATB, pin)
                .expect("LEDs should have turned on");
        }

        I2CEvent::Off(device, button) => {
            thread::sleep(time::Duration::from_secs(2));
            let mut i2c = {
                match device {
                    Device::D0 => &mut i2c_struct.i2c_device_0,
                }
            };
            let pin = button_to_address(button);
            i2c.smbus_write_byte_data(OLATB, !pin)
                .expect("LEDs should have turned on");
        }
    }
}

fn initialize_i2c_device(dev: &mut LinuxI2CDevice) -> Result<(), LinuxI2CError> {
    dev.smbus_write_byte_data(IODIRB, 0x00)?;
    dev.smbus_write_byte_data(DEFVALA, 0x00)?;
    dev.smbus_write_byte_data(DEFVALB, 0xff)?;
    dev.smbus_write_byte_data(INTCONA, 0xff)?;
    dev.smbus_write_byte_data(IOCON, 0x02)?;
    dev.smbus_write_byte_data(GPINTAEN, 0xff)?;
    Ok(())
}

fn read_i2c(dev: &mut LinuxI2CDevice, register: u8) -> Result<u8, LinuxI2CError> {
    let pin = dev.smbus_read_byte_data(register)?;
    println!("{}", pin);
    println!("a button has been pressed");

    Ok(pin)
}

fn button_to_address(button: Button) -> u8 {
    match button {
        Button::B0 => !0x00,
        Button::B1 => !0x01,
        Button::B2 => !0x02,
        Button::B3 => !0x03,
        Button::B4 => !0x04,
        Button::B5 => !0x05,
        Button::B6 => !0x06,
        Button::B7 => !0x07,
    }
}

fn send_serial(button: button::ButtonStruct, i2c_struct: &mut I2CStruct){
    i2c_struct.tx.send(Event::Serial(SerialEvent::Pressed(button.device, button.button)))
        .expect("A pressed serial message shoul have been sent");
}
