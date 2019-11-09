use std::fs::File;
use std::io;
use std::io::prelude::*;

mod gui;
mod i2c;
mod protocol;
mod shared;

fn main() -> std::io::Result<()> {
    let mut file = File::create("/tmp/altctrl.serial")?;
    file.write_all(b"Hello, world!")?;
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
    Ok(())
}
