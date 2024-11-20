extern crate x11;

use std::process::exit;
use std::ptr;
use x11::xlib;

fn main() {
    println!("Hello, josh!");
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            eprintln!("Failed to open X display!");
            exit(1);
        }
        println!("horay!");
        xlib::XCloseDisplay(display);
    }
}
