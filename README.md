# The Death Panel
The Panel of (the) Deaths! **THE NEW VERSION!** This control panel is designed to allow anyone to interfere with CSH's GDD Imagine 2020 project! It utilizes a Rasberry Pi to control a lot of neato buttons and dials and switches and knobs and things. The software is written in Rust, and runs on top of a ~tiny~ Linux kernel.

This project is the successor to the prototype built by Will Nilges. (git.nilges.me/deathpanel)

The general idea is that we're going to have several co-op™ minigames for people to walk up to the booth and play. Meanwhile, this control panel will be sitting around and allow other passerbys to view game stats, press buttons, and change something in the game! Nobody really knows what exactly this whole thing will turn into, but I thought it was a neat project that I wanted to do well, and so I made this repo. 


### Dependencies:

 - `ncurses`
 - `ncurses-dev`
 - `libudev` (might be included in `systemd-devel`)

### Garfanzo:
Garfanzo™ is our debug program.

To properly run Garfanzo, you need to pipe the output to another tty. You can do this from a desktop session like so:

Open two terminal emulators.
In the first terminal run the tty command. This command will return the tty device of the terminal emulator.
In the second terminal navigate to this repo and run,
`cargo run --bin garfanzo > {result of tty}`

### Setting up usb gadget
Altctrl communicates with the client over the rpi zero's data usb port.
The rpi must be configured to allow serial over usb.

1. Establish a form of communication unrelated to the usb port.
    Enabling the serial gadget module disables the usb port for use with other devices such as keyboards. 
    There are many ways to communicate with a rpi without a usb port. For this project we had the rpi connected to wifi and communicated with it via ssh.

2. Enable the dwc2 module in config.txt.
    This step is not performed on the rpi. Take the sd card out of the rpi while it is off and insert it into a computer so you can edit the files.
    Open the boot partition and find the file named config.txt.
    Open config.txt with a text editor of your choice and append a line containing 'dtoverlay=dwc2'. Save this file.


3. Enable dwc2 and g_serial in cmdline.txt.
    This step is also performed on the computer. Open the file named cmdline.txt.
    Be careful with this file, it has strict formatting! Each parameter is seperated by a single space.
    Insert 'modules-load=dwc2,g_serial' after rootwait and before any other parameters.
    Make sure there are no newline characters in the file before saving.

### I2C
All the user inputs, and button LEDs, of the Death Panel are connected to MCP23017 DIO expanders. These communicate with the Raspberry PI via I2C. Each device can have 1 of 8 different addresses. Buttons are connected to bank A and LEDs are connected to bank B. To add a device to the Death Panel:

1. Make sure that there are no more than 7 devices connected to the Raspberry PI

2. Wire the address pins to ground in an order that creates an address **UNIQUE** from those of the other devices. 

3. Add a constant with the address to the new device. 

4. Initialize the device in the I2C module (in the initialize fn):
`LinuxI2CDevice::new("/dev/i2c-1", I2C_EXPANDER_0).expect("A new i2c device should have been created")`

Each MCP23017 is set up with interrpts so that when a button is pressed a GPIO pin on the Rasperry PI is raised. This triggers an Event that will poll the device that caused the interrupt for the following information:

- Which button was pressed
- What is the actual state of that button

In order to setup an interrupt the following must be done:

1. Wire the INTA pin on the MCP23017 to a GPIO pin on the Rasperry PI. Use a 4.7kOhm **pull-down resistor**.

2. Set-up the device configurations by calling the following function:
`initialize_i2c_device(INSERT_DEVICE_HERE).expect("An i2c device should have been initialied")`
The `initialize_i2c_device(device)` function configures the device for use with the Death Panel. Please refer to in-code comments for specifics.

3. Get a Pin from the GPIO and make it an InputPin. 

4. Set the interrupt function for the InputPin:
``` Rust
INPUT_PIN
        .set_async_interrupt(Trigger::RisingEdge, move |level: Level| match level {
            Level::High => INSERT_CODE_RUN_ON_INTERRUPT_WHEN_PIN_STATE_IS_HIGH

            Level::Low => INSERT_CODE_RUN_ON_INTERRUPT_WHEN_PIN_STATE_IS_LOW 
        })
        .expect("The rising edge interrupt function should be configured")
```

### I2C Dependencies
- `rppal`
- `i2cdev`
