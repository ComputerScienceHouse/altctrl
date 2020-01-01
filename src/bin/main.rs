use std::thread;
use std::sync::mpsc;
use altctrl::{i2c, gui, Event, AltctrlInterface};

// Main thread of altctrl
// Handles the setup of all the modules in the project
// Handles message schedule
fn main() {
    let interface = std::env::args().nth(0).expect("should get first argument");

    let interface: Box<dyn AltctrlInterface + Send> = match &*interface {
        "chungo" => Box::new(altctrl::Chungo),
        "garfanzo" => Box::new(altctrl::Garfanzo),
        "fatkhiyev" => Box::new(altctrl::Fatkhiyev),
        _ => {
            println!("Invalid interface!");
            return;
        }
    };

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
    let mut i2c_struct = i2c::initialize(tx.clone());

    //Handles message scheduling in a loop
    for event in rx.iter() {
        match event {
            Event::I2C(i2c_event) => i2c::handle(i2c_event, &mut i2c_struct),
            Event::Serial(serial_event) => serial_tx.send(serial_event).unwrap(),
            Event::Gui(gui_event) => gui_tx.send(gui_event).unwrap(),
        }
    }
}
