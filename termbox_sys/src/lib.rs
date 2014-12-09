#![allow(non_camel_case_types)]

extern crate libc;

use libc::c_int;

pub const TB_HIDE_CURSOR: c_int = -1;
pub const TB_MOD_ALT: c_int = 1;

pub enum tb_color {
    TB_DEFAULT,
    TB_BLACK,
    TB_RED,
    TB_GREEN,
    TB_YELLOW,
    TB_BLUE,
    TB_MAGENTA,
    TB_CYAN,
    TB_WHITE,
}

bitflags! {
    flags tb_attribute: u16 {
        const TB_BOLD = 0x0100,
        const TB_UNDERLINE = 0x0200,
        const TB_REVERSE = 0x0400,
    }
}

#[deriving(FromPrimitive)]
pub enum tb_event_type {
    TB_EVENT_ERROR = -1,
    TB_EVENT_NONE = 0,
    TB_EVENT_KEY = 1,
    TB_EVENT_RESIZE = 2,
}

pub enum tb_error {
    TB_EUNSUPPORTED_TERMINAL = -1,
    TB_EFAILED_TO_OPEN_TTY = -2,
    TB_EPIPE_TRAP_ERROR = -3,
}

pub enum tb_input_mode {
    TB_INPUT_CURRENT = 0,
    TB_INPUT_ESC = 1,
    TB_INPUT_ALT = 2,
}

pub enum tb_output_mode {
    TB_OUTPUT_CURRENT = 0,
    TB_OUTPUT_NORMAL = 1,
    TB_OUTPUT_256 = 2,
    TB_OUTPUT_216 = 3,
    TB_OUTPUT_GRAYSCALE = 4,
}

/*
 * The event type matches struct tb_event from termbox.h
 */
#[repr(C)]
pub struct tb_event {
    pub etype: u8,
    pub emod: u8,
    pub key: u16,
    pub ch: u32,
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
pub struct tb_cell {
    pub ch: u32,
    pub fg: u16,
    pub bg: u16,
}

extern {
    pub fn tb_init() -> c_int;
    pub fn tb_shutdown();

    pub fn tb_width() -> c_int;
    pub fn tb_height() -> c_int;

    pub fn tb_clear();
    pub fn tb_set_clear_attributes(fg: u16, bg: u16);

    pub fn tb_present();

    pub fn tb_set_cursor(cx: c_int, cy: c_int);

    pub fn tb_put_cell(x: c_int, y: c_int, cell: *const tb_cell);
    pub fn tb_change_cell(x: c_int, y: c_int, ch: u32, fg: u16, bg: u16);

    pub fn tb_cell_buffer() -> *mut tb_cell;

    pub fn tb_select_input_mode(mode: c_int) -> c_int;
    pub fn tb_select_output_mode(mode: c_int) -> c_int;

    pub fn tb_peek_event(ev: *const tb_event, timeout: c_int) -> c_int;

    pub fn tb_poll_event(ev: *const tb_event) -> c_int;
}
