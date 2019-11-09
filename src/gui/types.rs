#![allow(dead_code)]

pub struct NewWindow {
    id: u32,
    content: String,
    start_x: i32,
    start_y: i32,
    window_width: i32,
    window_height: i32,
}

pub enum GuiMsg {
    CreateWindow(NewWindow),
    DestroyWindow(u32),
}
