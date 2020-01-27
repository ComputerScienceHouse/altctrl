use crate::{AltctrlInterface, Event, SerialEvent, protocol};
use std::sync::mpsc::{Sender, Receiver};
use std::io::BufRead;
use clap::{App, SubCommand, Arg, ArgMatches, AppSettings};
use crate::gui::GuiEvent;

pub struct Garfanzo {
    event_queue: Sender<Event>,
}

impl Garfanzo {
    pub fn new(event_queue: Sender<Event>) -> Garfanzo {
        Garfanzo { event_queue }
    }

    pub fn repl<B: BufRead>(&self, reader: B) {
        let mut app = Self::app();
        for line in reader.lines() {
            let line = match line {
                Err(_) => return,
                Ok(line) => line,
            };
            let command = line.split(" ");
            let matches = app.get_matches_from_safe_borrow(command);
            match matches {
                Ok(matches) => self.execute(&matches),
                _ => {
                    Self::app().write_help(&mut std::io::stderr()).unwrap();
                    eprintln!();
                },
            }
        }
    }

    pub fn app<'a, 'b>() -> App<'a, 'b> {
        App::new("garfanzo")
            .setting(AppSettings::UnifiedHelpMessage)
            .setting(AppSettings::NoBinaryName)
            .subcommand(Self::subcommand_log())
            .subcommand(Self::subcommand_window())
            .subcommand(SubCommand::with_name("redraw")
                .about("Redraws the entire screen"))
            .subcommand(SubCommand::with_name("clear")
                .about("Clears the screen"))
    }

    fn subcommand_log<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("log")
            .about("Controls the log console")
            .arg(Arg::with_name("toggle")
                .help("Toggle the log window")
                .long("--toggle").short("t")
                .takes_value(false))
            .arg(Arg::with_name("message")
                .help("The message to print to the log window")
                .index(1)
                .takes_value(true))
    }

    fn subcommand_window<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("window")
            .about("Creates and controls windows")
            .subcommand(Self::subcommand_window_new())
            .subcommand(Self::subcommand_window_close())
            .subcommand(Self::subcommand_window_move())
            .subcommand(Self::subcommand_window_resize())
            .subcommand(SubCommand::with_name("list").alias("ls")
                .about("Lists all of the existing windows"))
    }

    fn subcommand_window_new<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("new")
            .about("Creates a new window")
            .arg(Arg::with_name("id").help("The ID of the window to create")
                .index(1).takes_value(true))
            // TODO fill in content help
            .arg(Arg::with_name("content").help("TODO I don't know what this does")
                .index(2).takes_value(true))
            // TODO fill in message help
            .arg(Arg::with_name("message").help("TODO I don't know what this does")
                .index(3).takes_value(true))
            // TODO fill in style help
            .arg(Arg::with_name("style").help("TODO I don't know what this does")
                .index(4).takes_value(true))
            .arg(Arg::with_name("x_pos").help("The X coordinate of the window to create")
                .index(5).takes_value(true)
                .validator(|x_pos| x_pos.parse::<i32>().map(|_| ()).map_err(|_| format!("x_pos must be an integer"))))
            .arg(Arg::with_name("y_pos").help("The Y coordinate of the window to create")
                .index(6).takes_value(true)
                .validator(|y_pos| y_pos.parse::<i32>().map(|_| ()).map_err(|_| format!("y_pos must be an integer"))))
            .arg(Arg::with_name("width").help("The width of the window to create")
                .index(7).takes_value(true)
                .validator(|w| w.parse::<i32>().map(|_| ()).map_err(|_| format!("width must be an integer"))))
            .arg(Arg::with_name("height").help("The height of the window to create")
                .index(8).takes_value(true)
                .validator(|h| h.parse::<i32>().map(|_| ()).map_err(|_| format!("height must be an integer"))))
            .arg(Arg::with_name("priority").help("TODO I don't know what this does")
                .index(9).takes_value(false).short("!"))
    }

    fn subcommand_window_close<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("close")
            .about("Closes an existing window")
            .arg(Arg::with_name("id").help("The ID of the window to close")
                .index(1).takes_value(true))
    }

    fn subcommand_window_move<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("move")
            .about("Move an existing window to new coordinates")
            .arg(Arg::with_name("id").help("The ID of the window to move")
                .index(1).takes_value(true))
            .arg(Arg::with_name("x_pos").help("The X coordinate to move the window to")
                .index(2).takes_value(true))
            .arg(Arg::with_name("y_pos").help("The Y coordinate to move the window to")
                .index(3).takes_value(true))
    }

    fn subcommand_window_resize<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("resize")
            .about("Resizes an existing window")
            .arg(Arg::with_name("id").help("The ID of the window to resize")
                .index(1).takes_value(true))
            .arg(Arg::with_name("width").help("The new width for the window")
                .index(2).takes_value(true)
                .validator(|w| w.parse::<i32>().map(|_| ()).map_err(|_| format!("width must be an integer"))))
            .arg(Arg::with_name("height").help("The new height for the window")
                .index(3).takes_value(true)
                .validator(|h| h.parse::<i32>().map(|_| ()).map_err(|_| format!("height must be an integer"))))
    }

    pub fn execute(&self, args: &ArgMatches) {
        match args.subcommand() {
            ("log", Some(log_args)) => self.execute_log(log_args),
            ("window", Some(window_args)) => self.execute_window(window_args),
            ("redraw", _) => self.execute_redraw(),
            ("clear", _) => self.execute_clear(),
            _ => {
                Self::app().write_help(&mut std::io::stderr()).unwrap();
                eprintln!();
            },
        }
    }

    fn execute_log(&self, args: &ArgMatches) {
        if args.is_present("toggle") {
            self.event_queue.send(Event::Gui(GuiEvent::ToggleConsole())).unwrap();
            return;
        }

        if let Some(message) = args.value_of("message") {
            self.log(message);
            return;
        }

        Self::subcommand_log().write_help(&mut std::io::stderr()).unwrap();
        eprintln!();
    }

    fn log<S: AsRef<str>>(&self, msg: S) {
        let msg = msg.as_ref();
        self.event_queue.send(Event::Gui(GuiEvent::Log(msg.to_string()))).unwrap();
    }

    fn execute_window(&self, args: &ArgMatches) {
        match args.subcommand() {
            ("new", Some(new_args)) => self.execute_window_new(new_args),
            ("close", Some(close_args)) => self.execute_window_close(close_args),
            ("move", Some(move_args)) => self.execute_window_move(move_args),
            ("list", _) => self.event_queue.send(Event::Gui(GuiEvent::List())).unwrap(),
            ("resize", Some(resize_args)) => self.execute_window_resize(resize_args),
            _ => {
                Self::subcommand_window().write_help(&mut std::io::stderr()).unwrap();
                eprintln!();
            },
        }
    }

    fn execute_window_new(&self, args: &ArgMatches) {
        let id = args.value_of("id");
        let content = args.value_of("content");
        let message = args.value_of("message");
        let style = args.value_of("style");
        let x_pos = args.value_of("x_pos");
        let y_pos = args.value_of("y_pos");
        let width = args.value_of("width");
        let height = args.value_of("height");
        let priority = args.is_present("priority");

        let required_args = vec![id, content, message, style, x_pos, y_pos, width, height];
        if required_args.iter().any(|item| item.is_none()) {
            Self::subcommand_window_new().write_help(&mut std::io::stderr()).unwrap();
            eprintln!();
            return;
        }

        // Required args have been checked, unwrap all.
        let id = id.unwrap().to_string();
        let content = content.unwrap().to_string();
        let message = message.unwrap().to_string();
        let style = style.unwrap().to_string();
        let x_pos = x_pos.unwrap();
        let y_pos = y_pos.unwrap();
        let width = width.unwrap();
        let height = height.unwrap();

        // These already passed validation so unwrapping is safe
        let x_pos: i32 = x_pos.parse().unwrap();
        let y_pos: i32 = y_pos.parse().unwrap();
        let width: i32 = width.parse().unwrap();
        let height: i32 = height.parse().unwrap();

        self.log(format!("Creating window \"{}\"", &id));
        let window = protocol::WindowData {
            id, content, message, style, x_pos, y_pos, width, height, priority
        };
        self.event_queue.send(Event::Gui(GuiEvent::CreateWindow(window))).unwrap();
    }

    fn execute_window_close(&self, args: &ArgMatches) {
        let id = args.value_of("if");
        if id.is_none() {
            Self::subcommand_window_close().write_help(&mut std::io::stderr()).unwrap();
            eprintln!();
            return;
        }

        let id = id.unwrap().to_string();
        self.log(format!("Closing window \"{}\"", id));
        self.event_queue.send(Event::Gui(GuiEvent::DestroyWindow(id))).unwrap();
    }

    fn execute_window_move(&self, args: &ArgMatches) {
        let id = args.value_of("id");
        let x_pos = args.value_of("x_pos");
        let y_pos = args.value_of("y_pos");

        let required_args = vec![id, x_pos, y_pos];
        if required_args.iter().any(|item| item.is_none()) {
            Self::subcommand_window_move().write_help(&mut std::io::stderr()).unwrap();
            eprintln!();
            return;
        }

        let id = id.unwrap().to_string();
        let x_pos = x_pos.unwrap();
        let y_pos = y_pos.unwrap();

        let x_pos: i32 = x_pos.parse().unwrap();
        let y_pos: i32 = y_pos.parse().unwrap();
        self.log(format!("Moving window \"{}\"", id));
        self.event_queue.send(Event::Gui(GuiEvent::MoveWindow(id, x_pos, y_pos))).unwrap();
    }

    fn execute_window_resize(&self, args: &ArgMatches) {
        let id = args.value_of("id");
        let width = args.value_of("width");
        let height = args.value_of("height");

        let required_args = vec![id, width, height];
        if required_args.iter().any(|item| item.is_none()) {
            Self::subcommand_window_resize().write_help(&mut std::io::stderr()).unwrap();
            eprintln!();
            return;
        }

        let id = id.unwrap().to_string();
        let width = width.unwrap();
        let height = height.unwrap();

        let width: i32 = width.parse().unwrap();
        let height: i32 = height.parse().unwrap();
        self.log(format!("Resizing window \"{}\"", id));
        self.event_queue.send(Event::Gui(GuiEvent::ResizeWindow(id, width, height))).unwrap();
    }

    fn execute_redraw(&self) {
        self.event_queue.send(Event::Gui(GuiEvent::Redraw())).unwrap();
    }

    fn execute_clear(&self) {
        self.event_queue.send(Event::Gui(GuiEvent::Clear())).unwrap();
        self.log("Screen cleared.");
    }
}

impl AltctrlInterface for Garfanzo {
    fn launch(&self, _: Sender<Event>, _: Receiver<SerialEvent>) {
        self.repl(std::io::stdin().lock());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::ErrorKind;

    #[test]
    fn test_subcommand_window() {
        let mut app = Garfanzo::app();
        let command = vec!["window", "new", "0", "Hello, world", "Message", "bold", "10", "15", "20", "25"];
        let result = app.get_matches_from_safe_borrow(command);
        assert!(result.is_ok());

        let matches = result.unwrap();
        assert!(matches.subcommand_matches("window").is_some());
        if let ("window", Some(sub)) = matches.subcommand() {
            if let ("new", Some(sub)) = sub.subcommand() {
                assert_eq!(sub.value_of("id"), Some("0"));
                assert_eq!(sub.value_of("content"), Some("Hello, world"));
                assert_eq!(sub.value_of("message"), Some("Message"));
                assert_eq!(sub.value_of("style"), Some("bold"));
                assert_eq!(sub.value_of("x_pos"), Some("10"));
                assert_eq!(sub.value_of("y_pos"), Some("15"));
                assert_eq!(sub.value_of("width"), Some("20"));
                assert_eq!(sub.value_of("height"), Some("25"));
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_int_validator() {
        let mut app = Garfanzo::app();
        let command = vec!["window", "new", "0", "Hello, world", "message", "bold", "ten", "15", "20", "25"];
        let result = app.get_matches_from_safe_borrow(command);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.kind, ErrorKind::ValueValidation);
    }

    #[test]
    fn test_priority() {
        let mut app = Garfanzo::app();
        let command = vec!["window", "new", "0", "Hello", "message", "bold", "10", "20", "30", "40", "!"];
        let result = app.get_matches_from_safe_borrow(command);
        assert!(result.is_ok());
        let matches = result.unwrap();
        if let ("window", Some(win_sub)) = matches.subcommand() {
            if let ("new", Some(new_sub)) = win_sub.subcommand() {
                assert!(new_sub.is_present("priority"));
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }
}
