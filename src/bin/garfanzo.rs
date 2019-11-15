use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::sync::mpsc::{Receiver, Sender};

use altctrl::{
    self,
    Event,
    SerialEvent,
};

pub fn launch(tx: Sender<Event>, rx: Receiver<SerialEvent>) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(command) => {
                let command = command.split(",").collect::<Vec<&str>>();
                if command.len() >= 1 {
                    match command[0].as_ref() {
                        "log" => {
                            tx.send(Event::Gui(gui::GuiEvent::Log(command[1].to_string()))).unwrap();
                        },
                        "window" => {
                            match command[1].as_ref() {
                                "new" => {
                                    if command.len() == 8 {
                                        tx.send(
                                            Event::Gui(
                                            gui::GuiEvent::Log(
                                                format!("Creating window, \"{}\"", command[2]).to_string()))).unwrap();
                                        let window = protocol::NewWindow {
                                            id: command[2].to_string(),
                                            content: command[3].to_string(),
                                            x_pos: command[4].parse::<i32>().unwrap(),
                                            y_pos: command[5].parse::<i32>().unwrap(), 
                                            width: command[6].parse::<i32>().unwrap(), 
                                            height: command[7].parse::<i32>().unwrap()
                                        };
                                        tx.send(Event::Gui(gui::GuiEvent::CreateWindow(window))).unwrap();    
                                    }
                                },
                                "close" => {
                                    let window = command[2].to_string();
                                    tx.send(Event::Gui(gui::GuiEvent::DestroyWindow(window))).unwrap();    
                                },
                                "list" => {
                                    tx.send(Event::Gui(gui::GuiEvent::List())).unwrap();
                                },
                                _ => {
                                tx.send(Event::Gui(gui::GuiEvent::Log(format!("Invalid command received. ({}) Please enter a window subcommand. (new, close, list)", command[1]).to_string()))).unwrap();
                                },
                            }                            
                        },
                        "clear" => {
                            tx.send(Event::Gui(gui::GuiEvent::Clear())).unwrap();
                        },
                        "help" => {
                            tx.send(Event::Gui(gui::GuiEvent::Log("(log, window(id, content, x_pos, y_pos, width, height), clear, help) Separate arguments with \',\'".to_string()))).unwrap();
                        },
                        _ => {
                            tx.send(Event::Gui(gui::GuiEvent::Log("Invalid command received.".to_string()))).unwrap();
                        },

                    }
                } else {
                    tx.send(Event::Gui(gui::GuiEvent::Log("Invalid command received.".to_string()))).unwrap();
                }
            },
            _ => {

            },
            // match command.as_ref() {
            //     "log" => {
            //         let message = stdin.lock().lines().next();
            //         match message.unwrap() {
            //             Ok(m) => {
            //                 tx.send(Event::Gui(gui::GuiEvent::Log(m))).unwrap();
            //             },
            //             _ => {
            //                 tx.send(Event::Gui(gui::GuiEvent::Log("Error parsing log message.".to_string()))).unwrap();
            //             },
            //         }
            //     }
            //     "yeet" => {
            //         tx.send(Event::Gui(gui::GuiEvent::Log("Who is Ligma?".to_string()))).unwrap();
            //     },
            //     "clear" => {
            //         tx.send(Event::Gui(gui::GuiEvent::Clear())).unwrap();
            //     },
            //     "leave" => {
            //         tx.send(Event::Gui(gui::GuiEvent::Log("Later brother!".to_string()))).unwrap();
            //         std::process::exit(0);
            //     },
            //     "window" => {
            //         tx.send(Event::Gui(gui::GuiEvent::Log("Creating window...".to_string()))).unwrap();
            //         let window = protocol::NewWindow {id: "Win01".to_string(), content: "Hello there my funky groovy dude".to_string(), x_pos: 20, y_pos: 20, width: 20, height: 20};
            //         tx.send(Event::Gui(gui::GuiEvent::CreateWindow(window))).unwrap();
            //     },
            //     _ => {
            //         tx.send(Event::Gui(gui::GuiEvent::Log(command.to_string()))).unwrap();
                // }
            _ => {
                println!("*** OH F*CK!!! ***");
            }
        }
    }
}

fn main() {
    altctrl::start(launch);
}
