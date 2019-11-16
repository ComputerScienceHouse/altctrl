// use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use altctrl::{self, gui, protocol, Event, SerialEvent};

pub fn launch(tx: Sender<Event>, rx: Receiver<SerialEvent>) {
    let tx_clone = tx.clone();

    thread::spawn(move || {
        for message in rx.iter() {
            match message {
                SerialEvent::Pressed(device, button) => {
                    let string = format!("Button pressed: {:?} {:?}", device, button);
                    tx_clone
                        .send(Event::Gui(gui::GuiEvent::Log(string)))
                        .unwrap();
                }
                SerialEvent::Released(device, button) => {
                    let string = format!("Button released: {:?} {:?}", device, button);
                    tx_clone
                        .send(Event::Gui(gui::GuiEvent::Log(string)))
                        .unwrap();
                }
            }
        }
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(command) => {
                let command = command.split(',').collect::<Vec<&str>>();
                if !command.is_empty() {
                    match command[0] {
                        "log" => {
                            tx.send(Event::Gui(gui::GuiEvent::Log(command[1].to_string())))
                                .unwrap();
                        }
                        "window" => match command[1] {
                            "new" => {
                                if command.len() == 8 {
                                    tx.send(Event::Gui(gui::GuiEvent::Log(
                                        format!("Creating window, \"{}\"", command[2]).to_string(),
                                    )))
                                    .unwrap();
                                    let window = protocol::NewWindow {
                                        id: command[2].to_string(),
                                        content: command[3].to_string(),
                                        x_pos: command[4].parse::<i32>().unwrap(),
                                        y_pos: command[5].parse::<i32>().unwrap(),
                                        width: command[6].parse::<i32>().unwrap(),
                                        height: command[7].parse::<i32>().unwrap(),
                                    };
                                    tx.send(Event::Gui(gui::GuiEvent::CreateWindow(window)))
                                        .unwrap();
                                }
                            }
                            "close" => {
                                let window = command[2].to_string();
                                tx.send(Event::Gui(gui::GuiEvent::DestroyWindow(window)))
                                    .unwrap();
                            }
                            "list" => {
                                tx.send(Event::Gui(gui::GuiEvent::List())).unwrap();
                            }
                            _ => {
                                tx.send(Event::Gui(gui::GuiEvent::Log(format!("Invalid command received. ({}) Please enter a window subcommand. (new, close, list)", command[1]).to_string()))).unwrap();
                            }
                        },
                        "clear" => {
                            tx.send(Event::Gui(gui::GuiEvent::Clear())).unwrap();
                        }
                        "help" => {
                            tx.send(Event::Gui(gui::GuiEvent::Log("(log, window(id, content, x_pos, y_pos, width, height), clear, help) Separate arguments with \',\'".to_string()))).unwrap();
                        }
                        _ => {
                            tx.send(Event::Gui(gui::GuiEvent::Log(
                                "Invalid command received.".to_string(),
                            )))
                            .unwrap();
                        }
                    }
                } else {
                    tx.send(Event::Gui(gui::GuiEvent::Log(
                        "Invalid command received.".to_string(),
                    )))
                    .unwrap();
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}

fn main() {
    altctrl::start(launch);
}
