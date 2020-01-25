use altctrl::{gui, i2c, AltctrlInterface, Event};
use clap::{App, Arg};
use std::sync::mpsc;
use std::thread;

// Main thread of altctrl
// Handles the setup of all the modules in the project
// Handles message schedule
fn main() {
    let matches = App::new("altctrl")
        .arg(
            Arg::with_name("interface")
                .short("i")
                .long("interface")
                .help("Set the interface for altctl.")
                .takes_value(true)
                .default_value("chungo"),
        )
        .arg(
            Arg::with_name("disable-i2c")
                .short("d")
                .long("disable-i2c")
                .help("Disable i2c module."),
        )
        .get_matches();

    let interface: Box<dyn AltctrlInterface + Send> = match matches.value_of("interface") {
        Some("chungo") => Box::new(altctrl::Chungo::new("/dev/ttyGS0")),
        Some("garfanzo") => Box::new(altctrl::Garfanzo),
        Some("fatkhiyev") => Box::new(altctrl::Fatkhiyev::new("0.0.0.0:6969")),
        Some(_) | None => {
            println!("Invalid interface (or no interface provided)");
            return;
        }
    };

    let disable_i2c = matches.is_present("disable-i2c");

    //Outcoming message channel for all modules
    let (tx, rx) = mpsc::channel();

    //Incoming message channels for specific modules
    let (gui_tx, gui_rx) = mpsc::channel();
    let (serial_tx, serial_rx) = mpsc::channel();

    let clone_tx = tx.clone();

    //Launch the interface module
    thread::spawn(move || {
        interface.launch(clone_tx, serial_rx);
    });

    let clone_tx = tx.clone();

    //Launch the gui module
    thread::spawn(move || {
        gui::launch(clone_tx, gui_rx);
    });

    //Initalizes the data structure for the i2c module

    let mut i2c_struct = None;
    if !disable_i2c {
        i2c_struct = Some(i2c::initialize(tx.clone()));
    }

    //Handles message scheduling in a loop
    for event in rx.iter() {
        match event {
            Event::I2C(i2c_event) => {
                if let Some(structure) = i2c_struct.as_mut() {
                    i2c::handle(i2c_event, structure);
                }
            }
            Event::Serial(serial_event) => serial_tx.send(serial_event).unwrap(),
            Event::Gui(gui_event) => gui_tx.send(gui_event).unwrap(),
        }
    }
}
