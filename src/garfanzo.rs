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
    /*tx.send(Event::Gui(gui::GuiEvent::Log("meme!".to_string()))).unwrap();
    tx.send(Event::Gui(gui::GuiEvent::Log("yaye!".to_string()))).unwrap();
    tx.send(Event::Gui(gui::GuiEvent::Log("goooo!".to_string()))).unwrap();
    tx.send(Event::Gui(gui::GuiEvent::Log("water".to_string()))).unwrap();
    tx.send(Event::Gui(gui::GuiEvent::Log("slaghajaja".to_string()))).unwrap();
    tx.send(Event::Gui(gui::GuiEvent::Log("eeeee??????".to_string()))).unwrap();*/

    //tx.send(Event::Gui(gui::GuiEvent::Log("aAaA".to_string()))).unwrap();
    //let window = protocol::NewWindow {id: "Win00".to_string(), content: "I'm gonna /yeet".to_string(), x_pos: 40, y_pos: 30, width: 10, height: 5};
    //tx.send(Event::Gui(gui::GuiEvent::CreateWindow(window))).unwrap();
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
                "clear" => {
                    tx.send(Event::Gui(gui::GuiEvent::Clear())).unwrap();
                },
                "leave" => {
                    tx.send(Event::Gui(gui::GuiEvent::Log("Later brother!".to_string()))).unwrap();
                    std::process::exit(0);
                },
                "window" => {
                    tx.send(Event::Gui(gui::GuiEvent::Log("Creating window...".to_string()))).unwrap();
                    let window = protocol::NewWindow {id: "Win01".to_string(), content: "Hello there my funky groovy dude".to_string(), x_pos: 20, y_pos: 20, width: 20, height: 20};
                    tx.send(Event::Gui(gui::GuiEvent::CreateWindow(window))).unwrap();
                },
                _ => {
                    // println!("WHAT??? {}", command);
                    tx.send(Event::Gui(gui::GuiEvent::Log(command.to_string()))).unwrap();
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
