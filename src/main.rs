// The majority of this project is somewhat lifted from TinyWM and jichu4n's basic_wm.
// And when I say lifted I mean not at all because both are better written and in C and C++ respectively.
// FFI types are my new reason to continue hating Rust, but the syntax is starting to catch on me.
extern crate x11;

use std::cmp::max;
use std::ffi::{c_char, c_int, c_uint, CStr};
use std::process::exit;
use std::{mem, ptr};
use std::collections::HashMap;
use std::os::raw::c_ulong;
use x11::{keysym, xlib};
use x11::xlib::{SubstructureNotifyMask, SubstructureRedirectMask};

fn close_quit(display: *mut xlib::Display, code: i32) {
    unsafe {
        xlib::XCloseDisplay(display);
        exit(code);
    }
}

unsafe extern "C" fn handle_wm_open(display: *mut xlib::Display, xerror: *mut xlib::XErrorEvent) -> c_int {
    if (*xerror).error_code == xlib::BadAccess {
        eprintln!("WM already existed on display {:?}",  CStr::from_ptr(xlib::XDisplayString(display)));
        exit(1);
    }
    0
}

// I don't get rust yet this project was mistake.
unsafe extern "C" fn error(display: *mut xlib::Display, xerror: *mut xlib::XErrorEvent) -> c_int {
    let mut v = Vec::<u8>::with_capacity(1025); // I don't know if I need to account for null terminate but why not
    xlib::XGetErrorText(display, (*xerror).error_code as c_int, v.as_mut_ptr() as *mut c_char, 1024);
    eprintln!("Error {:?} on display {:?}", std::ffi::CString::from_raw(v.as_mut_ptr() as *mut c_char), CStr::from_ptr(xlib::XDisplayString(display)));
    mem::forget(v); // This probably causes a memory leak. However, for some reason v doesn't exist after I from_raw it.
    0
}

fn get_window_attributes(display: *mut xlib::Display, window: xlib::Window) -> xlib::XWindowAttributes {
    let mut window_attributes: xlib::XWindowAttributes = unsafe { mem::zeroed() };
    unsafe { xlib::XGetWindowAttributes(display, window, &mut window_attributes); }
    window_attributes
}

fn frame_window(display: *mut xlib::Display, root_window: std::ffi::c_ulong, window: xlib::Window) -> c_ulong {
    let border_thickness: u32 = 2;
    let top_size: u32 = 0;
    let top_size_real: u32 = max(top_size as i64 - border_thickness as i64, 0) as u32;
    let border_color: u64 = 0xEFEFFF;

    let attr = get_window_attributes(display, window);

    let frame = unsafe {
        xlib::XCreateSimpleWindow(
            display,
            root_window,
            attr.x,
            attr.y,
            attr.width as u32,
            top_size_real + attr.height as u32,
            border_thickness,
            border_color,
            border_color
        )
    };
    unsafe {
        xlib::XSelectInput(display, frame, SubstructureNotifyMask | SubstructureRedirectMask);
        xlib::XAddToSaveSet(display, window);
        xlib::XReparentWindow(display, window, frame, 0, top_size_real as c_int);
        xlib::XMapWindow(display, frame);

        // XQuartz didn't like alt-f4 so I just did ctrl-q
        xlib::XGrabKey(display, xlib::XKeysymToKeycode(display, keysym::XK_Q as std::ffi::c_ulong) as c_int, xlib::ControlMask, window, false as c_int, xlib::GrabModeAsync, xlib::GrabModeAsync);

        xlib::XGrabButton(display, 1, xlib::Mod1Mask, frame, 1, (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);
        xlib::XGrabButton(display, 1, xlib::ShiftMask, frame, 1, xlib::ButtonPressMask as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);
        xlib::XGrabButton(display, 3, xlib::Mod1Mask, window, 1, (xlib::ButtonPressMask|xlib::ButtonReleaseMask|xlib::PointerMotionMask) as c_uint, xlib::GrabModeAsync, xlib::GrabModeAsync, 0, 0);
    }
    frame
}

unsafe fn unframe_window(display: *mut xlib::Display, root_window: std::ffi::c_ulong, window: std::ffi::c_ulong, frame: std::ffi::c_ulong) {
    xlib::XUnmapWindow(display, frame);

    // The parameter should be correct by all means but it isn't.
    // X server keeps giving me shit about BadWindow even though it's correct code (i think, probably not tho).
    // Thus I will not care about doing either of these things that a normal WM should do.
    // xlib::XReparentWindow(display, window, root_window, 0, 0);
    // xlib::XRemoveFromSaveSet(display, window);

    xlib::XDestroyWindow(display, frame);
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
        xlib::XSetErrorHandler(Some(handle_wm_open));
        xlib::XSelectInput(display, root, xlib::SubstructureRedirectMask | xlib::SubstructureNotifyMask);
        xlib::XSync(display, false as c_int);

        xlib::XSetErrorHandler(Some(error));
    }

    // Rust was a mistake Rust was a mistake Rust was a mistake
    let mut start_drag: Option<(c_int, c_int, c_uint, xlib::Window)> = None;
    let mut window_attributes: xlib::XWindowAttributes = unsafe { mem::zeroed() };
    let mut focused_window: xlib::Window = 0;
    let mut frames: HashMap<std::ffi::c_ulong, std::ffi::c_ulong> = HashMap::new();
    loop {
        let mut event: xlib::XEvent = unsafe { mem::zeroed() };
        // Atp there is a comical amount of unsafe blocks.
        // Is this even worth doing in Rust?
        unsafe { xlib::XNextEvent(display, &mut event); }
        match event.get_type() {
            // All my mouse button besties
            xlib::ButtonPress => unsafe {
                if event.button.subwindow != 0 {
                    if (event.button.state & xlib::Mod1Mask) == xlib::Mod1Mask {
                        window_attributes = get_window_attributes(display, event.button.window);
                        start_drag = Option::from((event.button.x_root, event.button.y_root, event.button.button, event.button.window));
                    }
                    if focused_window != event.button.window { // Assumed state = ShiftMask or Mod1Mask
                        xlib::XRaiseWindow(display, event.button.window);
                        focused_window = event.button.window;
                    }
                }
            }
            xlib::MotionNotify => unsafe {
                if start_drag.is_some() {
                    let x_difference = event.button.x_root - start_drag.unwrap().0;
                    let y_difference = event.button.y_root - start_drag.unwrap().1;
                    let window = start_drag.unwrap().3;
                    if start_drag.unwrap().2 == xlib::Button1 {
                        xlib::XMoveWindow(display, window, x_difference, y_difference);
                    } else if start_drag.unwrap().2 == xlib::Button3 {
                        xlib::XResizeWindow(display, window,
                                            max(1, window_attributes.width + x_difference) as c_uint,
                                            max(1, window_attributes.height + y_difference) as c_uint);
                        if frames.contains_key(&window) {
                            xlib::XResizeWindow(display, *frames.get(&window).unwrap(),
                                                max(1, window_attributes.width + x_difference) as c_uint,
                                                max(1, window_attributes.height + y_difference) as c_uint);
                        }
                    }
                }
            }
            xlib::ButtonRelease => unsafe {
                if event.button.subwindow == 0 {
                    start_drag = None;
                }
            }
            xlib::KeyPress => unsafe { // Honorary mouse member (key)
                if event.key.keycode == (xlib::XKeysymToKeycode(display, keysym::XK_Q as c_ulong) as c_uint) {
                    xlib::XKillClient(display, event.key.window);
                }
            }

            // Okay, now time to manage the windows.
            xlib::ConfigureRequest => unsafe {
                let mut changes: xlib::XWindowChanges = mem::zeroed();
                changes.x = event.configure_request.x;
                changes.y = event.configure_request.y;
                changes.width = event.configure_request.width;
                changes.height = event.configure_request.height;
                changes.border_width = event.configure_request.border_width;
                changes.sibling = event.configure_request.above;
                changes.stack_mode = event.configure_request.detail;

                if frames.contains_key(&event.configure_request.window) {
                    xlib::XConfigureWindow(display, *frames.get(&event.configure_request.window).unwrap(), event.configure_request.value_mask as c_uint, &mut changes);
                }

                xlib::XConfigureWindow(display, event.configure_request.window, event.configure_request.value_mask as c_uint, &mut changes);

            }
            xlib::MapRequest => unsafe {
                frames.insert(event.map_request.window, frame_window(display, root, event.map_request.window));
                xlib::XMapWindow(display, event.map_request.window);

                focused_window = event.map_request.window;
                xlib::XRaiseWindow(display, event.map_request.window);
            }
            xlib::UnmapNotify => unsafe {
                if frames.contains_key(&event.unmap.window) {
                    unframe_window(display, root, event.unmap.window, *frames.get(&event.unmap.window).unwrap());
                    frames.remove(&event.unmap.window);
                }
            }
            _ => {}
        };
    }

    close_quit(display, 0);
}