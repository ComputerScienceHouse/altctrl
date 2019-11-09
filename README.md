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

`cargo run --bin garfanzo > /dev/pts/0`

you'll have to look in /dev/pts with multiple terminals open to see what is available.
