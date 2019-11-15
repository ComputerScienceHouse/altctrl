# The Death Panel (gddimagine2020-altctrl-deathpanel)
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