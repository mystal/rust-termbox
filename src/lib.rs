//! A lightweight curses alternative wrapping the termbox library.
//!
//! # SYNOPSIS
//!
//! A hello world for the terminal:
//!
//!     use std;
//!     use termbox;
//!
//!     import tb = termbox;
//!
//!     fn main() {
//!         tb::init();
//!         tb::print(1, 1, tb::bold, tb::white, tb::black, "Hello, world!");
//!         tb::present();
//!         std::timer::sleep(std::uv_global_loop::get(), 1000);
//!         tb::shutdown();
//!     }
//!
//! # DESCRIPTION
//!
//! Output is double-buffered.
//!
//! TODO
//!
//! # EXAMPLES
//!
//! TODO

#![feature(phase)]

extern crate libc;
#[phase(plugin, link)] extern crate log;

extern crate "termbox_sys" as ffi;

use libc::c_int;

use std::char;
use std::task;

use ffi::{
    tb_attribute,
    tb_color,
    tb_event_type,
};

#[deriving(Show, Eq, PartialEq, Copy)]
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

#[deriving(Show, Eq, PartialEq, Copy)]
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

#[deriving(Show, Eq, PartialEq, Copy)]
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

#[deriving(Show, Eq, PartialEq, Copy)]
pub enum Event {
    KeyEvent(u8, Option<Key>, Option<char>),
    ResizeEvent(i32, i32),
    NoEvent
}

#[deriving(Copy)]
pub struct Attribute {
    pub color: Color,
    pub style: Style,
}

#[deriving(Copy)]
pub struct Cell {
    pub ch: char,
    pub fg: Attribute,
    pub bg: Attribute,
}

impl Attribute {
    pub fn as_u16(&self) -> u16 {
        convert_color(self.color) | convert_style(self.style)
    }
}

impl Cell {
    pub fn as_raw(&self) -> ffi::tb_cell {
        ffi::tb_cell {
            ch: self.ch as u32,
            fg: self.fg.as_u16(),
            bg: self.bg.as_u16(),
        }
    }
}

pub fn init() -> int {
    unsafe { ffi::tb_init() as int }
}

pub fn shutdown() {
    unsafe { ffi::tb_shutdown(); }
}

pub fn width() -> uint {
    unsafe { ffi::tb_width() as uint }
}

pub fn height() -> uint {
    unsafe { ffi::tb_height() as uint }
}

/// Clear buffer.
pub fn clear() {
    unsafe { ffi::tb_clear(); }
}

/// Write buffer to terminal.
pub fn present() {
    unsafe { ffi::tb_present(); }
}

pub fn set_cursor(cx: uint, cy: uint) {
    unsafe { ffi::tb_set_cursor(cx as c_int, cy as c_int); }
}

pub fn hide_cursor() {
    unsafe { ffi::tb_set_cursor(ffi::TB_HIDE_CURSOR, ffi::TB_HIDE_CURSOR); }
}

pub fn set_cell(x: uint, y: uint, cell: &Cell) {
    unsafe { ffi::tb_put_cell(x as c_int, y as c_int, &cell.as_raw()); }
}

/// Print a charater to the buffer.
pub fn print_ch(x: uint, y: uint, fg: Attribute, bg: Attribute, ch: char) {
    let fg: u16 = fg.as_u16();
    let bg: u16 = bg.as_u16();
    unsafe {
        ffi::tb_change_cell(x as c_int, y as c_int, ch as u32, fg, bg);
    }
}

pub fn print_cells(x: uint, y: uint, cells: &[Cell]) {
    for (i, cell) in cells.iter().enumerate() {
        set_cell(x + i, y, cell);
    }
}

/// Print a string to the buffer. Leftmost charater is at (x, y).
pub fn print_string_styled(x: uint, y: uint, fg: Attribute, bg: Attribute, s: &str) {
    let fg: u16 = fg.as_u16();
    let bg: u16 = bg.as_u16();
    for (i, ch) in s.chars().enumerate() {
        unsafe {
            ffi::tb_change_cell((x + i) as c_int, y as c_int, ch as u32, fg, bg);
        }
    }
}

pub fn print_string(x: uint, y: uint, s: &str) {
    let default = Attribute {
        color: Color::Default,
        style: Style::Normal,
    };
    print_string_styled(x, y, default, default, s);
}

/// Get an event if within timeout milliseconds, otherwise return urn NoEvent.
pub fn peek_event(timeout: uint) -> Event {
    unsafe {
        let ev = nil_raw_event();
        let rc = ffi::tb_peek_event(&ev, timeout as c_int);
        unpack_event(rc, &ev)
    }
}

/// Blocking function to return urn next event.
pub fn poll_event() -> Event {
    unsafe {
        let ev = nil_raw_event();
        let rc = ffi::tb_poll_event(&ev);
        unpack_event(rc, &ev)
    }
}

/// Convenience functions
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

/// Convert from enums to u16
fn convert_color(c: Color) -> u16 {
    let ret = match c {
        Color::Default => tb_color::TB_DEFAULT,
        Color::Black => tb_color::TB_BLACK,
        Color::Red => tb_color::TB_RED,
        Color::Green => tb_color::TB_GREEN,
        Color::Yellow => tb_color::TB_YELLOW,
        Color::Blue => tb_color::TB_BLUE,
        Color::Magenta => tb_color::TB_MAGENTA,
        Color::Cyan => tb_color::TB_CYAN,
        Color::White => tb_color::TB_WHITE,
    };
    ret as u16
}

fn convert_style(style: Style) -> u16 {
    match style {
        Style::Normal => tb_attribute::empty(),
        Style::Bold => ffi::TB_BOLD,
        Style::Underline => ffi::TB_UNDERLINE,
        Style::BoldUnderline => ffi::TB_BOLD | ffi::TB_UNDERLINE,
        Style::Reverse => ffi::TB_UNDERLINE,
        Style::BoldReverse => ffi::TB_BOLD | ffi::TB_REVERSE,
        Style::UnderlineReverse => ffi::TB_UNDERLINE | ffi::TB_REVERSE,
        Style::BoldUnderlineReverse => ffi::TB_BOLD | ffi::TB_UNDERLINE |
            ffi::TB_REVERSE,
    }.bits()
}

fn reverse_convert_key(k: u16) -> Option<Key> {
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

fn nil_raw_event() -> ffi::tb_event {
    ffi::tb_event{etype: 0, emod: 0, key: 0, ch: 0, w: 0, h: 0}
}

fn unpack_event(ev_type: c_int, ev: &ffi::tb_event) -> Event {
    match FromPrimitive::from_i32(ev_type).unwrap() {
        tb_event_type::TB_EVENT_NONE => Event::NoEvent,
        tb_event_type::TB_EVENT_KEY => Event::KeyEvent(ev.emod, reverse_convert_key(ev.key), char::from_u32(ev.ch)),
        tb_event_type::TB_EVENT_RESIZE => Event::ResizeEvent(ev.w, ev.h),
        tb_event_type::TB_EVENT_ERROR => Event::NoEvent
    }
}
