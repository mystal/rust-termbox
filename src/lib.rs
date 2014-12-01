#![desc = "A Rust wrapper for the termbox library"]
#![license = "MIT"]
#![crate_type = "lib" ]

#![feature(globs)]
#![feature(phase)]

#[phase(plugin, link)] extern crate log;
extern crate libc;

use std::task;
use std::char;

pub use libc::types::os::arch::c95::{c_int, c_uint};

/*
 *
 * A lightweight curses alternative wrapping the termbox library.
 *
 * # SYNOPSIS
 *
 * A hello world for the terminal:
 *
 *     use std;
 *     use termbox;
 *
 *     import tb = termbox;
 *
 *     fn main() {
 *         tb::init();
 *         tb::print(1, 1, tb::bold, tb::white, tb::black, "Hello, world!");
 *         tb::present();
 *         std::timer::sleep(std::uv_global_loop::get(), 1000);
 *         tb::shutdown();
 *     }
 *
 * # DESCRIPTION
 *
 * Output is double-buffered.
 *
 * TODO
 *
 * # EXAMPLES
 *
 * TODO
 *
 */

// Exported functions
// export init, shutdown
//      , width, height
//      , clear, present
//      , set_cursor
//      , print, print_ch
//      , poll_event, peek_event
//      , event;

// Exported types
// export color, style
//      , event;



/*
 * The event type matches struct tb_event from termbox.h
 */
pub struct RawEvent {
    etype: u8,
    emod: u8,
    key: u16,
    ch: u32,
    w: i32,
    h: i32,
}

/*
 * Foreign functions from termbox.
 */
mod c {
    use libc::types::os::arch::c95::{ c_int, c_uint};

    #[link(name = "termbox")]
    extern {
        pub fn tb_init() -> c_int;
        pub fn tb_shutdown();

        pub fn tb_width() -> c_uint;
        pub fn tb_height() -> c_uint;

        pub fn tb_clear();
        pub fn tb_present();

        pub fn tb_set_cursor(cx: c_int, cy: c_int);

        pub fn tb_change_cell(x: c_uint, y: c_uint, ch: u32, fg: u16, bg: u16);

        pub fn tb_select_input_mode(mode: c_int) -> c_int;
        pub fn tb_set_clear_attributes(fg: u16, bg: u16);

        pub fn tb_peek_event(ev: *const ::RawEvent, timeout: c_uint) -> c_int;
        pub fn tb_poll_event(ev: *const ::RawEvent) -> c_int;
    }
}

pub fn init() -> int {
    unsafe { c::tb_init() as int }
}

pub fn shutdown() {
    unsafe { c::tb_shutdown(); }
}

pub fn width() -> uint {
    unsafe {
        return  c::tb_width() as uint;
    }
}

pub fn height() -> uint {
    unsafe {
        return  c::tb_height() as uint;
    }
}

/**
 * Clear buffer.
 */

pub fn clear() {
    unsafe {
        c::tb_clear();
    }
}

// /**
//  * Write buffer to terminal.
//  */

pub fn present() {
    unsafe {
        c::tb_present();
    }
}

pub fn set_cursor(cx: uint, cy: uint) {
    unsafe {
        c::tb_set_cursor(cx as c_int, cy as c_int);
    }
}

// low-level wrapper
pub fn change_cell(x: uint, y: uint, ch: u32, fg: u16, bg: u16) {
    unsafe {
        c::tb_change_cell(x as c_uint, y as c_uint, ch, fg, bg);
    }
}

/// Convert from enums to u16
pub fn convert_color(c: Color) -> u16 {
    match c {
        Color::Default => 0x00,
        Color::Black => 0x01,
        Color::Red => 0x02,
        Color::Green => 0x03,
        Color::Yellow => 0x04,
        Color::Blue => 0x05,
        Color::Magenta => 0x06,
        Color::Cyan => 0x07,
        Color::White => 0x08,
    }
}

pub fn convert_style(sty: Style) -> u16 {
    match sty {
        Style::Normal => 0x0000,
        Style::Bold => 0x0100,
        Style::Underline => 0x0200,
        Style::BoldUnderline => 0x0300,
        Style::Reverse => 0x0400,
        Style::BoldReverse => 0x0500,
        Style::UnderlineReverse => 0x0600,
        Style::BoldUnderlineReverse => 0x700,
    }
}

pub fn reverse_convert_key(k: u16) -> Option<Key> {
    match k {
        65535 => Some(Key::F1),
        65534 => Some(Key::F2),
        65533 => Some(Key::F3),
        65532 => Some(Key::F4),
        65531 => Some(Key::F5),
        65530 => Some(Key::F6),
        65529 => Some(Key::F7),
        65528 => Some(Key::F8),
        65527 => Some(Key::F9),
        65526 => Some(Key::F10),
        65525 => Some(Key::F11),
        65524 => Some(Key::F12),
        65523 => Some(Key::Insert),
        65522 => Some(Key::Delete),
        65521 => Some(Key::Home),
        65520 => Some(Key::End),
        65519 => Some(Key::Pgup),
        65518 => Some(Key::Pgdn),
        65517 => Some(Key::ArrowUp),
        65516 => Some(Key::ArrowDown),
        65515 => Some(Key::ArrowLeft),
        65514 => Some(Key::ArrowRight),
        0 => Some(Key::CtrlTilde),
        //0 => Some(ctrl2),
        1 => Some(Key::CtrlA),
        2 => Some(Key::CtrlB),
        3 => Some(Key::CtrlC),
        4 => Some(Key::CtrlD),
        5 => Some(Key::CtrlE),
        6 => Some(Key::CtrlF),
        7 => Some(Key::CtrlG),
        8 => Some(Key::Backspace),
        //8 => Some(ctrlH),
        9 => Some(Key::Tab),
        //9 => Some(ctrlI),
        10 => Some(Key::CtrlJ),
        11 => Some(Key::CtrlK),
        12 => Some(Key::CtrlL),
        13 => Some(Key::Enter),
        //13 => Some(ctrlM),
        14 => Some(Key::CtrlN),
        15 => Some(Key::CtrlO),
        16 => Some(Key::CtrlP),
        17 => Some(Key::CtrlQ),
        18 => Some(Key::CtrlR),
        19 => Some(Key::CtrlS),
        20 => Some(Key::CtrlT),
        21 => Some(Key::CtrlU),
        22 => Some(Key::CtrlV),
        23 => Some(Key::CtrlW),
        24 => Some(Key::CtrlX),
        25 => Some(Key::CtrlY),
        26 => Some(Key::CtrlZ),
        27 => Some(Key::Esc),
        //27 => Some(ctrlLsqBracket),
        //27 => Some(ctrl3),
        28 => Some(Key::Ctrl4),
        //28 => Some(ctrlBackslash),
        29 => Some(Key::Ctrl5),
        //29 => Some(ctrlRsqBracket),
        30 => Some(Key::Ctrl6),
        31 => Some(Key::Ctrl7),
        //31 => Some(ctrlSlash),
        //31 => Some(ctrlUnderscore),
        32 => Some(Key::Space),
        127 => Some(Key::Backspace2),
        //127 => ctrl8
        _ => None
    }
}
/**
 * Print a string to the buffer.  Leftmost charater is at (x, y).
 */

pub fn print(x: uint, y: uint, sty: Style, fg: Color, bg: Color, s: &str) {
    let fg: u16 = convert_color(fg) | convert_style(sty);
    let bg: u16 = convert_color(bg);
    for (i, ch) in s.chars().enumerate() {
        unsafe {
            c::tb_change_cell((x + i) as c_uint, y as c_uint, ch as u32, fg, bg);
        }
    }
}

// /**
//  * Print a charater to the buffer.
//  */

pub fn print_ch(x: uint, y: uint, sty: Style, fg: Color, bg: Color, ch: char) {
    unsafe {
        let fg: u16 = convert_color(fg) | convert_style(sty);
        let bg: u16 = convert_color(bg);
        c::tb_change_cell(x as c_uint, y as c_uint, ch as u32, fg, bg);
    }
}

#[deriving(Show, Eq, PartialEq)]
pub enum Key {
    F1 = 65535,
    F2 = 65534,
    F3 = 65533,
    F4 = 65532,
    F5 = 65531,
    F6 = 65530,
    F7 = 65529,
    F8 = 65528,
    F9 = 65527,
    F10 = 65526,
    F11 = 65525,
    F12 = 65524,
    Insert = 65523,
    Delete = 65522,
    Home = 65521,
    End = 65520,
    Pgup = 65519,
    Pgdn = 65518,
    ArrowUp = 65517,
    ArrowDown = 65516,
    ArrowLeft = 65515,
    ArrowRight = 65514,
    CtrlTilde = 0,
    // ctrl2 = 0,
    CtrlA = 1,
    CtrlB = 2,
    CtrlC = 3,
    CtrlD = 4,
    CtrlE = 5,
    CtrlF = 6,
    CtrlG = 7,
    Backspace = 8,
    //ctrlH = 8,
    Tab = 9,
    //ctrlI = 9,
    CtrlJ = 10,
    CtrlK = 11,
    CtrlL = 12,
    Enter = 13,
    //ctrlM = 13,
    CtrlN = 14,
    CtrlO = 15,
    CtrlP = 16,
    CtrlQ = 17,
    CtrlR = 18,
    CtrlS = 19,
    CtrlT = 20,
    CtrlU = 21,
    CtrlV = 22,
    CtrlW = 23,
    CtrlX = 24,
    CtrlY = 25,
    CtrlZ = 26,
    Esc = 27,
    //ctrlLsqBracket = 27,
    //ctrl3 = 27,
    Ctrl4 = 28,
    //ctrlBackslash = 28,
    Ctrl5 = 29,
    //ctrlRsqBracket = 29,
    Ctrl6 = 30,
    Ctrl7 = 31,
    //ctrlSlash = 31,
    //ctrlUnderscore = 31,
    Space = 32,
    Backspace2 = 127,
    //ctrl_8 = 127
}

#[deriving(Show, Eq, PartialEq)]
pub enum Color {
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White
}

#[deriving(Show, Eq, PartialEq)]
pub enum Style {
    Normal,
    Bold,
    Underline,
    BoldUnderline,
    Reverse,
    BoldReverse,
    UnderlineReverse,
    BoldUnderlineReverse
}

//Convenience functions
pub fn with_term(f: proc():Send) {
    init();
    let res = task::try(f);
    shutdown();
    match res {
        Err(_) => {
            error!("with_term: An error occured.");
        }
        _ => {}
    }
}

pub fn nil_raw_event() -> RawEvent {
    RawEvent{etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0}
}

#[deriving(Show, Eq, PartialEq)]
pub enum Event {
    KeyEvent(u8, Option<Key>, Option<char>),
    ResizeEvent(i32, i32),
    NoEvent
}

/**
 * Get an event if within timeout milliseconds, otherwise return urn NoEvent.
 */


pub fn peek_event(timeout: uint) -> Event {
    unsafe {
        let ev = nil_raw_event();
        let rc = c::tb_peek_event(&ev, timeout as c_uint);
        return unpack_event(rc, &ev);
    }
}

// /**
//  * Blocking function to return urn next event.
//  */

pub fn poll_event() -> Event {
    unsafe {
        let ev = nil_raw_event();
        let rc = c::tb_poll_event(&ev);
        return unpack_event(rc, &ev);
    }
}

// /* helper pub fn
//  *
//  * ev_type
//  *   0 -> no event
//  *   1 -> key
//  *   2 -> resize
//  *   -1 -> error
//  */
pub fn unpack_event(ev_type: c_int, ev: &RawEvent) -> Event {
    match ev_type {
        0 => Event::NoEvent,
        1 => Event::KeyEvent(ev.emod, reverse_convert_key(ev.key), char::from_u32(ev.ch)),
        2 => Event::ResizeEvent(ev.w, ev.h),
        _ => Event::NoEvent
    }
}
