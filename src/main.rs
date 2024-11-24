extern crate x11;

use std::cmp::max;
use std::ffi::{c_char, c_int, c_uint, CStr};
use std::process::exit;
use std::{mem, ptr};
use x11::xlib;

fn close_quit(display: *mut xlib::Display, code: i32) {
    unsafe {
        xlib::XCloseDisplay(display);
        exit(code);
    }
}

unsafe extern "C" fn handle_wm_open(display: *mut xlib::Display, xerror: *mut xlib::XErrorEvent) -> c_int {
    if (*xerror).error_code == xlib::BadAccess {
        eprintln!("WM already existed on Display {:?} :despair:",  CStr::from_ptr(xlib::XDisplayString(display)));
        exit(1);
    }
    0
}

unsafe extern "C" fn error(display: *mut xlib::Display, xerror: *mut xlib::XErrorEvent) -> c_int {
    let less_pain: String = String::with_capacity(1024);
    xlib::XGetErrorText(display, (*xerror).error_code as c_int, less_pain.as_ptr() as *mut c_char, 1024);
    eprintln!("Error {:?} on display {:?}", less_pain, CStr::from_ptr(xlib::XDisplayString(display)));
    0
}

fn main() {
    // Xlib is apparently just unsafe entirely so fuck it we ball.
    // I'll rewrite this in XCB once I understand
    // what is even going on with any of the things I am using
    let display = unsafe { xlib::XOpenDisplay(ptr::null()) };
    if display.is_null() {
        eprintln!("Failed to open X display!");
        exit(1);
    }
    let root = unsafe { xlib::XDefaultRootWindow(display) };
    unsafe { println!("Connected to display {:?}", CStr::from_ptr(xlib::XDisplayString(display))); }

    // Maybe an FFI project as my first real one was a bad plan...
    unsafe {
        //xlib::XSetErrorHandler(Some(handle_wm_open));
        //xlib::XSelectInput(display, root, xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask);
        //::XSync(display, false as c_int);

        xlib::XSetErrorHandler(Some(error));

        xlib::XGrabButton(display, 1, xlib::Mod1Mask, root, 1, (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);
        xlib::XGrabButton(display, 1, xlib::ShiftMask, root, 1, xlib::ButtonPressMask as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);
        xlib::XGrabButton(display, 3, xlib::Mod1Mask, root, 1, (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);
    }

    // Rust was a mistake Rust was a mistake Rust was a mistake
    let mut start_drag: Option<(c_int, c_int, c_uint, xlib::Window)> = None;
    let mut window_attributes: xlib::XWindowAttributes = unsafe { mem::zeroed() };
    let mut top_window: xlib::Window = 0;
    loop {
        let mut event: xlib::XEvent = unsafe { mem::zeroed() };
        unsafe { xlib::XNextEvent(display, &mut event); }
        match event.get_type() {
            // Atp there is a comical amount of unsafe blocks.
            // Is this even worth doing in Rust?
            xlib::ButtonPress => unsafe {
                if event.button.subwindow != 0 {
                    if (event.button.state & xlib::Mod1Mask) == xlib::Mod1Mask {
                        xlib::XGetWindowAttributes(display, event.button.subwindow, &mut window_attributes);
                        start_drag = Option::from((event.button.x_root, event.button.y_root, event.button.button, event.button.subwindow));
                    } else if top_window != event.button.subwindow { // Assumed state = ShiftMask
                        xlib::XRaiseWindow(display, event.button.subwindow);
                        top_window = event.button.subwindow;
                    }
                }
            },
            xlib::MotionNotify => unsafe {
                if start_drag.is_some() {
                    let x_difference = event.button.x_root - start_drag.unwrap().0;
                    let y_difference = event.button.y_root - start_drag.unwrap().1;
                    if start_drag.unwrap().2 == xlib::Button1 {
                        xlib::XMoveWindow(display, start_drag.unwrap().3, x_difference, y_difference);
                    } else if start_drag.unwrap().2 == xlib::Button3 {
                        xlib::XResizeWindow(display, start_drag.unwrap().3,
                                            max(1, window_attributes.width + x_difference) as c_uint,
                                            max(1, window_attributes.height + y_difference) as c_uint);
                    }
                }
            }
            xlib::ButtonRelease => unsafe {
                if event.button.subwindow == 0 {
                    start_drag = None;
                }
            },
            _ => {}
        };
    }

    close_quit(display, 0);
}