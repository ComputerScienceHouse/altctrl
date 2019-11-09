use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::sync::mpsc::{Receiver, Sender};

mod gui;
mod i2c;
mod protocol;
mod shared;

use shared::{Event, SerialEvent};

pub fn launch(tx: Sender<Event>, rx: Receiver<SerialEvent>) {
    let mut file = File::create("/tmp/altctrl.serial").unwrap();
    file.write_all(b"Hello, world!").unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // println!("Input received: {}", line.unwrap());
        match line {
            Ok(command) => match command.as_ref() {
                "print" => {
                    println!("Hullo!!!! :)");
                }
                "yeet" => {
                    println!("who is ligma?");
                }
                _ => {
                    println!("NANI THE FUCK???");
                }
            },
            _ => {
                println!("*** OH FUCK!!! ***");
            }
        }
    }
}

fn main() {
    shared::start(launch);
}
