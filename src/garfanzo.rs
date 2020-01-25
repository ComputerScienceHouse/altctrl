use crate::{AltctrlInterface, Event, SerialEvent, gui, protocol};
use std::sync::mpsc::{Sender, Receiver};
use std::{thread, io};
use std::io::BufRead;

pub struct Garfanzo;

impl AltctrlInterface for Garfanzo {
    fn launch(&self, sender: Sender<Event>, serial_receiver: Receiver<SerialEvent>) {
        let sender_clone = sender.clone();

        thread::spawn(move || {
            for message in serial_receiver.iter() {
                match message {
                    SerialEvent::Pressed(device, button) => {
                        let string = format!("Button pressed: {:?} {:?}", device, button);
                        sender_clone
                            .send(Event::Gui(gui::GuiEvent::Log(string)))
                            .unwrap();
                    }
                    SerialEvent::Released(device, button) => {
                        let string = format!("Button released: {:?} {:?}", device, button);
                        sender_clone
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
                    let command = command.split('`').collect::<Vec<&str>>();
                    if !command.is_empty() {
                        match command[0] {
                            "log" => {
                                if command.len() > 1 {
                                    match command[1] {
                                        "put" => {
                                            if command.len() == 3 {
                                                notify(command[2], &sender);
                                            }
                                        },
                                        "toggle" => {
                                            sender.send(Event::Gui(gui::GuiEvent::ToggleConsole())).unwrap();
                                        },
                                        _ => {
                                            invalid_command(&sender);
                                        },
                                    }
                                }
                            }
                            "window" => {
                                if command.len() > 1 {
                                    match command[1] {
                                        "new" => {
                                            if command.len() >= 10 && command.len() <= 11 { // Remember to update this number when you add new shit to the window protocol
                                                sender.send(Event::Gui(gui::GuiEvent::Log(format!("Creating window, \"{}\"", command[2]).to_string(),))).unwrap();
                                                //TODO: Why can't I figure out this unwrap?
                                                let mut priority = false;
                                                if command.len() == 11 && command[10] == "!" {
                                                    priority = true;
                                                }
                                                let window = protocol::WindowData {
                                                    id:      command[2].to_string(),
                                                    content: command[3].to_string(),
                                                    message: command[4].to_string(),
                                                    style:   command[5].to_string(),
                                                    x_pos:   command[6].parse::<i32>().unwrap(),
                                                    y_pos:   command[7].parse::<i32>().unwrap(),
                                                    width:   command[8].parse::<i32>().unwrap(),
                                                    height:  command[9].parse::<i32>().unwrap(),
                                                    priority: priority,
                                                };
                                                sender.send(Event::Gui(gui::GuiEvent::CreateWindow(window))).unwrap();
                                            } else {
                                                invalid_command(&sender);
                                            }
                                        }
                                        "close" => {
                                            let window = command[2].to_string();
                                            notify(format!("Closing window \"{}\"", &window).as_str(), &sender);
                                            sender.send(Event::Gui(gui::GuiEvent::DestroyWindow(window))).unwrap();
                                        }
                                        "list" => {
                                            sender.send(Event::Gui(gui::GuiEvent::List())).unwrap();
                                        }
                                        "move" => {
                                            if command.len() == 5 {
                                                notify(format!("Moving window \"{}\"", command[2]).as_str(), &sender);
                                                sender.send(Event::Gui(gui::GuiEvent::MoveWindow(
                                                    command[2].to_string(),
                                                    command[3].parse::<i32>().unwrap(),
                                                    command[4].parse::<i32>().unwrap(),
                                                ))).unwrap();
                                            }
                                        }
                                        "resize" => {
                                            notify(format!("Resizing window \"{}\"", command[2]).as_str(), &sender);
                                            sender.send(Event::Gui(gui::GuiEvent::ResizeWindow(
                                                command[2].to_string(),
                                                command[3].parse::<i32>().unwrap(),
                                                command[4].parse::<i32>().unwrap(),
                                            ))).unwrap();
                                        }
                                        _ => {
                                            invalid_command(&sender);
                                        }
                                    }
                                } else {
                                    invalid_command(&sender);
                                }
                            },
                            "redraw" => {
                                sender.send(Event::Gui(gui::GuiEvent::Redraw())).unwrap();
                            }
                            "clear" => {
                                sender.send(Event::Gui(gui::GuiEvent::Clear())).unwrap();
                                notify("Screen cleared.", &sender);
                            }
                            "help" => {
                                //(log, window(command (new, (id, content (Text, List, Scoreboard, ProgressBar), message (separate with | and then with +), x_pos, y_pos, width, height), move (id, x, y), resize (id, x, y), close)), clear, help) Separate arguments with \',\'";
                                let help_message =
                                    "===HOW TO USE GARFANZO===
—————————————————————————
log [put | toggle],
window [
    new[
        id,
        content[ T(ext) | L(ist) | SB (ScoreBoard) | PB (ProgressBar) ],
        message (Separate elements with '|' and sub elements with ':'),
        style[ (If styling Text, List, or ScoreBoard [bold | highlight | blink | underline]) | (If styling a ProgressBar [[low | blink], <character>]) ],
        x_pos, y_pos,
        width, height,
        (Optionally, add '!' at the end of your command to allow overriding.
        Separate arguments with '`')
    ],
    close[id],
    list,
    move[id, x, y],
    resize[id, x, y]
]
redraw,
clear,
help
"

                                        // "[ log (put | toggle) ],
                                        // [ window(
                                        //     new(
                                        //         id,
                                        //         content(
                                        //             T <text> | L <list> | SB <scoreboard> | PB <progressbar> )
                                        //         message <separate with | and then with +>
                                        //         style <For text, lists, and scoreboards (bold | highlight | blink | underline)> <For progress bars ()  | x_pos | y_pos | width | height) | close (id) | list | move (x | y) | resize (x | y )) ], [ redraw ], [ clear ], [ help ], Optionally, add '!' at the end of your command to allow overriding. Separate arguments with '`'."
                                        .to_string();
                                eprint!("{}", &help_message);
                                // let window = protocol::WindowData {
                                //     id:      "help".to_string(),
                                //     content: "T".to_string(),
                                //     message: message.to_string(),
                                //     x_pos:   10,
                                //     y_pos:   10,
                                //     width:   40,
                                //     height:  20,
                                //     priority: true,
                                // };
                                // sender.send(Event::Gui(gui::GuiEvent::CreateWindow(window))).unwrap();
                            }
                            _ => {
                                invalid_command(&sender);
                            }
                        }
                    } else {
                        invalid_command(&sender);
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
        }
        fn notify(info: &str, sender: &Sender<Event>) {
            sender.send(Event::Gui(gui::GuiEvent::Log(info.to_string()))).unwrap();
        }
        fn invalid_command(sender: &Sender<Event>) {
            sender.send(Event::Gui(gui::GuiEvent::Log("Invalid command received.".to_string(),))).unwrap();
        }
    }
}
