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
    // let mut file = File::create("/tmp/altctrl.serial").unwrap();
    // file.write_all(b"Hello, world!").unwrap();
    tx.send(Event::Gui(gui::GuiEvent::Log("Hello there!".to_string()))).unwrap();
    tx.send(Event::Gui(gui::GuiEvent::Log("there!".to_string()))).unwrap();
    tx.send(Event::Gui(gui::GuiEvent::Log("Here!".to_string()))).unwrap();
    tx.send(Event::Gui(gui::GuiEvent::Log("Hi!".to_string()))).unwrap();
    tx.send(Event::Gui(gui::GuiEvent::Log("Hello!".to_string()))).unwrap();
    tx.send(Event::Gui(gui::GuiEvent::Log("Hello there!".to_string()))).unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        // println!("Input received: {}", line.unwrap());
        match line {
            Ok(command) => match command.as_ref() {
                "print" => {
                    // println!("Hullo!!!! :)");
                    tx.send(Event::Gui(gui::GuiEvent::Log("Hello there!".to_string()))).unwrap();
                }
                "yeet" => {
                    // println!("who is ligma?");
                    tx.send(Event::Gui(gui::GuiEvent::Log("Who is Ligma?".to_string()))).unwrap();

                },
                "leave" => {
                    tx.send(Event::Gui(gui::GuiEvent::Log("Later brother!".to_string()))).unwrap();
                    std::process::exit(0);
                },
                "window" => {
                    // let window = protocol::NewWindow {"Win01", "Hello there my funky groovy dude", 20,20,20,20};
                    tx.send(Event::Gui(gui::GuiEvent::CreateWindow(window))).unwrap();
                },
                _ => {
                    println!("WHAT??? {}", command);
                    tx.send(Event::Gui(gui::GuiEvent::Log("command".to_string()))).unwrap();
                }
            },
            _ => {
                println!("*** OH F*CK!!! ***");
            }
        }
    }
}

fn main() {
    shared::start(launch);
}
