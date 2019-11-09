use std::io;
use std::fs::File;
use std::io::prelude::*;

use std::sync::mpsc;
use std::thread;

pub mod protocol;
pub mod gui;
mod i2c;
mod serial;

use protocol::*;

fn main() -> std::io::Result<()> {
    let mut file = File::create("/tmp/altctrl.serial")?;
    file.write_all(b"Hello, world!")?;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("Input received: {}", line.unwrap());
    }
    Ok(())
}